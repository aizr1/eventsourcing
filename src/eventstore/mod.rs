use super::{Event, Result};
use chrono::prelude::*;
use serde_json;
use uuid::Uuid;
use serde::Serialize;
use super::cloudevents::CloudEvent;

pub use self::inmemory::MemoryEventStore;

#[cfg(feature = "orgeventstore")]
pub use self::orgeventstore::OrgEventStore;

/*
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnrichedEvent {
    pub serial_number: String,
    pub event_type: String,
    pub schema_version: String,
    pub payload: String,
    pub timestamp: DateTime<Utc>,
}

impl<E> From<E> for EnrichedEvent
where
    E: Event,
{
    fn from(source: E) -> Self {
        EnrichedEvent {
            serial_number: Uuid::new_v4().hyphenated().to_string(),
            event_type: source.event_type().to_owned(),
            schema_version: source.event_type_version().to_owned(),
            payload: serde_json::to_string(&source).unwrap(),
            timestamp: Utc::now(),
        }
    }
} */
pub trait EventStore {    

    fn append(&self, evt: impl Event, stream: &str) -> Result<CloudEvent>;
}

mod inmemory;
#[cfg(feature = "orgeventstore")]
mod orgeventstore;