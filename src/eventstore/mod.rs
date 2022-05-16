//! Event store trait and implementations
#[cfg(feature = "eventstore")]
use super::cloudevents::CloudEvent;
use super::{Event, Result};

#[cfg(feature = "eventstore")]
pub use self::inmemory::MemoryEventStore;

#[cfg(feature = "orgeventstore")]
pub use self::orgeventstore::OrgEventStore;

#[cfg(feature = "eventstore")]
/// Trait required for event stores. For the moment, event stores are append-only
pub trait EventStore {
    fn append<T>(&self, evt: impl Event, stream: &str) -> Result<T>;
}

mod inmemory;
#[cfg(feature = "orgeventstore")]
mod orgeventstore;
