//! Producer-side abstractions for a local outbox table.
//!
//! `prom` describes the write path of an outbox: application code invokes a
//! `Prom` step with a message identity, payload, and optional delay; the step
//! persists that information in the local outbox table. A separate dispatcher
//! can later read the table and deliver the message, allowing the producer to
//! participate in the `poprako-orchestra` step ecosystem without coupling to a
//! transport or dispatcher implementation.

use poprako_orchestra::Step;

use self::oper::Defer;

/// Operations accepted by an outbox producer.
pub mod oper;

/// A `poprako-orchestra` step that stores a message for deferred delivery.
///
/// Implement this trait for the infrastructure adapter that writes to the
/// application's local outbox table. `C` is the execution context, `I` is the
/// message identity type, and `P` is the payload type. The producer receives a
/// [`Defer`] value so callers can request either immediate eligibility or a
/// delayed delivery time without exposing storage details.
///
/// This marker trait intentionally adds no methods: the [`Step`] implementation
/// is the executable contract, while `Prom` expresses the adapter's role in the
/// outbox producer pipeline.
pub trait Prom<C, I, P>: for<'a> Step<Defer<'a, I, P>, C> {}

impl<T, C, I, P> Prom<C, I, P> for T where T: for<'a> Step<Defer<'a, I, P>, C> {}
