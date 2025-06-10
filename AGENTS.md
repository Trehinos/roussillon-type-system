# Repository Information

This repository contains the Rust crate **roussillon-type-system**. It offers a basic type system for a future programming language. The crate provides:

- Primitive types and related values (boolean, bytes, numbers, references, lists).
- Algebraic data types such as `SumType` and `ProductType`.
- Custom types (`Structure`, `Enumeration`) and functional types (`FunctionType`).
- Values for these types including sequences, records, unions and functions.

The code is organized under `src/` with separate modules for types (`src/types`), values (`src/value`), and helper utilities.

## Development Notes
- Run `cargo test` to build the crate and execute the test suite. This ensures that the code compiles correctly.
- The optional formatting step using `cargo fmt` is recommended when available but not required.
- Do not commit the `target/` directory (already ignored in `.gitignore`).
