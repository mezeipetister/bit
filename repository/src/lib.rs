use std::path::PathBuf;

use chrono::{DateTime, Utc};
use uuid::Uuid;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod fs;
pub mod prelude;
pub mod server;
pub mod sync;

struct Patch {
  id: Uuid,
  patch: String,
}

struct Path {
  path: PathBuf,
  object_id: String,
}

struct Object {
  path: Path,
  local_version: String,
  remote_version: String,
  removed: bool,
  version: Vec<Patch>,
  created: DateTime<Utc>,
  updated: DateTime<Utc>,
}
