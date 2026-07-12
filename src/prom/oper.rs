//! Outbox producer operations.
//!
//! This module defines [`Defer`] for persisting a single outbox task and
//! [`DeferBatch`] for persisting a slice of tasks in one operation. Both types
//! implement [`Oper`](poprako_orchestra::Oper) and are intended to be used as
//! the operation parameter of a [`Step`](poprako_orchestra::Step).

use std::marker::PhantomData;

use poprako_orchestra::Oper;

use crate::prom::task::Task;

/// One task to persist in the local outbox.
///
/// This operation has the same task data shape as [`DeferBatch`], but permits a
/// producer to use a single-message persistence path. `O` is the successful
/// value produced by the corresponding [`Step`](poprako_orchestra::Step).
pub struct Defer<'a, I, P, O>
where
    I: AsRef<str>,
    P: ?Sized,
{
    /// The task to persist for deferred delivery.
    pub task: Task<'a, I, P>,

    #[doc(hidden)]
    _m: PhantomData<O>,
}

impl<'a, I, P, O> Defer<'a, I, P, O>
where
    I: AsRef<str>,
    P: ?Sized,
{
    /// Creates a one-task operation with the output type selected by `O`.
    pub fn new(task: Task<'a, I, P>) -> Self {
        Self {
            task,
            _m: PhantomData,
        }
    }
}

impl<'a, I, P, O> Oper for Defer<'a, I, P, O>
where
    I: AsRef<str>,
    P: ?Sized,
{
    type Output = O;
}

/// Multiple tasks to persist in one deferred-delivery operation.
///
/// The slice borrow and the task-data borrow have independent lifetimes: `'t`
/// keeps the slice available to the producer and `'a` keeps each task's ID and
/// payload available. The producer should persist all tasks atomically with the
/// business change that caused them. A dispatcher can subsequently use every
/// stored task's identifier, payload, and delay to determine delivery. `O` is
/// the successful value produced by the corresponding
/// [`Step`](poprako_orchestra::Step).
pub struct DeferBatch<'t, 'a, I, P, O>
where
    I: AsRef<str>,
    P: ?Sized,
{
    /// The tasks to persist. An empty slice is valid and is a no-op unless an
    /// adapter documents different behavior.
    pub tasks: &'t [Task<'a, I, P>],

    #[doc(hidden)]
    _m: PhantomData<O>,
}

impl<'t, 'a, I, P, O> DeferBatch<'t, 'a, I, P, O>
where
    I: AsRef<str>,
    P: ?Sized,
{
    /// Creates a batch operation with the output type selected by `O`.
    pub fn new(tasks: &'t [Task<'a, I, P>]) -> Self {
        Self {
            tasks,
            _m: PhantomData,
        }
    }
}

impl<'t, 'a, I, P, O> Oper for DeferBatch<'t, 'a, I, P, O>
where
    I: AsRef<str>,
    P: ?Sized,
{
    type Output = O;
}
