# Classic Computer Science Problems in Rust

Rust port of David Kopec's "Classic Computer Science Problems" series, based on a fusion of his Java and Swift books.

Rather than being a plain port, I have chosen to lean into Rust's ecosystem and modify the programs to fit Rust's type system and idioms as applicable.

For instance, this port uses Nom to parse a string into a sequence of nucleotides in the "Compressed Gene" example in chapter 1.

## Examples

The examples are all under `/book`, organized into chapters.

Each example includes a short doc-comment overview with a link to the Java implementation.

For original examples such as `/book/chapter_01/fib6.rs`, there is obviously no link to a corresponding Java implementation.

Keep in mind that some examples are intentionally slow, like the naive fibonacci function implementation.

## How to run the examples

Much like the Java implementation, each example is its own program entry. However, Cargo allows us to use the `/src` directory as a common library, not unlike the shared `/Sources` directory in the Swift Playground version.

To run the examples, please follow these directions:

1. Check the lines with `[[bin]]` in `Cargo.toml`. Make a note of the name.
2. Open a terminal and run `cargo run --bin <NAME>`. For instance, `cargo run --bin fib1`.
3. Cargo will take a couple moments to collect dependencies, and then run the program.
4. That's it! If all goes well, you'll see the program's output in your terminal.

## License

In keeping with Rust ecosystem conventions, this repository is dual-licensed under the Apache-2.0 license and the MIT license.
