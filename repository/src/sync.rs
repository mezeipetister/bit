use chrono::{DateTime, Utc};
use futures_util::stream;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::{
  collections::{HashMap, HashSet},
  fmt::{Debug, Display},
  hash::Hash,
  marker::PhantomData,
  ops::{Deref, DerefMut},
  path::PathBuf,
  sync::{Arc, Mutex, MutexGuard},
};
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
pub trait ActionExt: Clone + Send + Sync {
  /// Human readable display msg
  /// This can be used in UI to display
  /// Patch actions
  fn display(&self) -> String;
}

/// Generic acion representation
/// Atomic action kinds with the following states:
/// Create, Patch, Remove, Recover
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ActionKind<A>
where
  A: ActionExt,
{
  /// Create a new empty object
  Create(A),
  /// Patch object with action A
  Patch(A),
}

impl<A: ActionExt> Display for ActionKind<A> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ActionKind::Create(a) => write!(f, "CREATE: {}", a.display()),
      ActionKind::Patch(a) => write!(f, "PATCH: {}", a.display()),
    }
  }
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
  pub storage_id: String,
  // Referred ObjectId
  // must be applied on it
  pub object_id: Uuid,
  // UserID
  pub uid: String,
  // Applied date and time in Utc
  pub dtime: DateTime<Utc>,
  // Related commit id
  pub commit_id: Option<Uuid>,
  // Object actions parent action id
  // We can use this attribute to check action chain per storage object
  pub parent_action_id: Option<Uuid>,
  // Create(T) or Patch(A)
  pub action: ActionKind<A>,
  // Remote signature
  pub remote_signature: Option<String>,
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
    if let ActionKind::Create(_) = self.action {
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
  // Add aob to local commit
  fn be_commited(&mut self, commit: &mut Commit) {
    if self.is_staging() {
      self.commit_id = Some(commit.id);
      commit.add_action_object(self);
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
  fn remote_sign(&mut self) {
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
pub enum Status {
  Ok,
  Conflict,
}

pub trait ActionPatch<A: ActionExt>: Default {
  const storage_id: &'static str;
  fn patch(&mut self, action: A, dtime: DateTime<Utc>, uid: &str);
}

// Data layer to sync with storage documents
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DocRef<T, A>
where
  A: ActionExt,
  T: ActionPatch<A> + Default,
{
  pub object_id: Uuid,
  storage_id: String,
  data: T,
  last_aob_id: Uuid,
  action: PhantomData<A>,
}

impl<T, A> DocRef<T, A>
where
  A: ActionExt + Serialize + for<'de> Deserialize<'de> + Debug,
  T: ActionPatch<A> + Default,
{
  pub fn init_from_aob(aob: &ActionObject<A>) -> Self {
    Self {
      object_id: aob.object_id,
      storage_id: aob.storage_id.to_owned(),
      data: T::default(),
      last_aob_id: aob.id,
      action: PhantomData,
    }
  }
  pub fn create_aob(
    &self,
    repository: &Repository,
    patch: A,
  ) -> Result<ActionObject<A>, String> {
    let doc: Document<A> = repository.get_doc(self.object_id)?;
    let ctx = repository.ctx();
    let res = doc.create_aob(ctx, ActionKind::Patch(patch))?;
    Ok(res)
  }
}

impl<T, A> Deref for DocRef<T, A>
where
  A: ActionExt,
  T: ActionPatch<A>,
{
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.data
  }
}

impl<T, A> DerefMut for DocRef<T, A>
where
  A: ActionExt,
  T: ActionPatch<A>,
{
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.data
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DocRefVec<T, A>
where
  A: ActionExt,
  T: ActionPatch<A>,
{
  doc_refs: Vec<DocRef<T, A>>,
  action: PhantomData<A>,
}

impl<T, A> DocRefVec<T, A>
where
  A: ActionExt + Serialize + for<'de> Deserialize<'de> + Debug,
  T: ActionPatch<A> + Debug,
{
  /// Reset all docs
  pub fn reset(&mut self) {
    self.doc_refs = vec![];
  }
  /// Try to get object ref by object_id
  pub fn get(&self, object_id: Uuid) -> Result<&DocRef<T, A>, String> {
    let res = self
      .doc_refs
      .iter()
      .find(|d| d.object_id == object_id)
      .ok_or("Cannot find docref by objectid".to_string())?;
    Ok(res)
  }
  /// Try to get mutable object ref by object id
  pub fn get_mut(
    &mut self,
    object_id: Uuid,
  ) -> Result<&mut DocRef<T, A>, String> {
    let res = self
      .doc_refs
      .iter_mut()
      .find(|d| d.object_id == object_id)
      .ok_or("Cannot find docref by objectid".to_string())?;
    Ok(res)
  }
  /// Sync docref by a document
  pub fn sync_with_doc(&mut self, doc: &Document<A>) -> Result<(), String> {
    // Check if index obj exist
    if self.get(doc.id).is_err() {
      // Create index obj if has not existed yet
      self.push(DocRef::init_from_aob(doc.actions.first().unwrap()));
    }
    // First find index obj
    let mut r = self.get_mut(doc.id)?;
    let last_index_aob_id = r.last_aob_id;
    let pos = doc
      .actions
      .iter()
      .position(|aob| aob.id == last_index_aob_id)
      .unwrap_or(0);
    let aobs_to_update = match pos == 0 {
      true => &doc.actions[0..],
      false => &doc.actions[pos + 1..],
    };
    for i in aobs_to_update {
      if let ActionKind::Create(p) = &i.action {
        r.patch(p.to_owned(), i.dtime, &i.uid);
      }
      if let ActionKind::Patch(p) = &i.action {
        r.patch(p.to_owned(), i.dtime, &i.uid);
      }
      r.last_aob_id = i.id;
    }
    Ok(())
  }
  pub fn create_init_aob(
    &self,
    init_action: A,
    uid: String,
  ) -> ActionObject<A> {
    ActionObject {
      id: Uuid::new_v4(),
      storage_id: T::storage_id.to_string(),
      object_id: Uuid::new_v4(),
      uid,
      dtime: Utc::now(),
      commit_id: None,
      parent_action_id: None,
      action: ActionKind::Create(init_action),
      remote_signature: None,
    }
  }
}

impl<T, A> Deref for DocRefVec<T, A>
where
  A: ActionExt,
  T: ActionPatch<A>,
{
  type Target = Vec<DocRef<T, A>>;

  fn deref(&self) -> &Self::Target {
    &self.doc_refs
  }
}

impl<T, A> DerefMut for DocRefVec<T, A>
where
  A: ActionExt,
  T: ActionPatch<A>,
{
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.doc_refs
  }
}

impl<T, A> Default for DocRefVec<T, A>
where
  A: ActionExt,
  T: ActionPatch<A>,
{
  fn default() -> Self {
    Self {
      doc_refs: Default::default(),
      action: PhantomData,
    }
  }
}

pub trait IndexExt {
  type ActionType: ActionExt;
  fn reset_docrefs(&mut self) -> Result<(), String>;
  fn sync_doc(
    &mut self,
    doc: &Document<Self::ActionType>,
  ) -> Result<(), String>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Document<A>
where
  A: ActionExt,
{
  // Storage Object unique ID
  pub id: Uuid,
  // StorageId
  pub storage_id: String,
  // Actions
  pub actions: Vec<ActionObject<A>>,
  // Status
  pub status: Status,
}

impl<A> Document<A>
where
  A: ActionExt + Serialize + for<'de> Deserialize<'de> + Debug,
{
  pub fn print(&self) -> String {
    let mut res = String::new();
    writeln!(res, "ID: {id}", id = self.id);
    writeln!(
      res,
      "Storage ID: {storage_id}",
      storage_id = self.storage_id
    );
    writeln!(res, "Status: {:?}\n", self.status);
    writeln!(res, "History:\n----------------------\n");
    self.actions.iter().for_each(|action| {
      writeln!(
        res,
        "[{local}] {msg}\n    {dtime} - {uid}\n",
        local = match action.is_staging() {
          true => "S",
          false => match action.is_local() {
            true => "L",
            false => "R",
          },
        },
        dtime = action.dtime.naive_local().to_string(),
        uid = action.uid,
        msg = action.action
      );
    });
    res
  }
  // Return staging aobs
  // using to collect all repo staging aobs
  fn get_staging_aobs(&self) -> Vec<&ActionObject<A>> {
    self
      .actions
      .iter()
      .filter(|aob| aob.is_staging())
      .collect::<Vec<&ActionObject<A>>>()
  }
  // Return staging aobs
  // using to collect all repo staging aobs
  pub fn commit(&mut self, commit: &mut Commit) {
    self
      .actions
      .iter_mut()
      .filter(|aob| aob.is_staging())
      .for_each(|aob| {
        aob.be_commited(commit);
      });
  }
  // Find AOB place in actions
  // and insert it
  fn add_aob(mut self, new_aob: ActionObject<A>) -> Result<Self, String> {
    // 1) Update itself
    if let Some(aob) = &mut self.actions.iter().find(|aob| aob.id == new_aob.id)
    {
      // If aob is in the acionts;
      // update it
      // TODO! We should check to replace only width remote AOB
      std::mem::swap(aob, &mut &new_aob);
      return Ok(self);
    }
    // 2) Insert after position
    if let Some(position) = self.actions.iter().position(|aob| {
      if let Some(parent_action_id) = new_aob.parent_action_id {
        aob.id == parent_action_id
      } else {
        false
      }
    }) {
      self.actions.insert(position + 1, new_aob);
      self.check_status();
      return Ok(self);
    }

    return Err("Cannot insert aob; parent aob not found".to_string());

    // match new_aob.is_local() {
    //   true => self.actions.push(new_aob),
    //   false => {
    //     // Remove itself as local aob
    //     self.actions.retain(|aob| aob.id != new_aob.id);
    //     // find parent position
    //     let position = self.actions.iter().position(|aob| {
    //       if let Some(parent_action_id) = new_aob.parent_action_id {
    //         aob.id == parent_action_id
    //       } else {
    //         false
    //       }
    //     });
    //     match position {
    //       Some(index) => {
    //         self.actions.insert(index + 1, new_aob);
    //         self.check_status();
    //       }
    //       None => {
    //         return Err("Cannot insert aob; parent aob not found".to_string())
    //       }
    //     }
    //   }
    // }
    Ok(self)
  }

  // Append aob after parent check
  fn add_aob_server_side(
    mut self,
    new_aob: ActionObject<A>,
  ) -> Result<Self, String> {
    match new_aob.is_local() {
      true => {
        return Err(
          "Only remote aob can be append as remote procedure".to_string(),
        )
      }
      false => {
        let last_aob_id = self.actions.last().map(|aob| aob.id);
        if new_aob.parent_action_id == last_aob_id {
          self.actions.push(new_aob);
          // self.check_status();
        } else {
          return Err("Cannot insert aob; parent aob not found".to_string());
        }
      }
    }
    Ok(self)
  }

  // Check wether StorageObject is only local
  // True if no remote document
  fn is_local_document(&self) -> bool {
    self.actions.first().unwrap().remote_signature.is_none()
  }
  // Check wether StorageObject is remote
  // True if Some remote document
  fn is_remote_document(&self) -> bool {
    !self.is_local_document()
  }
  // Clear all local action objects
  // If object is local (no remote actions and object state)
  // we should not be here. That object should be removed without
  // clearing it.
  pub fn clear_local_aobs(&mut self) -> Result<(), String> {
    // Check if remote
    if self.is_local_document() {
      return Err(
        "Only remote StorageObject can be cleared locally".to_string(),
      );
    }
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
  fn read_from_fs(ctx: &Context, object_id: Uuid) -> Result<Self, String> {
    binary_read(path_helper::storage_object_path(ctx, object_id))
  }
  // Update storage object file
  fn save_to_fs(&self, ctx: &Context) -> Result<(), String> {
    let object_path = path_helper::storage_object_path(ctx, self.id);
    binary_update(object_path, &self)
  }
  fn check_has_staging_aob(&self) -> bool {
    let mut res = false;
    self.actions.iter().for_each(|aob| {
      if aob.is_staging() {
        res = true;
      }
    });
    res
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

#[derive(Clone, Debug)]
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
    s.latest_remote_commit_id
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
  ancestor_id: Option<Uuid>,
  serialized_actions: Vec<String>, // ActionObject JSONs in Vec
  remote_signature: Option<String>, // Remote signature
}

impl Display for Commit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "id: {id}\nparent_id: {parent_id}\naction_count: {action_count}\nsignature: {signature}",id = &self.id, action_count = self.serialized_actions.len(),  parent_id = &self.ancestor_id.map(|id| id.as_simple().to_string()).unwrap_or("-".to_string()), signature=self.remote_signature.as_ref().map(|sig| sig.to_owned()).unwrap_or("-".to_string()))
  }
}

impl Commit {
  fn new(uid: String, comment: String) -> Self {
    Self {
      id: Uuid::new_v4(),
      uid,
      dtime: Utc::now(),
      comment,
      ancestor_id: None,
      serialized_actions: vec![],
      remote_signature: None,
    }
  }
  fn add_action_object<A>(&mut self, aob: &ActionObject<A>)
  where
    A: ActionExt + Serialize,
  {
    self
      .serialized_actions
      .push(serde_json::to_string(aob).unwrap());
  }
  fn set_dtime(&mut self) {
    self.dtime = Utc::now()
  }
  fn set_ancestor_id(&mut self, ancestor_id: Option<Uuid>) {
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
    let last_local_commit_id = CommitIndex::latest_local_commit_id(ctx);
    local_commit.set_ancestor_id(last_local_commit_id);

    // Set commit index
    CommitIndex::set_latest_local_id(ctx, Some(local_commit.id))?;
    // Save local commit
    binary_continuous_append(path_helper::commit_local_log(ctx), local_commit)
  }
  fn add_remote_commit(
    ctx: &Context,
    remote_commit: Commit,
  ) -> Result<(), String> {
    // check ancestor ID
    let last_remote_commit_id = CommitIndex::latest_remote_commit_id(ctx);
    if remote_commit.ancestor_id != last_remote_commit_id {
      return Err("Remote commit ancestor ID error! Please pull".into());
    }

    // Set commit index
    CommitIndex::set_latest_remote_id(ctx, Some(remote_commit.id))?;
    // Save remote commit
    binary_continuous_append(path_helper::commit_remote_log(ctx), remote_commit)
  }
  fn reset_local_commits(ctx: &Context) -> Result<(), String> {
    // Set local commit log as empty
    binary_init_empty(path_helper::commit_local_log(ctx))?;
    // Set latest local id
    CommitIndex::set_latest_local_id(ctx, None)?;
    Ok(())
  }
}

// Repository Mode
// Local, Remote or Server
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub enum Mode {
  Server {
    remote_address: String,
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
    Self::Server {
      remote_address: server_addr,
    }
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
  document_ids: HashSet<Uuid>,
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
  fn save_fs(&self, ctx: &Context) -> Result<(), String> {
    binary_update(path_helper::repo_details(ctx), self.to_owned())
  }
  fn document_get<
    A: ActionExt + Serialize + for<'de> Deserialize<'de> + Debug,
  >(
    &self,
    ctx: &Context,
    document_id: Uuid,
  ) -> Result<Document<A>, String> {
    if let Some(doc_id) = self.document_ids.get(&document_id) {
      return Document::read_from_fs(ctx, *doc_id);
    }
    Err("No document by id found".to_string())
  }
  fn document_create<
    A: ActionExt + Serialize + for<'de> Deserialize<'de> + Debug,
  >(
    &mut self,
    ctx: &Context,
    aob: ActionObject<A>,
  ) -> Result<Document<A>, String> {
    if self.document_get::<A>(ctx, aob.object_id).is_ok() {
      return Err("Document already exist".to_string());
    }
    match &aob.action {
      ActionKind::Create(init_action) => {
        let new_doc: Document<A> = Document {
          id: aob.object_id,
          storage_id: aob.storage_id.to_string(),
          actions: vec![aob],
          status: Status::Ok,
        };
        crate::fs::binary_init(
          crate::prelude::path_helper::storage_object_path(ctx, new_doc.id),
          new_doc.clone(),
        )?;
        // Insert doc id
        self.document_ids.insert(new_doc.id);
        // Save repo details
        self.save_fs(ctx)?;
        // Return new doc
        Ok(new_doc)
      }
      ActionKind::Patch(_) => {
        Err("Aob must be create action kind to create new document".to_string())
      }
    }
  }
  // Update aob signature after push
  // fn update_aob_signature<
  //   A: ActionExt + Serialize + for<'de> Deserialize<'de> + Debug,
  // >(
  //   &mut self,
  //   ctx: &Context,
  //   aob: ActionObject<A>,
  // ) -> Result<Document<A>, String> {
  //   if let Ok(doc) = self.document_get::<A>(ctx, aob.object_id) {
  //     for aob in &mut doc.actions {}
  //     return Err("Cannot find AOB to update signature".to_string());
  //   }
  //   return Err("Unknown document".to_string());
  // }
}

#[derive(Debug)]
pub struct Repository {
  ctx: Context,
  repo_details: RepoDetails,
}

impl Repository {
  pub fn get_server_address(&self) -> Option<&String> {
    match &self.repo_details.mode {
      Mode::Server { remote_address } => Some(remote_address),
      _ => None,
    }
  }
  pub fn is_server_mode(&self) -> bool {
    match &self.repo_details.mode {
      Mode::Server { .. } => true,
      _ => false,
    }
  }
  pub fn commit<
    A: ActionExt + Serialize + for<'de> Deserialize<'de> + Debug,
  >(
    &mut self,
    comment: String,
  ) -> Result<(), String> {
    let mut docs_to_commit = vec![];
    let ctx = self.ctx();
    for document_id in &self.repo_details.document_ids {
      let doc = self.repo_details.document_get::<A>(ctx, *document_id)?;
      if doc.check_has_staging_aob() {
        docs_to_commit.push(doc);
      }
    }
    if docs_to_commit.is_empty() {
      return Err("Nothing to commit.".to_string());
    }
    // Create empty commit
    let mut commit = Commit::new(ctx.uid.to_string(), comment);
    // Add all docs to commit
    docs_to_commit.into_iter().for_each(|mut d| {
      d.commit(&mut commit);
      let _ = d.save_to_fs(ctx);
    });
    // Add commit as local commit
    CommitLog::add_local_commit(ctx, commit)?;
    // Return ok
    Ok(())
  }
  pub fn get_doc<
    A: ActionExt + Serialize + for<'de> Deserialize<'de> + Debug,
  >(
    &self,
    document_id: Uuid,
  ) -> Result<Document<A>, String> {
    let ctx = self.ctx();
    self.repo_details.document_get(ctx, document_id)
  }
  pub fn reset_index<
    A: ActionExt + Serialize + for<'de> Deserialize<'de> + Debug,
  >(
    &mut self,
    index: &mut impl IndexExt<ActionType = A>,
  ) -> Result<(), String> {
    let ctx = self.ctx();
    index.reset_docrefs()?;
    for document_id in &self.repo_details.document_ids {
      let doc = self.repo_details.document_get::<A>(ctx, *document_id)?;
      index.sync_doc(&doc)?;
    }

    Ok(())
  }
  pub fn get_staging_aobs<
    A: ActionExt + Serialize + for<'de> Deserialize<'de> + Debug,
  >(
    &self,
  ) -> Result<Vec<ActionObject<A>>, String> {
    let mut res = vec![];
    for document_id in &self.repo_details.document_ids {
      if let Ok(doc) = self.get_doc::<A>(*document_id) {
        let aobs = doc.get_staging_aobs();
        if !aobs.is_empty() {
          aobs.into_iter().for_each(|aob| res.push((*aob).clone()));
        }
      }
    }
    Ok(res)
  }
  pub fn add_aob<
    A: ActionExt + Serialize + for<'de> Deserialize<'de> + Debug,
  >(
    &mut self,
    new_aob: ActionObject<A>,
    index: &mut impl IndexExt<ActionType = A>,
  ) -> Result<(), String> {
    // 1) Check if document exist
    let doc = match self.get_doc(new_aob.object_id) {
      // If document exist, add aob to it
      Ok(doc) => doc.add_aob(new_aob)?,
      // If document does not exit
      Err(_) => {
        // Check if action create
        match &new_aob.action {
          // If create action, then create it
          ActionKind::Create(_) => {
            let ctx = self.ctx().to_owned();
            self.repo_details.document_create(&ctx, new_aob)?
          }
          ActionKind::Patch(_) => {
            return Err("Patch aob cannot be added to a not existing document. Pull first.".to_string());
          }
        }
      }
    };
    // Update index with the updated doc
    index.sync_doc(&doc)?;
    // Save updated doc to the FS
    doc.save_to_fs(self.ctx())?;
    // Return Ok
    Ok(())
  }
  /// Add AOB as a server side method
  /// This method does not save the updated doc to the FS
  /// Please handle the FS save later
  pub fn add_aob_server_side<
    A: ActionExt + Serialize + for<'de> Deserialize<'de> + Debug,
  >(
    &mut self,
    new_aob: ActionObject<A>,
  ) -> Result<Document<A>, String> {
    let doc = match &new_aob.action {
      ActionKind::Create(_) => {
        let ctx = self.ctx().to_owned();
        self.repo_details.document_create(&ctx, new_aob)?
      }
      ActionKind::Patch(_) => {
        let doc: Document<A> = self.get_doc(new_aob.object_id)?;
        // Add aob as server side procedure
        // Just append after parent id check!
        doc.add_aob_server_side(new_aob)?
      }
    };
    Ok(doc)
  }
  fn ctx(&self) -> &Context {
    &self.ctx
  }
  fn ctx_mut(&mut self) -> &mut Context {
    &mut self.ctx
  }
  /// Load repository
  pub fn load(ctx: Context) -> Result<Self, String> {
    // Load repo details
    let repo_details = RepoDetails::load(&ctx)?;
    // Create res
    let res = Self { ctx, repo_details };
    Ok(res)
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
    let res = Self { ctx, repo_details };
    Ok(res)
  }
  // Clone remote repository to local
  fn clone(
    remote_url: &str,
    index: &mut impl IndexExt,
  ) -> Result<Self, String> {
    // TODO! Fix path and UID
    let ctx = Context::init(PathBuf::from("./data"), "mezeipetister".into());
    // Check if repository inited
    if Self::load(ctx.clone()).is_ok() {
      return Err("Existing repository. Cannot clone again".into());
    }

    unimplemented!()
  }

  // Start server
  pub fn start_server<A>(self) -> BitServer<A>
  where
    A: ActionExt + Serialize + for<'de> Deserialize<'de> + Debug,
  {
    BitServer {
      inner: Arc::new(Mutex::new(ServerInner {
        repository: self,
        action_kind: PhantomData,
      })),
    }
  }

  /// Pull remote repository
  pub fn proceed_pull<A>(
    &mut self,
    index: &mut impl IndexExt<ActionType = A>,
  ) -> Result<(), String>
  where
    A: ActionExt + Serialize + for<'de> Deserialize<'de> + Debug,
  {
    let remote_addr = match &self.repo_details.mode {
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

    runtime.block_on(async {
      let ctx = self.ctx().to_owned();

      // Get last local REMOTE commit id
      let after_commit_id = CommitIndex::latest_remote_commit_id(&ctx)
        .map(|i| i.to_string())
        .unwrap_or("".to_string());

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

        // Save commit as remote commit
        CommitLog::add_remote_commit(&ctx, commit.clone())
          .expect("Failed to add remote commit");

        // Process commit
        for aob_str in &commit.serialized_actions {
          let aob: ActionObject<A> = serde_json::from_str(aob_str).unwrap();
          self.add_aob(aob, index).unwrap();
        }
      }
    });

    Ok(())
  }
  /// Push repository local commits to remote
  pub fn proceed_push<A>(
    &mut self,
    index: &mut impl IndexExt<ActionType = A>,
  ) -> Result<(), String>
  where
    A: ActionExt + Serialize + for<'de> Deserialize<'de> + Debug,
  {
    let remote_addr = match &self.repo_details.mode {
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

      let mut local_commits = self.local_commits().unwrap();

      // Update first local commit with tha latest remote commit id
      if let Some(mut commit) = local_commits.first_mut() {
        let ctx = self.ctx().to_owned();
        commit.ancestor_id = CommitIndex::latest_remote_commit_id(&ctx);
      }

      let local_commit_objects = local_commits
        .into_iter()
        .map(|c| CommitObj {
          obj_json_string: serde_json::to_string(&c).unwrap(),
        })
        .collect::<Vec<CommitObj>>();

      for commit in local_commit_objects {
        info!("Sending commit obj");
        let mut commit = remote_client.push(commit).await.unwrap().into_inner();
        info!("Commit received back");
        // Deserialize commit obj
        let c: Commit = serde_json::from_str(&commit.obj_json_string)
          .expect("Error deserializing response commit obj");
      }

      let ctx = self.ctx();
      // Reset local commits
      CommitLog::reset_local_commits(ctx)
        .expect("Error reseting local commits");
    });

    // Proceed pull operation
    self.proceed_pull::<A>(index)?;

    Ok(())
  }
  /// Clean local repository, clear local changes
  /// And performs remote pull
  pub fn proceed_clean(&self, index: &mut impl IndexExt) -> Result<(), String> {
    unimplemented!()
  }
  pub fn print_commit_index(&self) -> Result<(), String> {
    let ctx = self.ctx();
    let local = CommitIndex::latest_local_commit_id(ctx);
    let remote = CommitIndex::latest_remote_commit_id(ctx);
    println!("Latest local commit id: {:?}", local);
    println!("Latest remote commit id: {:?}", remote);
    Ok(())
  }
  pub fn local_commits(&self) -> Result<Vec<Commit>, String> {
    CommitLog::load_locals(self.ctx())
  }
  pub fn remote_commits(&self) -> Result<Vec<Commit>, String> {
    CommitLog::load_remotes(self.ctx())
  }
  pub fn remote_commits_after(
    &self,
    after_id: Uuid,
  ) -> Result<Vec<Commit>, String> {
    CommitLog::load_remotes_after(self.ctx(), after_id)
  }
}

pub struct BitServer<A> {
  pub inner: Arc<Mutex<ServerInner<A>>>,
}

// Server implementation
impl<A> BitServer<A>
where
  A: ActionExt + Serialize + for<'de> Deserialize<'de> + Debug + 'static,
{
  /// Start remote server
  /// Consumes self into server
  pub fn serve(self) -> Result<(), String> {
    let server_addr =
      match &self.inner.lock().unwrap().repository.repo_details.mode {
        Mode::Server {
          remote_address: server_addr,
        } => server_addr.to_string(),
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
        .add_service(ApiServer::new(self))
        .serve(server_addr.parse().unwrap())
        .await
        .expect("Error starting server");
    });
    Ok(())
  }
}

pub struct ServerInner<A> {
  pub repository: Repository,
  action_kind: PhantomData<A>,
}

impl<A> ServerInner<A>
where
  A: ActionExt + Serialize + for<'de> Deserialize<'de> + Debug + 'static,
{
  pub fn merge_push_request(
    &mut self,
    commit_str: &str,
  ) -> Result<Commit, String> {
    // First deserialize commit from str
    let mut commit: Commit = serde_json::from_str(commit_str)
      .map_err(|_| "Error during deserialize commit json object".to_string())?;

    // Let create ctx for file operations
    let ctx = self.repository.ctx().to_owned();

    // Check if commit parent id is ok
    let last_remote_commit_id = CommitIndex::latest_remote_commit_id(&ctx);
    if last_remote_commit_id != commit.ancestor_id {
      return Err(
        "Commit parent id error. Please pull before commit.".to_string(),
      );
    }

    let mut updated_docs: Vec<Document<A>> = vec![];
    let mut signed_aobs: Vec<ActionObject<A>> = vec![];

    // Process aobs
    for aob_str in &commit.serialized_actions {
      // Deserialize aob
      let mut aob: ActionObject<A> = serde_json::from_str(aob_str)
        .map_err(|_| "Error during aob deserialization".to_string())?;
      // Sign AOB to be a remote one!
      aob.remote_sign();
      // Try to add aob
      let doc = self.repository.add_aob_server_side(aob.clone())?;
      // Add unsaved new docs to the result vector
      // FS save later
      doc.save_to_fs(&ctx)?;
      updated_docs.push(doc);
      // Add signed aob to Signed aobs
      signed_aobs.push(aob);
    }

    // Sign commit
    commit.add_remote_signature()?;
    // Change aobs to signed ones
    commit.serialized_actions = signed_aobs
      .into_iter()
      .map(|aob| serde_json::to_string(&aob).unwrap())
      .collect::<Vec<String>>();

    // Save commit to FS
    CommitLog::add_remote_commit(&ctx, commit.clone())?;

    // Save updated docs to the FS
    for doc in updated_docs {
      // doc.save_to_fs(&ctx)?;
    }

    // Return commit as signed
    Ok(commit)
  }
}
