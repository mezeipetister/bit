use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{db::Database, prelude::BitResult};

pub trait EventData: Serialize + for<'de> Deserialize<'de> {
    const EVENT_NAME: &'static str;
    fn apply(db: &mut Database) -> BitResult<()>;
    fn event_name(&self) -> &'static str {
        Self::EVENT_NAME
    }
}

#[derive(Serialize, Deserialize)]
struct Event {
    event_id: Uuid,
    ddate: DateTime<Utc>,
    uid: String,
    target: String,
    event_obj: String,
}
