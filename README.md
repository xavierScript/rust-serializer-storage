# Generic Storage with Multiple Serialization Formats

Solana Turbine Accelerated Builders' Cohort 2026 Q2 challenge project.

This project implements a generic Rust storage container that can save and load data using multiple serialization formats:

- Borsh
- JSON (`serde_json`)
- Wincode-style serializer (currently backed by `bincode` in code)

## Objective

Build a generic storage system that can serialize and deserialize data using different formats while practicing:

- Traits and generic APIs
- Type bounds across multiple libraries
- `PhantomData` for zero-cost type tracking
- Result-based error handling

## What This Project Includes

- A `Serializer` trait with:
  - `to_bytes()`
  - `from_bytes()`
- Three serializer implementations:
  - `BorshSerializer`
  - `JsonSerializer`
  - `WincodeSerializer`
- A generic container:
  - `Storage<T, S>`
- Core storage methods:
  - `new(serializer: S)`
  - `save(&mut self, value: &T)`
  - `load(&self)`
  - `has_data(&self)`
- A test data type:
  - `Person { name: String, age: u32 }`
- Unit tests for all serializer flows

## Project Structure

- `src/main.rs` - challenge implementation + tests
- `Cargo.toml` - dependencies

## Requirements Checklist

- [x] Generic `Serializer` trait with byte conversion methods
- [x] `Result`-based error handling
- [x] Borsh serializer
- [x] JSON serializer
- [x] Wincode serializer implementation path
- [x] Generic `Storage<T, S>` with `PhantomData<T>`
- [x] Save/load/has_data methods
- [x] Test type deriving needed serialization traits
- [x] Unit tests for Borsh, JSON, and Wincode flows

## Run the Project

```bash
cargo run
```

## Run Tests

```bash
cargo test
```

