# Chronicle

Chronicle is a Rust library for processing observations from devices and experiments. It provides explicit channel identities, signed microsecond timestamps, finite numeric values, and units. Batches retain input order and repeated observations so later processing can apply the required time and duplicate policies.

## Current capabilities

- Construct and validate an observation without losing its channel or unit.
- Retain empty or mixed-channel batches without sorting or removing records.
- Iterate over observations without exposing mutable access to their fields.
- Run a small example using only the Rust standard library.

Input decoding, time alignment, aggregation, and durable processing are separate capabilities to be added as their contracts are established. There is currently no file or streaming decoder.

## Build and check

Install a Rust toolchain and a C linker. On Ubuntu, the linker is provided by `build-essential`. When using rustup in a shell that has not loaded it, run:

```sh
. "$HOME/.cargo/env"
```

From the repository root:

```sh
cargo test --offline --locked
cargo run --offline --locked --example observations
```

The current package has no third-party crate dependencies. `sh scripts/check.sh` runs the library tests and the example using the installed toolchain.

## Data contract

See [the observation contract](docs/observations.md) for identifiers, timestamp meaning, units, value validation, and batch behavior. The proposed delimited input format is specified in [the ingestion contract](docs/ingestion.md).

```rust
use chronicle_timeseries::{Batch, Observation, Unit};

let reading = Observation::new("motor.temperature", 1_250_000, 23.5, Unit::Celsius)
    .expect("valid temperature observation");
let batch = Batch::new(vec![reading]);
assert_eq!(batch.len(), 1);
```
