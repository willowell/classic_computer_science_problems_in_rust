/// # Fibonacci with simple memoization
///
/// This version expands on `fib2.rs` by using a `HashMap` to memoize intermediate values.
///
/// Java implementation: https://github.com/davecom/ClassicComputerScienceProblemsInJava/blob/master/CCSPiJ/src/chapter1/Fib3.java
///
use std::collections::HashMap;

use classic_computer_science_problems::timed;

fn fib_memo(x: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    let possible_cache_entry = cache.get(&x).map(|entry| entry.clone());

    match possible_cache_entry {
        Some(result) => result,
        None => {
            let result = match x {
                0 | 1 => x,
                n => fib_memo(n - 1, cache) + fib_memo(n - 2, cache),
            };

            cache.insert(x, result.clone());

            result
        }
    }
}

fn main() {
    timed!({
        let xs: Vec<u64> = (1..=40).map(|x| fib_memo(x, &mut HashMap::new())).collect();

        println!("Results of fib:");

        for (i, x) in xs.into_iter().enumerate() {
            println!("fib({}): {}", i + 1, x);
        }
    });
}
