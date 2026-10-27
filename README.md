# 0801: Unrecoverable Errors

Material for the Rust Programming course @ SAMK

## Setting `RUST_BACKTRACE` in RustRover

> [!NOTE]
> At least on my system RustRover seems to use `RUST_BACKTRACE=1` by default.
> Try this, and if you see no difference in output between not setting the 
> variable and setting it, then try running the cargo run from terminal with 
> different settings. To set the variable in unix environments you can write 
> the variable in front of the run command. `RUST_BACKTRACE=1 cargo run`

To see a backtrace when a demo panics, set the `RUST_BACKTRACE` environment
variable in the run configuration:

1. Open **Run | Edit Configurations...**.
2. Select the Cargo run configuration for the demo (or create one with **+ | Cargo**).
3. In the **Environment variables** field, enter `RUST_BACKTRACE=1`.
   Use `RUST_BACKTRACE=full` for a more verbose backtrace.
4. Click **OK** and run the configuration.

To apply it to every new Cargo configuration, set it under
**Run | Edit Configurations... | Edit configuration templates... | Cargo**.

## References

This demo follows concepts and selected examples from
[The Rust Programming Language, Chapter 9.1: Unrecoverable Errors with panic!](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html).

The Rust Book is available under the MIT License and Apache License, Version 2.0.
