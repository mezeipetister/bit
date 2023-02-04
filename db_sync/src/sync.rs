use std::{
  collections::HashMap,
  default,
  fmt::{Debug, Display},
  ops::{Deref, DerefMut},
  path::PathBuf,
  sync::{Arc, Mutex, MutexGuard},
};

use chrono::{DateTime, Utc};
use futures_util::stream;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tonic::{transport::Server, Request};
use uuid::Uuid;

use crate::{
  fs::{
    binary_continuous_append, binary_continuous_read,
    binary_continuous_read_after_filter, binary_init, binary_init_empty,
    binary_read, binary_update,
  },
  prelude::{path_helper, sha1_signature},
  server::sync_api::{
    api_client::ApiClient,
    api_server::{Api, ApiServer},
    CommitObj, PullRequest,
  },
};

/// Action trait for Actionable types
/// Implemented types can be used as storage patch objects.
pub trait ActionExt: Clone + Send {
  /// Human readable display msg
  /// This can be used in UI to display
  /// Patch actions
  fn display(&self) -> String;
}

/// Generic acion representation
/// Atomic action kinds with the following states:
/// Create, Patch, Remove, Recover
#[derive(Serialize, Deserialize, Clone, Debug)]
enum ActionKind<A>
where
  A: ActionExt,
{
  /// Create a new empty object
  Create,
  /// Patch object with action A
  Patch(A),
}

/// ActionObject must be produced by a StorageObject
/// By providing a &Commit and an A: impl ActionExt to it.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ActionObject<A>
where
  A: ActionExt,
{
  // Unique ID
  id: Uuid,
  // Referred Storage ID
  // Object must be located under it
  storage_id: String,
  // Referred ObjectId
  // must be applied on it
  object_id: Uuid,
  // UserID
  uid: String,
  // Applied date and time in Utc
  dtime: DateTime<Utc>,
  // Related commit id
  commit_id: Option<Uuid>,
  // Object actions parent action id
  // We can use this attribute to check action chain per storage object
  parent_action_id: Option<Uuid>,
  // Create(T) or Patch(A)
  action: ActionKind<A>,
  // Remote signature
  remote_signature: Option<String>,
}

