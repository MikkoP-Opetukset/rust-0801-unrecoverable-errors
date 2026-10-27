#![allow(dead_code, unused_variables, unused_mut)]

/// Entry point for Chapter 9.1: Unrecoverable Errors with `panic!`
///
/// Sometimes something goes wrong and there is nothing the program can do
/// about it. For those cases Rust has the `panic!` macro. This file covers the
/// two ways a panic happens, what Rust does after a panic (unwind or abort),
/// and how to read a backtrace.
///
/// Every panicking line is commented out so the whole demo runs to the end.
/// Uncomment one line at a time and run the program to see the failure
/// message, then run with `RUST_BACKTRACE=1` to see the call stack.
fn main() {
    calling_panic_directly();
    panicking_from_an_invalid_index();
    unwinding_vs_aborting();
    reading_a_backtrace();
    when_to_panic();
}

/// # Calling `panic!` Directly
/// - One way to cause a panic is to do something that panics; the other is to
///   call `panic!` ourselves.
/// - `panic!` prints a failure message, unwinds and cleans up the stack, and
///   quits. Nothing after it runs.
/// - We use it when a state is one we are not prepared to handle.
fn calling_panic_directly() {
    println!("\n{:=>80}", "");
    println!("calling_panic_directly()\n");

    println!("Before the panic: the program is still running.");

    // Uncomment to see the program stop here. The last two lines of the output
    // are the panic message and the place in the source where it happened:
    //
    //   thread 'main' panicked at src/main.rs:LINE:COL:
    //   the door is locked
    //   note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    //
    // panic!("the door is locked");

    println!("After the panic: this line runs only when the panic is commented out.");
}

/// # Panicking from an Invalid Operation
/// - Reading `v[99]` from a `Vec` of three elements panics: `[]` promises an
///   element, and for an invalid index there is no correct value to return.
/// - In C this is a *buffer overread*: you get whatever is in the memory after
///   the structure, which is undefined behaviour and a common security problem.
/// - Rust refuses to continue instead. This bounds check is part of the Rusts
///   safety guarantee
fn panicking_from_an_invalid_index() {
    println!("\n{:=>80}", "");
    println!("panicking_from_an_invalid_index()\n");

    let inventory = vec![1, 2, 3];

    // Uncomment to see:
    //
    //   thread 'main' panicked at src/main.rs:LINE:COL:
    //   index out of bounds: the len is 3 but the index is 99
    //
    // inventory[99];

    // The non-panicking way to read an index is `get`, which returns an
    // `Option`. A missing element becomes a value we can handle instead of a
    // crash. The next demo builds the same idea with `Result`.
    match inventory.get(99) {
        Some(value) => println!("slot 99 holds {value}"),
        None => println!("slot 99 does not exist: no panic, the case is handled."),
    }
}

/// # Unwinding the Stack or Aborting
/// - By default a panic *unwinds*: Rust walks back up the stack and cleans up
///   the data from each function it passes on the way out.
/// - Walking back and cleaning up is a lot of work, so Rust also allows
///   *aborting*: the program ends immediately and the operating system reclaims
///   the memory.
/// - Choose abort with `panic = 'abort'` in the appropriate `[profile]` section
///   of `Cargo.toml` when a small binary matters more than cleanup.
fn unwinding_vs_aborting() {
    println!("\n{:=>80}", "");
    println!("unwinding_vs_aborting()\n");

    // A type with a `Drop` implementation shows the difference between the two
    // behaviours: unwinding runs this cleanup, aborting does not.
    struct Gate {
        name: String,
    }

    impl Drop for Gate {
        fn drop(&mut self) {
            println!("Gate {} closes: cleanup ran.", self.name);
        }
    }

    let gate = Gate {
        name: String::from("north"),
    };

    println!("Default is unwinding, so the panic message prints and then the gate closes.");

    // Uncomment to see the panic message followed by "Gate north closes".
    // panic!("the gate is jammed");

    // With this in Cargo.toml the same panic prints the message and exits at
    // once, so the gate never closes:
    //
    //   [profile.release]
    //   panic = 'abort'
    //
    // Compare the two by running `cargo run` and `cargo run --release` with
    // that section added.
    println!(
        "After the panic: printed only for unwinding, and only when the panic is commented out."
    );
}

