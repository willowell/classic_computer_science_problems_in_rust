/// # Iterative fibonacci
///
/// This fibonacci program uses a lazy, infinite list of fibonacci numbers, which are generated iteratively.
///
/// Java implementation: https://github.com/davecom/ClassicComputerScienceProblemsInJava/blob/master/CCSPiJ/src/chapter1/Fib5.java
///
use std::iter;

use classic_computer_science_problems::timed;

fn main() {
    timed!({
        let mut last = 0;
        let mut next = 1;

        let fibs = iter::repeat_with(|| {
            let old_last = last;
            last = next;
            next = old_last + next;

            last
        })
        .take(40);

        println!("Results of fib:");

        for (i, x) in fibs.enumerate() {
            println!("fib({}): {}", i + 1, x);
        }
    });
}
