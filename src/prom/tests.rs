use std::future::Future;
use std::pin::pin;
use std::task::{Context as PollContext, Poll, Waker};

use poprako_orchestra::{AtLeast, Context, Level, Step};

use super::Prom;
use super::oper::{Defer, DeferBatch};
use super::task::Task;

struct Read;

impl Level for Read {}

struct Write;

impl Level for Write {}

impl AtLeast<Read> for Write {}

struct Transaction;

impl Context for Transaction {
    type Level = Write;
}

struct Producer;

impl Step<Defer<'_, String, [u8], String>, Transaction> for Producer {
    type Level = Read;

    type Error = &'static str;

    async fn step(
        &self,
        _: &mut Transaction,
        oper: &Defer<'_, String, [u8], String>,
    ) -> Result<String, Self::Error> {
        if oper.task.payload.is_empty() {
            return Err("empty payload");
        }

        Ok(oper.task.id.clone())
    }
}

impl Step<DeferBatch<'_, '_, String, [u8], usize>, Transaction> for Producer {
    type Level = Write;

    type Error = &'static str;

    async fn step(
        &self,
        _: &mut Transaction,
        oper: &DeferBatch<'_, '_, String, [u8], usize>,
    ) -> Result<usize, Self::Error> {
        if oper.tasks.iter().any(|task| task.payload.is_empty()) {
            return Err("empty payload");
        }

        Ok(oper.tasks.len())
    }
}

impl Prom<Transaction, String, [u8]> for Producer {
    type Error = &'static str;

    type IndivOutput = String;

    type BatchOutput = usize;
}

// A Prom bound alone must support both operations and propagate their errors.
async fn defer_both<C, P>(
    producer: &P,
    context: &mut C,
    id: &String,
    payload: &[u8],
) -> Result<(P::IndivOutput, P::BatchOutput), &'static str>
where
    C: Context,
    P: Prom<C, String, [u8], Error = &'static str>,
{
    let output = producer
        .step(
            context,
            &Defer::new(Task {
                id,
                payload,
                delay: None,
            }),
        )
        .await?;

    let tasks = [Task {
        id,
        payload,
        delay: None,
    }];

    let batch_output = producer.step(context, &DeferBatch::new(&tasks)).await?;

    Ok((output, batch_output))
}

async fn defer_batch<C, P>(
    producer: &P,
    context: &mut C,
    tasks: &[Task<'_, String, [u8]>],
) -> Result<P::BatchOutput, &'static str>
where
    C: Context,
    P: Prom<C, String, [u8], Error = &'static str>,
{
    let output = producer.step(context, &DeferBatch::new(tasks)).await?;

    Ok(output)
}

fn ready<T>(future: impl Future<Output = T>) -> T {
    let mut future = pin!(future);

    let mut context = PollContext::from_waker(Waker::noop());

    let Poll::Ready(output) = future.as_mut().poll(&mut context) else {
        panic!("in-memory producer must complete immediately");
    };

    output
}

#[test]
fn generic_prom_executes_distinct_outputs_and_levels() {
    let id = String::from("task-1");

    let result = ready(defer_both(&Producer, &mut Transaction, &id, &[1]));

    assert_eq!(result, Ok((id, 1)));
}

#[test]
fn generic_prom_propagates_shared_error() {
    let id = String::from("task-1");

    let result = ready(defer_both(&Producer, &mut Transaction, &id, &[]));

    assert_eq!(result, Err("empty payload"));
}

#[test]
fn generic_prom_batch_propagates_shared_error() {
    let id = String::from("task-1");

    let tasks = [Task {
        id: &id,
        payload: &[] as &[u8],
        delay: None,
    }];

    let result = ready(defer_batch(&Producer, &mut Transaction, &tasks));

    assert_eq!(result, Err("empty payload"));
}

#[test]
fn generic_prom_accepts_empty_batch() {
    let result = ready(defer_batch(&Producer, &mut Transaction, &[]));

    assert_eq!(result, Ok(0));
}
