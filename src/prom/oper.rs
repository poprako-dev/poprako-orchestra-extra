use std::time::Duration;

/// Input supplied to an outbox producer when a message is deferred.
///
/// The producer should persist this input atomically with the business change
/// that caused it. A dispatcher can subsequently use the stored identifier,
/// payload, and delay to decide when and how to deliver the message. Keeping
/// references here lets the producer serialize or copy the values into its
/// storage format without imposing ownership or encoding choices on callers.
pub struct Defer<'a, I, P>
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
