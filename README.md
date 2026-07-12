# poprako-orchestra-extra

Practical extensions for Rust applications built on
[`poprako-orchestra`](https://crates.io/crates/poprako-orchestra).

`poprako-orchestra-extra` is the companion crate for utilities and reusable
abstractions that complement the `poprako-orchestra` ecosystem. Each extension
remains opt-in through its own Cargo feature, so applications depend only on
the additions they use.

## Available extensions

### `promise`

The current extension provides the producer-side contract for a local outbox
table. An application implements a `Prom` step to persist a message alongside
its business transaction; a separate dispatcher can later read the outbox and
deliver the message through the application's chosen transport.

It deliberately does not prescribe a database, serialization format, or
dispatcher. It supplies the compatibility layer needed to model this write path
inside the `poprako-orchestra` ecosystem.

## Installation

Extensions are opt-in. Enable the currently available `promise` feature:

```toml
[dependencies]
poprako-orchestra-extra = { version = "0.2", features = ["promise"] }
```

## Core concepts

| Item | Responsibility |
| --- | --- |
| `prom::task::Task` | Stable message ID, payload, and optional delivery delay for one outbox record. |
| `prom::oper::Defer` | Operation that persists one `Task` and is generic over its successful output. |
| `prom::oper::DeferBatch` | Operation that persists a borrowed slice of `Task` values and is generic over its successful output. |
| `prom::Prom` | Contract that declares independent single-task and batch output types and implements both matching `Step` values. |

`Task::delay` controls delivery eligibility: `None` makes the message eligible
immediately, while `Some(Duration)` requests a minimum delay measured from the
time the outbox record is persisted.

## Usage

Implement both `poprako_orchestra::Step<Defer<'_, I, P, O>, C>` and
`poprako_orchestra::Step<DeferBatch<'_, '_, I, P, O>, C>` for the adapter that
writes to the local outbox. Then implement `Prom` to associate the distinct
successful output types for the single-task and batch paths.

```rust,ignore
use poprako_orchestra::Step;
use poprako_orchestra_extra::prom::oper::{Defer, DeferBatch};
use poprako_orchestra_extra::prom::task::Task;
use poprako_orchestra_extra::prom::Prom;

struct OutboxProducer;

impl Step<Defer<'_, String, [u8], OutboxRecordId>, DatabaseTransaction> for OutboxProducer {
    type Error = DatabaseError;

    async fn step(
        &self,
        transaction: &mut DatabaseTransaction,
        defer: &Defer<'_, String, [u8], OutboxRecordId>,
    ) -> Result<OutboxRecordId, Self::Error> {
        transaction.insert_outbox(
            defer.task.id.as_ref(),
            defer.task.payload,
            defer.task.delay,
        )
    }
}

impl Step<DeferBatch<'_, '_, String, [u8], Vec<OutboxRecordId>>, DatabaseTransaction>
    for OutboxProducer
{
    type Error = DatabaseError;

    async fn step(
        &self,
        transaction: &mut DatabaseTransaction,
        defer_batch: &DeferBatch<'_, '_, String, [u8], Vec<OutboxRecordId>>,
    ) -> Result<Vec<OutboxRecordId>, Self::Error> {
        transaction.insert_outbox_batch(defer_batch.tasks)
    }
}

impl Prom<DatabaseTransaction, String, [u8]> for OutboxProducer {
    type IndivOutput = OutboxRecordId;

    type BatchOutput = Vec<OutboxRecordId>;
}

fn requires_outbox_producer<P>(producer: P)
where
    P: Prom<DatabaseTransaction, String, [u8]>,
{
    // Use `producer` wherever an outbox-producing step is required.
}
```

The producer should persist its outbox row or batch atomically with the business
change that triggered it. A batch implementation may use a bulk database
operation, while its single-task implementation may use a dedicated one-row
operation. The dispatcher, retry policy, transport, and payload encoding remain
application concerns.

## Version policy

- Rust edition **2024** — requires Rust 1.85+.
- Before 1.0, minor versions may include breaking changes. Pin your version.

## License

Licensed under the [MIT License](LICENSE).
