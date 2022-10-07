/// # Naive fibonacci
/// This fibonacci program terminates but is very slow (more than 1000 ms)
/// since it doesn't keep repeated intermediate values.
/// 
/// For fun, this program demonstrates two different fibonacci implementations:
/// * with `match`
/// * with `if`.
/// 
/// Additionally, starting with this program, I use `std::time::Instant` to time execution.
/// 
/// Java implementation: https://github.com/davecom/ClassicComputerScienceProblemsInJava/blob/master/CCSPiJ/src/chapter1/Fib2.java
/// 

use std::time::Instant;

fn fib(x: u64) -> u64 {
    match x {
        0 | 1 => x,
        _     => fib(x - 2) + fib(x - 1),
    }
}

fn fib_ifelse(x: u64) -> u64 {
    if x < 2 {
        x
    } else {
        fib_ifelse(x - 2) + fib_ifelse(x - 1)
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

    let start = Instant::now();

    let xs: Vec<u64> = (1..=40)
        .map(|x| fib_ifelse(x))
        .collect();

    println!("Results of fib, using if statements:");

    for (i, x) in xs.into_iter().enumerate() {
        println!("fib({}): {}", i + 1, x);
    }

    println!("Took {} ms.", start.elapsed().as_millis());
}
