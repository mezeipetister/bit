use crate::{
    context::Context,
    db::Database,
    prelude::BitResult,
    sync::{Message, ToMessage},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use uuid::Uuid;

struct Signature([u8; 20]);

impl Signature {
    fn as_slice(&self) -> &[u8] {
        &self.0
    }
    fn to_string(&self) -> String {
        hex::encode(&self.0)
    }
}

fn sha1_sign<T>(i: T) -> BitResult<Signature>
where
    T: Serialize,
{
    let mut hasher = Sha1::new();
    hasher.update(&bincode::serialize(&i)?);
    let signature = hasher
        .finalize()
        .as_slice()
        .try_into()
        .expect("Error during converting sha1 to 20 bytes array");
    Ok(Signature(signature))
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CommitCandidate {
    id: Uuid,
    uid: String,
    dtime: DateTime<Utc>,
    message: String,
    entries: Vec<Entry>,
    previous_commit_id: Uuid,
}

impl CommitCandidate {
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn uid(&self) -> &str {
        &self.uid
    }
    pub fn message(&self) -> &str {
        &self.message
    }
    pub fn dtime(&self) -> DateTime<Utc> {
        self.dtime
    }
    pub fn previous_commit_id(&self) -> Uuid {
        self.previous_commit_id
    }
    pub fn etries(&self) -> &Vec<Entry> {
        &self.entries
    }
    fn sign(&self) -> BitResult<Signature> {
        sha1_sign(&self)
    }
    pub fn from_staging(
        ctx: &Context,
        staging: &Staging,
        message: String,
        previous_commit_id: Uuid,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            uid: ctx.username().into(),
            dtime: Utc::now(),
            message,
            entries: staging.entries.clone(),
            previous_commit_id,
        }
    }
    pub fn set_previous_commit_id(&mut self, pci: Uuid) {
        self.previous_commit_id = pci;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Commit {
    data: CommitCandidate,
    signature: String,
}

impl Commit {
    pub fn from_candidate(db: &Database, ctx: &Context, data: CommitCandidate) -> BitResult<Self> {
        // SERVER ONLY
        if !ctx.mode_is_server() {
            panic!("Signing is not allowed in local mode");
        }
        let signature = data.sign()?.to_string();
        Ok(Self { data, signature })
    }
    pub fn has_valid_signature(&self) -> bool {
        self.signature == self.data.sign().unwrap().to_string()
    }
    pub fn data(&self) -> &CommitCandidate {
        &self.data
    }
    pub fn signature_str(&self) -> &str {
        &self.signature
    }
}

impl ToMessage for Vec<Commit> {
    fn to_message(self, ctx: &Context) -> crate::sync::Message {
        Message::new_response(ctx, crate::sync::Status::Ok)
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Staging {
    entries: Vec<Entry>,
}

impl Staging {
    pub fn reset(&mut self) {
        self.entries = Vec::new();
    }
    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Entry {
    id: Uuid,             // Entry id
    dtime: DateTime<Utc>, //
    uid: String,          // User id
    command: String,      // String
    params: String,       // JSON encoded string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candidate_signature() {
        // assert_eq!(
        //     "ef38fd3c89d9b6cf432d391705084b4d79b31d39",
        //     CommitCandidate::default().sign().unwrap().to_string()
        // );
    }
}
