# Classic Computer Science Problems in Rust

Rust port of David Kopec's "Classic Computer Science Problems" series, based on a fusion of his Java and Swift books.

Rather than being a plain port, I have chosen to lean into Rust's ecosystem and modify the programs to fit Rust's type system and idioms as applicable. Therefore, some of the examples are noticeably different and use functional programming idioms.

For instance, this port uses Nom to parse a string into a sequence of nucleotides in the "Compressed Gene" example in chapter 1.

## Examples

The examples are all under `/book`, organized into chapters.

Each example includes a short doc-comment overview with a link to the Java implementation.

For original examples such as `/book/chapter_01/fib6.rs`, there is obviously no link to a corresponding Java implementation.

Keep in mind that some examples are intentionally slow, like the naive fibonacci function implementation.

## How to build the examples

Simply run `cargo build`! Cargo will build all the examples for you. Or, you can ask Cargo to build just one example using the command described in the next section.

## How to run the examples

Much like the Java implementation, each example is its own program entry. However, Cargo allows us to use the `/src` directory as a common library, not unlike the shared `/Sources` directory in the Swift Playground version.

To run the examples, please follow these directions:

1. Check the lines with `[[bin]]` in `Cargo.toml`. Make a note of the name.
2. Open a terminal and run `cargo run --bin <NAME>`. For instance, `cargo run --bin fib1`.
3. Cargo will take a couple moments to collect dependencies, and then run the program.
4. That's it! If all goes well, you'll see the program's output in your terminal.

## Assumptions

I am assuming:
* You have downloaded and installed the Rust toolchain and verified that it is working.
* You are using a platform Rust readily supports.
* You are somewhat familiar with Rust and its standard library.

For the most part, this implementation does not dive into advanced features in Rust.

## Portability and External Dependencies

This implementation forgoes excluding external dependencies due to an unavoidable dependency on the `rand` crate and on the understanding that handling external Rust dependencies is very easy thanks to Cargo.

However, this implementation *does* still implement most of the code in the book, while also providing examples that use crates where applicable, particularly if they provide a different perspective on the same example.

For instance, Chapter 2 includes an example where I have ported Kopec's A*, BFS, and DFS implementations and an example where I use the `pathfinding` crate.

Kopec makes an excellent point of using only the Java standard library in his "Classic Computer Science Problems in Java" book - as any Node.js developer no doubt fully understands, external dependencies introduce entropy and maintainance costs into an otherwise self-contained program. At one point, some minor update or misconfiguration will result in the program failing in some way.

However, this restriction creates an unavoidable issue for a Rust implementation: Rust does not include any random number facility in its standard library. Suffice it to say, I am not interested in re-implementing the `rand` crate for the sake of avoiding external dependencies.

Furthermore, there are some excellent crates like the `nom` parser combinator crate which allow for elegant, functional approaches, inspired by Haskell's [Parsec library](https://hackage.haskell.org/package/parsec).

### Stable vs. Unstable Rust

*However*, this implementation does *not* use unstable / nightly Rust. All you need is a stable Rust toolchain for this implementation.

## How this Repository is Organized

This repository consists of a `/src` directory that holds shared code and a `/book` directory that consists of binary entrypoints which may or may not use code from the `/src` directory.

This implementation sits somewhere between the Swift and Java implementations - the shared `/src` directory is not unlike the shared `Sources` in the Swift Playground implementation, but this implementation uses ostensibly standalone entrypoints

## License

In keeping with Rust ecosystem conventions, this repository is dual-licensed under the Apache-2.0 license and the MIT license.
