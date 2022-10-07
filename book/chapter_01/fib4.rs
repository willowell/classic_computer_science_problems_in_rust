/// # Iterative fibonacci
/// This fibonacci program uses an iterative approach rather than recursion.
/// 
/// Java implementation: https://github.com/davecom/ClassicComputerScienceProblemsInJava/blob/master/CCSPiJ/src/chapter1/Fib4.java
/// 

use std::time::Instant;

fn fib(x: u64) -> u64 {
    let mut last = 0;
    let mut next = 1;

    let mut i = 0;

    while i < x {
        let old_last = last;
        last = next;
        next = old_last + next;

        i += 1;
    }

    last
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
