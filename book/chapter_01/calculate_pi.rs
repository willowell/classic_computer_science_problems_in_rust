use std::f64::consts::PI;

use classic_computer_science_problems::timed;

/// # Calculate pi
///
/// Java implementation: https://github.com/davecom/ClassicComputerScienceProblemsInJava/blob/master/CCSPiJ/src/chapter1/PiCalculator.java

fn calculate_pi(num_terms: u64) -> f64 {
    let numerator = 4.0;

    let mut denominator = 1.0;
    let mut operation = 1.0;
    let mut pi = 0.0;

    for _ in 0..num_terms {
        pi += operation * (numerator / denominator);
        denominator += 2.0;
        operation *= -1.0;
    }

    pi
}

fn main() {
    println!("Let's make some pi!");

    let powers_of_ten = std::iter::successors(Some(1_u32), |n| n.checked_mul(10));

    println!("Numbers are ready! Time to bake some pi!");

    timed!({
        let xs: Vec<f64> = powers_of_ten
            .clone()
            .map(|n| -> f64 { calculate_pi(n as u64) })
            .collect();

        let result = xs.iter().zip(powers_of_ten.clone().into_iter());

        for (i, (x, y)) in result.enumerate() {
            println!(
                "{:<10} iterations, result: {:>20.2$}, difference: {3:>20.2$}%",
                y,
                x,
                i + 1,
                x / PI
            );
        }
    });

    println!("Your pi is ready! 😋");
}
