use std::io::Read;

use crate::{
    context::Context,
    prelude::{BitError, BitResult},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha1::{
    digest::{generic_array::GenericArray, typenum::TypeArray},
    Digest, Sha1,
};
use uuid::Uuid;

fn sha1_bytes_to_string(b: [u8; 20]) -> String {
    hex::encode(b)
}

fn sha1_sign<T>(i: T) -> BitResult<[u8; 20]>
where
    T: Serialize,
{
    let mut hasher = Sha1::new();
    hasher.update(&bincode::serialize(&i)?);
    let res = hasher
        .finalize()
        .as_slice()
        .try_into()
        .expect("Error during converting sha1 to 20 bytes array");
    Ok(res)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitCandidate {
    id: Uuid,
    uid: String,
    dtime: String,
    message: String,
    entries: Vec<Entry>,
    previous_commit_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Commit {
    data: CommitCandidate,
    signature: String,
}

impl Commit {
    // TODO! DB access, to check prev commit id
    pub fn new(ctx: &Context, data: CommitCandidate) -> BitResult<Self> {
        if !ctx.mode_is_server() {
            panic!("Signing is not allowed in local mode");
        }
        // TODO! Check commit prev id here
        let signature = sha1_bytes_to_string(sha1_sign(bincode::serialize(&data)?)?);
        Ok(Self { data, signature })
    }
    pub fn has_valid_signature(&self) -> bool {
        sha1_bytes_to_string(sha1_sign(&self.data).unwrap()) == self.signature
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Staging {
    entries: Vec<Entry>,
}

#[derive(Serialize, Deserialize, Debug)]
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
    fn test_sha1() {
        assert_eq!(
            "77d8d459c5fab783862eecb74ad4a53afd04c45d",
            &sha1_bytes_to_string(sha1_sign("hello").unwrap())
        );
    }
}
