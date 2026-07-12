//! Producer-side abstractions for a local outbox table.
//!
//! `prom` describes the write path of an outbox: application code invokes a
//! `Prom` step with a message identity, payload, and optional delay; the step
//! persists that information in the local outbox table. A separate dispatcher
//! can later read the table and deliver the message, allowing the producer to
//! participate in the `poprako-orchestra` step ecosystem without coupling to a
//! transport or dispatcher implementation.

use poprako_orchestra::Step;

use self::oper::{Defer, DeferBatch};

/// Data types shared by outbox producer operations.
///
/// The [`Task`] type defined here is used by both [`Defer`](super::oper::Defer)
/// and [`DeferBatch`](super::oper::DeferBatch) so that one-message and batch
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
pub trait Prom<C, I, P>
where
    I: AsRef<str>,
    P: ?Sized,
    Self: for<'a> Step<Defer<'a, I, P, Self::IndivOutput>, C>
        + for<'t, 'a> Step<DeferBatch<'t, 'a, I, P, Self::BatchOutput>, C>,
{
    /// Successful value produced when one task is deferred.
    type IndivOutput;

    /// Successful value produced when a batch of tasks is deferred.
    type BatchOutput;
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::future::Future;

    struct Context;

    struct Producer;

    impl Step<Defer<'_, String, [u8], String>, Context> for Producer {
        type Error = ();

        fn step(
            &self,
            _: &mut Context,
            _: &Defer<'_, String, [u8], String>,
        ) -> impl Future<Output = Result<String, Self::Error>> + Send {
            async { Ok(String::new()) }
        }
    }

    impl Step<DeferBatch<'_, '_, String, [u8], usize>, Context> for Producer {
        type Error = ();

        fn step(
            &self,
            _: &mut Context,
            _: &DeferBatch<'_, '_, String, [u8], usize>,
        ) -> impl Future<Output = Result<usize, Self::Error>> + Send {
            async { Ok(0) }
        }
    }

    impl Prom<Context, String, [u8]> for Producer {
        type IndivOutput = String;

        type BatchOutput = usize;
    }

    fn require_prom<T>()
    where
        T: Prom<Context, String, [u8]>,
    {
    }

    #[test]
    fn producer_implements_prom() {
        require_prom::<Producer>();
    }
}
