//! The [`Task`] payload type for outbox producer operations.
//!
//! [`Task`] carries the stable message identity, payload, and optional delay
//! that a producer persists to the local outbox table. It is the data unit
//! shared by [`Defer`](crate::prom::oper::Defer) and
//! [`DeferBatch`](crate::prom::oper::DeferBatch).

use std::time::Duration;

/// The data required to persist one deferred-delivery task.
///
/// `Task` is shared by [`super::oper::Defer`] and
/// [`super::oper::DeferBatch`], ensuring that one-message and batch operations
/// carry identical per-message data.
pub struct Task<'a, I, P>
where
    I: AsRef<str>,
    P: ?Sized,
{
    /// Stable message identity used for idempotency and outbox record lookup.
    pub id: &'a I,
    /// Message body to persist for a dispatcher to deliver later.
    pub payload: &'a P,
    /// Optional minimum delay before delivery becomes eligible.
    ///
    /// `None` means the message is eligible immediately. `Some(duration)` is
    /// interpreted relative to the time the producer persists the outbox
    /// record.
    pub delay: Option<Duration>,
}
