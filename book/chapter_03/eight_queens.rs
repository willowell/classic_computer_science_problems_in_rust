/// # Eight Queens
///
/// While implementing this example, I discovered a silly bug with my implementation of the CSP class:
/// I underestimated the importance of this null check on the `result` of the backtracking search: https://github.com/davecom/ClassicComputerScienceProblemsInJava/blob/master/CCSPiJ/src/chapter3/CSP.java#L76-L78
///
/// Since my `backtracking_search()` returns an `Option`, I just returned `result` directly since it fit the type.
/// However, I was getting some weird results with this example - the backtracking search never went backwards!
/// By the time it got to the 6th queen, it could not come up with a solution.
///
/// Checking the Swift implementation, I realized this error because it uses an `if let` on the result: https://github.com/davecom/ClassicComputerScienceProblemsInSwift/blob/master/Classic%20Computer%20Science%20Problems%20in%20Swift.playground/Pages/Chapter%203.xcplaygroundpage/Contents.swift#L89-L91.
///
/// Somehow, this bug did not surface during the Australian map colouring problem!
///
/// Along the way, I went down the rabbit hole learning about Rust's BTreeMap, thinking maybe the unordered nature of HashMap was causing it.
/// Nope!
///
/// Other than that silly bug, this example is pretty straightforward.
///
/// Java implementation: https://github.com/davecom/ClassicComputerScienceProblemsInJava/blob/master/CCSPiJ/src/chapter3/QueensConstraint.java
use std::collections::HashMap;

use classic_computer_science_problems::csp::{Constraint, CSP};

use console::style;

#[derive(Clone, Debug)]
struct QueensConstraint {
    columns: Vec<i32>,
}

impl Constraint<i32, i32> for QueensConstraint {
    fn variables(&self) -> Vec<i32> {
        self.columns.clone()
    }

    fn is_satisfied(&self, assignment: &HashMap<i32, i32>) -> bool {
        for (&q1c, &q1r) in assignment {
            for q2c in (q1c + 1)..=(self.variables().len() as i32) {
                if let Some(&q2r) = assignment.get(&q2c) {
                    if q1r == q2r {
                        return false;
                    }

                    if (q1r - q2r).abs() == (q1c - q2c).abs() {
                        return false;
                    }
                }
            }
        }

        true
    }
}

fn display_board(board: HashMap<i32, i32>) {
    for row in 1..=8 {
        for col in 1..=8 {
            let qc = board.get_key_value(&col);

            print!(
                "{}{}{}",
                style("[").dim(),
                match qc {
                    Some((&c, &r)) => {
                        if c == col && r == row {
                            style("Q").yellow()
                        } else {
                            style(" ")
                        }
                    }
                    None => style(" "),
                },
                style("]").dim()
            );
        }
        println!();
    }
}

fn main() {
    let columns = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let mut rows = HashMap::<i32, Vec<i32>>::new();

    for &column in &columns {
        rows.insert(column, columns.clone());
    }

    let mut csp = CSP::<_, _, QueensConstraint>::new(columns.clone(), rows).unwrap();

    csp.add_constraint(QueensConstraint {
        columns: columns.clone(),
    })
    .unwrap();

    let solution = csp.backtracking_search();

    match solution {
        Some(solution) => {
            println!("Found solution:");
            display_board(solution);
        }
        None => println!("No solution found :-("),
    }
}