/// # Reading a Backtrace
/// - A *backtrace* is the list of functions that were called to reach the panic.
/// - Set `RUST_BACKTRACE` to any value except `0`: `RUST_BACKTRACE=1 cargo run`.
/// - Read the backtrace from the top until the first file you wrote. That is
///   where the problem started. The lines above it are code your code called,
///   the lines below are code that called your code.
/// - Backtraces need debug symbols, which are on by default for `cargo build`
///   and `cargo run` without `--release`. `RUST_BACKTRACE=full` shows more.
fn reading_a_backtrace() {
    println!("\n{:=>80}", "");
    println!("reading_a_backtrace()\n");

    let slots = vec![10, 20, 30];

    // Uncomment to panic inside `slot_value`, which is called from here.
    // let value = slot_value(&slots, 99);
    let value = slot_value(&slots, 1);
    println!("slot 1 holds {value}");

    // The reported position points inside `slot_value`, not at the call in
    // `main`, because that is where the `panic!` call actually happens. The
    // backtrace is what shows the path that led there:
    //
    //   thread 'main' panicked at src/main.rs:LINE:COL:
    //   index out of bounds: the len is 3 but the index is 99
    //   stack backtrace:
    //      0: rust_begin_unwind
    //      1: core::panicking::panic_fmt
    //      2: core::panicking::panic_bounds_check
    //      3: rust_0801_unrecoverable_errors::slot_value   <- our code
    //      4: rust_0801_unrecoverable_errors::reading_a_backtrace
    //      5: rust_0801_unrecoverable_errors::main
    //   note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
    //
    // Frame 3 is the first frame that is our own code, so that is where the
    // investigation starts. The exact frames depend on the Rust version,
    // because the slice-index frames between the panic and our function can be
    // inlined and left out.
}

/// A slot read that panics for an index outside the slice.
fn slot_value(slots: &[i32], index: usize) -> i32 {
    // Uncomment the panic to see a backtrace that ends in this function.
    // if index >= slots.len() {
    //     panic!("no slot {index} in a shelf of {}", slots.len());
    // }
    slots[index]
}

/// # When to Panic
/// - Panic is a reasonable choice for examples, tests, and prototypes, where a
///   failure means the code itself is wrong.
/// - It is also the choice when an invalid state is possible but cannot be
///   proven impossible at compile time, and when we know a value is `Some` or
///   `Ok` even though the type does not.
/// - In a program that must keep running, a panic is the last resort: prefer a
///   recoverable error. That is the next demo, Chapter 9.2 with `Result`.
/// - Tests fail when they panic, which is how the testing chapter uses `panic!`
///   to report a failing assertion.
fn when_to_panic() {
    println!("\n{:=>80}", "");
    println!("when_to_panic()\n");

    // A lookup that may or may not find something. We know this one is `Some`,
    // but the type does not.
    let found = find_level("Aria");
    println!("known to be Some: {found:?}");

    // `unwrap` is a panic in disguise: it calls `panic!` when the value is None.
    println!("unwrap on a Some value gives {}", found.unwrap());

    // Uncomment to see that same panic, reported from the standard library
    // rather than from our own line:
    //
    //   thread 'main' panicked at src/main.rs:LINE:COL:
    //   called `Option::unwrap()` on a `None` value
    //
    // let missing = find_level("Zed");
    // println!("{}", missing.unwrap());
}

/// A lookup that returns `None` for a name that is not on the scoreboard.
fn find_level(name: &str) -> Option<i32> {
    match name {
        "Aria" => Some(120),
        _ => None,
    }
}
