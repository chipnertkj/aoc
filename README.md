# Advent of Code 2024

This repository contains my solutions to the [Advent of Code 2024](https://adventofcode.com/2024) puzzles. This is my first time participating in the event. I decided to use it as an opportunity to reflect on how far I've come as a self-taught programmer. It also serves as my attempt to strike a good balance between simplicity and performance in the solutions.

## Structure

This repository is a [Cargo virtual manifest](https://doc.rust-lang.org/cargo/reference/workspaces.html#virtual-workspace). Each puzzle solution is located in a separate binary crate, along with some benchmarking code and minimal tests.

## Input

The input for each puzzle is fed by providing the path to an input file after starting the program.
Sample inputs for tests are provided through `sample_input.txt` files and embedded at compile-time into the binary.

Many of the functions used in the solutions are generic over the data type used for calculations. This allows optimizing the underlying data type to optimize performance based on benchmarks.

## Running

At the time of writing, compiling the solutions requires a nightly version of Rust.
You can install it with [rustup](https://rustup.rs/).

```bash
rustup install nightly
```

Solutions can be executed with Cargo either by entering the directory containing a challenge and running the project

```bash
cd day1
cargo run
```

or by specifying the binary directly.

```bash
cargo run --release --bin day1
```

## Documentation

Most of the code items in this repository are (minimally) documented using [doc comments](https://doc.rust-lang.org/rust-by-example/meta/doc.html#doc-comments).

You can view the documentation for each solution as well as dependencies by running `cargo doc` on the repository.

```bash
cargo doc --open
```

## Testing

I have provided basic tests for each solution.
These cover the sample inputs provided in each challenge, as well as additional test cases I used to verify the correctness of the solutions during implementation.

The solutions can be tested by running `cargo test` in a challenge directory

```bash
cd day1
cargo test
```

or by specifying the binary directly.

```bash
cargo test --bin day1
```

## Benchmarking

I have provided basic [benchmark tests](https://doc.rust-lang.org/nightly/unstable-book/library-features/test.html) for the most important parts of the solutions.
These benchmarks only run on challenge inputs. Proper benchmarks would use randomized inputs for more accuracy.

You can test the performance of each solution by running `cargo bench` in a challenge directory

```bash
cd day1
cargo bench
```

or by specifying the binary directly.

```bash
cargo bench --bin day1
```

You're welcome to analyze the implementation of the benchmarking code and the solutions themselves. Benchmarking functions are marked with the `#[bench]` attribute.
