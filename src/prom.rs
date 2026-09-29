//! Producer-side abstractions for a local outbox table.
//!
//! `prom` describes the write path of an outbox: application code invokes a
//! `Prom` step with a message identity, payload, and optional delay; the step
//! persists that information in the local outbox table. A separate dispatcher
//! can later read the table and deliver the message, allowing the producer to
//! participate in the `poprako-orchestra` step ecosystem without coupling to a
//! transport or dispatcher implementation.

use poprako_orchestra::{Context, LevelGuard, Step};

use self::oper::{Defer, DeferBatch};

/// Data types shared by outbox producer operations.
///
/// The [`Task`](task::Task) type defined here is used by both [`Defer`]
/// and [`DeferBatch`] so that one-message and batch
/// persist operations carry identical per-message data.
pub mod task;

/// Operations accepted by an outbox producer.
pub mod oper;

/// A `poprako-orchestra` producer that supports one-message and batch deferral.
///
/// Implementors execute [`Defer`] for one outbox task and [`DeferBatch`] for a
/// borrowed slice of tasks. `C` is the execution context, `I` is the stable
/// message identity type, and `P` is the payload type. An implementation may
/// use different persistence strategies for the two operations, such as a
/// single-row insert for [`Defer`] and a bulk insert for [`DeferBatch`].
///
/// [`IndivOutput`](Prom::IndivOutput) and [`BatchOutput`](Prom::BatchOutput)
/// independently describe the successful values returned by the two operations.
/// Implementors must declare those types explicitly because a [`Step`] does not
/// expose an operation's [`Oper`](poprako_orchestra::Oper) output type for a
/// blanket `Prom` implementation to infer.
///
/// Both operations share [`Error`](Prom::Error). Transaction-level guards are
/// included in this contract, so generic callers can execute either operation
/// with only a `Prom` bound. Each operation retains its own required level.
pub trait Prom<C, I, P>
where
    C: Context,
    I: AsRef<str>,
    P: ?Sized,
    Self: for<'a> Step<Defer<'a, I, P, Self::IndivOutput>, C, Error = <Self as Prom<C, I, P>>::Error>
        + for<'t, 'a> Step<
            DeferBatch<'t, 'a, I, P, Self::BatchOutput>,
            C,
            Error = <Self as Prom<C, I, P>>::Error,
        > + for<'a> LevelGuard<C::Level, <Self as Step<Defer<'a, I, P, Self::IndivOutput>, C>>::Level>
        + for<'t, 'a> LevelGuard<
            C::Level,
            <Self as Step<DeferBatch<'t, 'a, I, P, Self::BatchOutput>, C>>::Level,
        >,
{
    /// Shared error returned by single-task and batch deferral.
    type Error;

    /// Successful value produced when one task is deferred.
    type IndivOutput;

    /// Successful value produced when a batch of tasks is deferred.
    type BatchOutput;
}

#[cfg(test)]
mod tests;
