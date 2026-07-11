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
poprako-orchestra-extra = { version = "0.1", features = ["promise"] }
```

## Core concepts

| Item | Responsibility |
| --- | --- |
| `prom::oper::Defer` | Input passed to an outbox producer: a stable message ID, payload, and optional delivery delay. |
| `prom::Prom` | Marker trait for a `poprako-orchestra::Step` that persists `Defer` in the local outbox. |

`Defer::delay` controls delivery eligibility: `None` makes the message eligible
immediately, while `Some(Duration)` requests a minimum delay measured from the
time the outbox record is persisted.

## Usage

Implement `poprako_orchestra::Step<Defer<'_, I, P>, C>` for the adapter that
writes to the local outbox. The blanket `Prom` implementation then identifies
that adapter as an outbox producer in generic application code.

```rust,ignore
use std::time::Duration;

use poprako_orchestra::Step;
use poprako_orchestra_extra::prom::oper::Defer;
use poprako_orchestra_extra::prom::Prom;

struct OutboxProducer;

impl Step<Defer<'_, String, [u8]>, DatabaseTransaction> for OutboxProducer {
    type Error = DatabaseError;

    async fn step(
        &self,
        transaction: &mut DatabaseTransaction,
        defer: &Defer<'_, String, [u8]>,
    ) -> Result<(), Self::Error> {
        transaction.insert_outbox(
            defer.id.as_ref(),
            defer.payload,
            defer.delay.unwrap_or(Duration::ZERO),
        )
    }
}

fn requires_outbox_producer<P>(producer: P)
where
    P: Prom<DatabaseTransaction, String, [u8]>,
{
    // Use `producer` wherever an outbox-producing step is required.
}
```

The producer should persist its outbox row atomically with the business change
that triggered it. The dispatcher, retry policy, transport, and payload
encoding remain application concerns.

## Version policy

- Rust edition **2024** — requires Rust 1.85+.
- Before 1.0, minor versions may include breaking changes. Pin your version.

## License

Licensed under the [MIT License](LICENSE).
