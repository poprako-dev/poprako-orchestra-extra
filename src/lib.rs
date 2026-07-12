//! Opt-in extensions for the `poprako-orchestra` step ecosystem.
//!
//! This crate provides producer-side outbox abstractions as an optional add-on
//! to [`poprako-orchestra`]. The available extensions are gated behind Cargo
//! features:
//!
//! - **`promise`** — Producer-side step contracts (`Defer`, `DeferBatch`) and
//!   the `Prom` trait, enabling an orchestration step to persist messages in a
//!   local outbox table for later delivery.
//!
//! See the [README] for a complete usage example and the full feature list.
//!
//! [README]: https://crates.io/crates/poprako-orchestra-extra

/// Outbox producer abstractions compatible with the `poprako-orchestra` ecosystem.
///
/// Enable the `promise` feature to use this module. It defines the producer-side
/// step contracts and the data passed to a producer when one or more messages
/// must be deferred for later delivery.
#[cfg(feature = "promise")]
pub mod prom;
