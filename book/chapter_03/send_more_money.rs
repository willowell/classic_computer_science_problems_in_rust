/// # SEND+MORE=MONEY
/// 
/// Like the Java implementation, this version is pretty straightforward.
///
/// Java implementation: https://github.com/davecom/ClassicComputerScienceProblemsInJava/blob/master/CCSPiJ/src/chapter3/SendMoreMoneyConstraint.java
use std::collections::{HashSet, HashMap};

use classic_computer_science_problems::csp::{Constraint, CSP};
#[derive(Clone, Debug)]
struct SendMoreMoneyConstraint {
    letters: Vec<char>,
}

impl Constraint<char, i32> for SendMoreMoneyConstraint {
    fn variables(&self) -> Vec<char> {
        self.letters.clone()        
    }

    fn is_satisfied(&self, assignment: &std::collections::HashMap<char, i32>) -> bool {
        // If there are duplicate values, then it's not a solution.
        let assignment_as_set = HashSet::<&i32>::from_iter(assignment.values().clone());

        if assignment_as_set.len() < assignment.len() {
            return false;
        }

        // If all variables have been assigned, check if it adds correctly.
        if assignment.len() == self.letters.len() {
            let s = assignment.get(&'S').unwrap();
            let e = assignment.get(&'E').unwrap();
            let n = assignment.get(&'N').unwrap();
            let d = assignment.get(&'D').unwrap();
            let m = assignment.get(&'M').unwrap();
            let o = assignment.get(&'O').unwrap();
            let r = assignment.get(&'R').unwrap();
            let y = assignment.get(&'Y').unwrap();

            let send =              s * 1000 + e * 100 + n * 10 + d;
            let more =              m * 1000 + o * 100 + r * 10 + e;
            let money = m * 10000 + o * 1000 + n * 100 + e * 10 + y;

            return send + more == money;
        }  

        // No conflicts
        true
    }
}

fn main() {
    let letters = vec!['S', 'E', 'N', 'D', 'M', 'O', 'R', 'Y'];
    let mut possible_digits = HashMap::<char, Vec<i32>>::new();

    for letter in letters.clone() {
        possible_digits.insert(letter, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    // So we don't get answers starting with an O.
    possible_digits.insert('M', vec![1]);

    let mut csp: CSP<_, _, SendMoreMoneyConstraint> = CSP::new(letters.clone(), possible_digits).unwrap();

    csp.add_constraint(SendMoreMoneyConstraint { letters }).expect("valid constraint");

    let solution = csp.backtracking_search();

    match solution {
        Some(solution) => {
            println!("Found solution:");
            println!("{solution:?}");
        },
        None => println!("No solution found :-("),
    }
}
