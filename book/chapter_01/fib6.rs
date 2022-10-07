/// # Fibonacci with `#[memoize]` macro
/// This fibonacci program uses the `#[memoize]` macro from the `memoize` crate.

use std::time::Instant;

use memoize::memoize;

#[memoize]
fn fib(x: u64) -> u64 {
    match x {
        0 | 1 => x,
        _     => fib(x - 2) + fib(x - 1),
    }
}

fn main() {
    let start = Instant::now();

    let xs: Vec<u64> = (1..=40)
        .map(|x| fib(x))
        .collect();

    println!("Results of fib, using match:");

    for (i, x) in xs.into_iter().enumerate() {
        println!("fib({}): {}", i + 1, x);
    }

    println!("Took {} ms.", start.elapsed().as_millis());
}
