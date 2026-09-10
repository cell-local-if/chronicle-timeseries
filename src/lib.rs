//! Typed device observations with explicit timestamps, units, and ordered batches.

mod batch;
mod observation;

pub use batch::Batch;
pub use observation::{Observation, ObservationError, Unit, UnitParseError};
