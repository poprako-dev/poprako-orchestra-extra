/// Outbox producer abstractions compatible with the `poprako-orchestra` ecosystem.
///
/// Enable the `promise` feature to use this module. It defines the producer-side
/// step contract and the data passed to a producer when a message must be
/// deferred for later delivery.
#[cfg(feature = "promise")]
pub mod prom;
