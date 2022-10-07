/// # Fibonacci with infinite recursion
/// This fibonacci program will never terminate!
/// You've been warned. Rust's compiler doesn't like this program either!
/// 
/// Java implementation: https://github.com/davecom/ClassicComputerScienceProblemsInJava/blob/master/CCSPiJ/src/chapter1/Fib1.java
/// 

// Notice that rustc warns about the unconditional recursion in this function!
fn fib(x: u64) -> u64 {
    fib(x - 1) + fib(x - 2)
}

fn main() {
    println!("I will run forever!");

    let res = fib(20);

    println!("Result: {}", res);
}