impl<A> ActionObject<A>
where
  A: ActionExt + Serialize,
{
  // Check if patch
  fn is_kind_patch(&self) -> bool {
    if let ActionKind::Patch(_) = self.action {
      return true;
    }
    false
  }
  // Check if create
  fn is_kind_create(&self) -> bool {
    if let ActionKind::Create = self.action {
      return true;
    }
    false
  }
  fn is_local(&self) -> bool {
    self.remote_signature.is_none()
  }
  fn is_remote(&self) -> bool {
    !self.is_local()
  }
  fn is_staging(&self) -> bool {
    self.commit_id.is_none()
  }
  fn be_commited(&mut self, commit_id: Uuid) {
    if self.is_staging() {
      self.commit_id = Some(commit_id);
    }
  }
  fn has_valid_signature(&self) -> bool {
    let mut s = self.to_owned();
    s.remote_signature = None;
    let sig = sha1_signature(self).unwrap();
    match &self.remote_signature {
      Some(s) => s == sig.as_str(),
      None => false,
    }
  }
  fn add_signature(&mut self, sig: String) -> Result<(), String> {
    if self.is_remote() {
      return Err("Signature can added only for local AOB".to_string());
    }
    self.remote_signature = Some(sig);
    Ok(())
  }
  fn sign(&mut self) {
    // Assert we are in server mode
    // No client can sign object
    assert_eq!(
      std::env::var("SYNC_MODE").expect("No SYNC_MODE env var set"),
      "SERVER"
    );
    // Change remote sign. to None
    self.remote_signature = None;
    // Create sha1 signature
    let sig = sha1_signature(self).unwrap();
    // Place new signature
    self.remote_signature = Some(sig);
  }
  // Reser parent action id
  // Only client can do it
  fn reset_parent_action_id(&mut self, parent_action_id: Option<Uuid>) {
    // Only NOT SERVER
    // can perform this action
    assert_ne!(
      std::env::var("SYNC_MODE").expect("No SYNC_MODE env var set"),
      "SERVER"
    );
    // Set parent id
    self.parent_action_id = parent_action_id;
  }
  // Reset dtime
  // Should apply only when remote update occurs
  fn reset_dtime(&mut self) {
    // Only NOT SERVER
    // can perform this action
    assert_ne!(
      std::env::var("SYNC_MODE").expect("No SYNC_MODE env var set"),
      "SERVER"
    );
    self.dtime = Utc::now();
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Status {
  Ok,
  Conflict,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Document<A>
where
  A: ActionExt,
{
  // Storage Object unique ID
  id: Uuid,
  // StorageId
  storage_id: String,
  // Actions
  actions: Vec<ActionObject<A>>,
  // Status
  status: Status,
}

impl<A> Document<A>
where
  A: ActionExt + Serialize + for<'de> Deserialize<'de> + Debug,
{
  /// Create ActionObject from Action
  /// and add it to the given Document
  pub fn patch(&mut self, ctx: &Context, action: A) -> Result<(), String> {
    let aob = self.create_aob(ctx, ActionKind::Patch(action))?;
    self.add_aob(aob);
    Ok(())
  }

  // Find AOB place in actions
  // and insert it
  fn add_aob(&mut self, aob: ActionObject<A>) -> Result<(), String> {
    // find parent position
    let position = self.actions.iter().position(|_aob| _aob.id == aob.id);
    match position {
      Some(index) => {
        self.actions.insert(index + 1, aob);
        self.check_status();
        Ok(())
      }
      None => Err("Cannot insert aob; parent aob not found".to_string()),
    }
  }

  // Check wether StorageObject is only local
  // True if no remote object
  fn is_local_object(&self) -> bool {
    self.actions.first().unwrap().remote_signature.is_none()
  }
  // Check wether StorageObject is remote
  // True if Some remote object
  fn is_remote_object(&self) -> bool {
    !self.is_local_object()
  }
  // Clear all local action objects
  // If object is local (no remote actions and object state)
  // we should not be here. That object should be removed without
  // clearing it.
  pub fn clear_local_aobs(&mut self) -> Result<(), String> {
    // Check if remote
    assert!(
      self.is_remote_object(),
      "Only remote StorageObject can be cleared locally"
    );
    // Clear all local actions
    self.actions.retain(|aob| aob.has_valid_signature());
    Ok(())
  }
  pub fn clear_staging(&mut self) -> Result<(), String> {
    // Clear all local actions
    self.actions.retain(|aob| aob.commit_id.is_some());
    Ok(())
  }
  // Create action object by providing a Context, Commit and Action object.
  // If Patch returns error, we return it back to the caller
  fn create_aob(
    &self,
    ctx: &Context,
    action: ActionKind<A>,
  ) -> Result<ActionObject<A>, String> {
    let dtime = Utc::now();
    let res = ActionObject {
      id: Uuid::new_v4(),
      storage_id: self.storage_id.clone(),
      object_id: self.id.clone(),
      uid: ctx.uid.to_owned(),
      dtime,
      commit_id: None,
      parent_action_id: self.actions.last().map(|i| i.id),
      action,
      remote_signature: None,
    };
    Ok(res)
  }
  // Init storage object from FS
  fn read_from_fs(
    ctx: &Context,
    storage_id: &str,
    object_id: Uuid,
  ) -> Result<Self, String> {
    binary_read(path_helper::storage_object_path(ctx, storage_id, object_id))
  }
  // Update storage object file
  fn save_to_fs(&self, ctx: &Context) -> Result<(), String> {
    let object_path =
      path_helper::storage_object_path(ctx, &self.storage_id, self.id);
    binary_update(object_path, &self)
  }
  fn check_has_staging_aob(&self) -> bool {
    match self.actions.last() {
      Some(aob) => true,
      None => false,
    }
  }
  fn close_staging(&mut self, commit_id: Uuid) {
    if self.check_has_staging_aob() {
      for aob in &mut self.actions {
        if aob.is_staging() {
          aob.be_commited(commit_id);
        }
      }
    }
  }
  // Perform status check
  fn check_status(&mut self) {
    // Check action object parent chain
    let mut status = Status::Ok;
    let previous_aob_id = None;
    for aob in &self.actions {
      if aob.parent_action_id != previous_aob_id {
        status = Status::Conflict;
      }
    }
    // Set new status as result
    self.status = status;
  }
  // Check if status is Conflict
  fn has_conflict(&self) -> bool {
    match self.status {
      Status::Conflict => true,
      _ => false,
    }
  }
}

#[derive(Clone)]
pub struct Context {
  pub db_root_path: PathBuf,
  pub uid: String,
}

impl Context {
  pub fn init(db_root_path: PathBuf, uid: String) -> Self {
    Self { db_root_path, uid }
  }
}

#[derive(Default, Serialize, Deserialize, Debug)]
struct CommitIndex {
  latest_local_commit_id: Option<Uuid>,
  latest_remote_commit_id: Option<Uuid>,
}

impl CommitIndex {
  fn init(ctx: &Context) {
    binary_init(path_helper::commit_index(ctx), Self::default());
  }
  fn load(ctx: &Context) -> Self {
    binary_read(path_helper::commit_index(&ctx))
      .expect("Error reading commit index")
  }
  fn save_fs(&self, ctx: &Context) -> Result<(), String> {
    binary_update(path_helper::commit_index(ctx), &self)
  }
  fn latest_local_commit_id(ctx: &Context) -> Option<Uuid> {
    let s = Self::load(ctx);
    s.latest_local_commit_id
  }
  fn latest_remote_commit_id(ctx: &Context) -> Option<Uuid> {
    let s = Self::load(ctx);
    s.latest_local_commit_id
  }
  fn set_latest_local_id(
    ctx: &Context,
    latest_local: Option<Uuid>,
  ) -> Result<(), String> {
    let mut s = Self::load(ctx);
    s.latest_local_commit_id = latest_local;
    s.save_fs(ctx)
  }
  fn set_latest_remote_id(
    ctx: &Context,
    latest_remote: Option<Uuid>,
  ) -> Result<(), String> {
    let mut s = Self::load(ctx);
    s.latest_remote_commit_id = latest_remote;
    s.save_fs(ctx)
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Commit {
  id: Uuid,
  uid: String,
  dtime: DateTime<Utc>,
  comment: String,
  ancestor_id: Uuid,
  serialized_actions: Vec<String>, // ActionObject JSONs in Vec
  remote_signature: Option<String>, // Remote signature
}

impl Commit {
  fn new(uid: String, comment: String) -> Self {
    Self {
      id: Uuid::new_v4(),
      uid,
      dtime: Utc::now(),
      comment,
      ancestor_id: Uuid::default(),
      serialized_actions: vec![],
      remote_signature: None,
    }
  }
  fn add_action_object<A>(&mut self, aob: ActionObject<A>)
  where
    A: ActionExt + Serialize,
  {
    self
      .serialized_actions
      .push(serde_json::to_string(&aob).unwrap());
  }
  fn set_dtime(&mut self) {
    self.dtime = Utc::now()
  }
  fn set_ancestor_id(&mut self, ancestor_id: Uuid) {
    self.ancestor_id = ancestor_id;
  }
  fn is_remote(&self) -> bool {
    self.remote_signature.is_some()
  }
  fn is_local(&self) -> bool {
    !self.is_remote()
  }
  fn add_remote_signature(&mut self) -> Result<(), String> {
    if self.is_remote() {
      return Err("Commit already has remote signature!".into());
    }
    let signature = sha1_signature(&self)?;
    self.remote_signature = Some(signature);
    Ok(())
  }
  fn has_valid_remote_signature(&self) -> Result<bool, String> {
    let mut copied = self.clone();
    let sig1 = copied.remote_signature.take();
    let sig2 = sha1_signature(&self)?;
    if let Some(sig1) = sig1 {
      if sig1 == sig2 {
        return Ok(true);
      }
    }
    Ok(false)
  }
}

/// Commit Log
/// contains all the repository related logs
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct CommitLog;

impl CommitLog {
  fn init(ctx: &Context) -> Result<(), String> {
    // Init latest log
    // binary_init::<HashMap<String, Uuid>>(
    //   path_helper::commit_latest(ctx),
    //   HashMap::default(),
    // )?;
    // Init local log
    binary_init_empty(path_helper::commit_local_log(ctx))?;
    // Init remote log
    binary_init_empty(path_helper::commit_remote_log(ctx))?;
    // Init commit index
    CommitIndex::init(ctx);
    Ok(())
  }

  fn load_locals(ctx: &Context) -> Result<Vec<Commit>, String> {
    let locals = binary_continuous_read(path_helper::commit_local_log(ctx))?;
    Ok(locals)
  }
  fn load_remotes(ctx: &Context) -> Result<Vec<Commit>, String> {
    let remotes = binary_continuous_read(path_helper::commit_remote_log(ctx))?;
    Ok(remotes)
  }
  fn load_remotes_after(
    ctx: &Context,
    after_id: Uuid,
  ) -> Result<Vec<Commit>, String> {
    let remotes = binary_continuous_read_after_filter(
      path_helper::commit_remote_log(ctx),
      |i: &Commit| i.id == after_id,
    )?;
    Ok(remotes)
  }
  fn add_local_commit(
    ctx: &Context,
    mut local_commit: Commit,
  ) -> Result<(), String> {
    // Set ancestor ID
    if let Some(last_local_commit_id) = CommitIndex::latest_local_commit_id(ctx)
    {
      local_commit.set_ancestor_id(last_local_commit_id);
    }
    // Set commit index
    CommitIndex::set_latest_local_id(ctx, Some(local_commit.id))?;
    // Save local commit
    binary_continuous_append(path_helper::commit_local_log(ctx), local_commit)
  }
  fn add_remote_commit(
    ctx: &Context,
    remote_commit: Commit,
  ) -> Result<(), String> {
    let mut commit_index = CommitIndex::load(ctx);
    // check ancestor ID
    if let Some(last_remote_commit_id) = commit_index.latest_remote_commit_id {
      if remote_commit.ancestor_id != last_remote_commit_id {
        return Err("Remote commit ancestor ID error! Please pull".into());
      }
    }
    // Set commit index
    CommitIndex::set_latest_remote_id(ctx, Some(remote_commit.id))?;
    // Save remote commit
    binary_continuous_append(path_helper::commit_remote_log(ctx), remote_commit)
  }
}

// Repository Mode
// Local, Remote or Server
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub enum Mode {
  Server {
    server_addr: String,
  },
  Remote {
    remote_url: String,
  },
  #[default]
  Local,
}

impl Display for Mode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Mode::Server { .. } => write!(f, "{}", "SERVER"),
      Mode::Remote { .. } => write!(f, "{}", "REMOTE"),
      Mode::Local => write!(f, "{}", "LOCAL"),
    }
  }
}

impl Mode {
  pub fn server(server_addr: String) -> Self {
    Self::Server { server_addr }
  }
  pub fn remote(remote_url: String) -> Self {
    Self::Remote { remote_url }
  }
  pub fn local() -> Self {
    Self::Local
  }
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct RepoDetails {
  mode: Mode,
  storage_ids: HashMap<String, Vec<Uuid>>,
}

impl RepoDetails {
  fn init(ctx: &Context, mode: Mode) -> Result<(), String> {
    binary_init(
      path_helper::repo_details(ctx),
      RepoDetails {
        mode,
        ..Self::default()
      },
    )?;
    Ok(())
  }
  fn load(ctx: &Context) -> Result<Self, String> {
    let repo_details: RepoDetails =
      binary_read(path_helper::repo_details(ctx))?;
    // Set ENV var
    std::env::set_var("SYNC_MODE", format!("{}", repo_details.mode));
    // Return repo details
    Ok(repo_details)
  }
}

pub struct Repository {
  inner: Arc<Mutex<RepoInner>>,
}

impl Repository {
  pub fn inner(&self) -> MutexGuard<RepoInner> {
    self.inner.lock().unwrap()
  }
}

pub struct RepoInner {
  ctx: Context,
  repo_details: RepoDetails,
}

impl RepoInner {
  fn ctx(&self) -> &Context {
    &self.ctx
  }
}

impl Repository {
  /// Load repository
  pub fn load(ctx: Context) -> Result<Self, String> {
    // Load repo details
    let repo_details = RepoDetails::load(&ctx)?;
    // Create res
    let inner = RepoInner { ctx, repo_details };
    Ok(Self {
      inner: Arc::new(Mutex::new(inner)),
    })
  }
  /// Init repository
  pub fn init(ctx: Context, mode: Mode) -> Result<Self, String> {
    // Check if repository inited
    if Self::load(ctx.clone()).is_ok() {
      return Err("Existing repository. Cannot init a new one".into());
    }
    // Init commit log
    CommitLog::init(&ctx)?;
    // Init repo details
    RepoDetails::init(&ctx, mode)?;
    // Load repo details
    let repo_details = RepoDetails::load(&ctx)?;
    // Create res
    let inner = RepoInner { ctx, repo_details };
    Ok(Self {
      inner: Arc::new(Mutex::new(inner)),
    })
  }
  // Clone remote repository to local
  fn clone(remote_url: &str) -> Result<Self, String> {
    // TODO! Fix path and UID
    let ctx = Context::init(PathBuf::from("./data"), "mezeipetister".into());
    // Check if repository inited
    if Self::load(ctx.clone()).is_ok() {
      return Err("Existing repository. Cannot clone again".into());
    }

    unimplemented!()
  }
  /// Start remote server
  pub fn serve(&self) -> Result<(), String> {
    let _self = Repository {
      inner: Arc::clone(&self.inner),
    };
    let server_addr = match &_self.inner().repo_details.mode {
      Mode::Server { server_addr } => server_addr.to_string(),
      _ => {
        panic!("Cannot start server, as the repository is not in server mode")
      }
    };
    let runtime = tokio::runtime::Builder::new_current_thread()
      .enable_all()
      .worker_threads(1)
      .thread_name("sync_server")
      .build()
      .unwrap();
    runtime.block_on(async {
      Server::builder()
        .add_service(ApiServer::new(_self))
        .serve(server_addr.parse().unwrap())
        .await
        .expect("Error starting server");
    });
    Ok(())
  }

  /// Pull remote repository
  pub fn proceed_pull(&self) -> Result<(), String> {
    let inner = self.inner();
    let remote_addr = match &inner.repo_details.mode {
      Mode::Remote { remote_url } => remote_url.to_string(),
      _ => {
        panic!("Cannot proceed pull operation, as the repository is not in remote mode")
      }
    };

    let runtime = tokio::runtime::Builder::new_current_thread()
      .enable_all()
      .worker_threads(1)
      .thread_name("sync_server")
      .build()
      .unwrap();

    let ctx = inner.ctx();

    // Get last local remote commit id
    let after_commit_id = CommitIndex::latest_remote_commit_id(&ctx)
      .map(|i| i.to_string())
      .unwrap_or("".to_string());

    runtime.block_on(async {
      let mut remote_client = ApiClient::connect(remote_addr)
        .await
        .expect("Could not connect to UPL service");

      let mut res = remote_client
        .pull(PullRequest { after_commit_id })
        .await
        .unwrap()
        .into_inner();

      let mut commits = vec![];

      while let Some(commit) = res.message().await.unwrap() {
        commits.push(commit);
      }

      for commit_obj in commits {
        let commit: Commit = serde_json::from_str(&commit_obj.obj_json_string)
          .expect("Commit deser error");
        // let ctx = self.merge_commit_ctx(commit);
        drop(ctx)
      }
    });

    Ok(())
  }
  /// Push repository local commits to remote
  pub fn proceed_push(&self) -> Result<(), String> {
    // Before push operation
    // Proceed pull
    self.proceed_pull()?;

    let inner = self.inner();

    let remote_addr = match &inner.repo_details.mode {
      Mode::Remote { remote_url } => remote_url.to_string(),
      _ => {
        panic!("Cannot proceed push operation, as the repository is not in remote mode")
      }
    };

    let runtime = tokio::runtime::Builder::new_current_thread()
      .enable_all()
      .worker_threads(1)
      .thread_name("sync_server")
      .build()
      .unwrap();

    runtime.block_on(async {
      let mut remote_client = ApiClient::connect(remote_addr)
        .await
        .expect("Could not connect to UPL service");

      let local_commits = self
        .local_commits()
        .unwrap()
        .into_iter()
        .map(|c| CommitObj {
          obj_json_string: serde_json::to_string(&c).unwrap(),
        })
        .collect::<Vec<CommitObj>>();

      let mut commits = vec![];

      for commit in local_commits {
        info!("Sending commit obj");
        let mut commit = remote_client.push(commit).await.unwrap().into_inner();
        info!("Commit received back");
        commits.push(commit);
      }

      info!("Pushed {} items", commits.len());
    });

    // After push operation
    // Proceed pull to update local storages
    self.proceed_pull()?;

    Ok(())
  }
  /// Clean local repository, clear local changes
  /// And performs remote pull
  pub fn proceed_clean(&self) -> Result<(), String> {
    unimplemented!()
  }
  pub fn local_commits(&self) -> Result<Vec<Commit>, String> {
    let inner = self.inner();
    CommitLog::load_locals(inner.ctx())
  }
  pub fn remote_commits(&self) -> Result<Vec<Commit>, String> {
    let inner = self.inner();
    CommitLog::load_remotes(inner.ctx())
  }
  pub fn remote_commits_after(
    &self,
    after_id: Uuid,
  ) -> Result<Vec<Commit>, String> {
    let inner = self.inner();
    CommitLog::load_remotes_after(inner.ctx(), after_id)
  }
}
