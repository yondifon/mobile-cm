# mobile-cm (Rust)

Identify the Cameroonian operator (MTN, Orange, Nexttel, Camtel) that issued a phone number.

Not published to crates.io. No runtime dependencies.

## Install

Use as a local path dependency until/unless it's published:

```toml
[dependencies]
mobile-cm = { path = "../rust" }
```

## Usage

```rust
use mobile_cm::{check, is_mtn, Operator};

check("676777777"); // Some(Operator::Mtn)
check("+237699238282"); // Some(Operator::Orange)
check("123456789"); // None

is_mtn("676777777"); // true

check("676777777").unwrap().to_string(); // "mtn"
```

See [`../spec/README.md`](../spec/README.md) for the input rules, prefix table, and portability caveat: a prefix names the operator that issued the number, not necessarily the subscriber's current network.
