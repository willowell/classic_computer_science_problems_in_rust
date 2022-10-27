/// # The Australian Map Colouring Constraint Problem
///
/// This example is a bit rough around the edges...
/// Since the Java implementation makes use of generic abstract classes and type elision, we have to address both of these
/// issues along the way. Rather than replace the stateful generic abstract class with a struct and trait combo, I have opted to
/// instead go with just a trait, whose `variables()` function mimics the `variables` property on the corresponding Java
/// abstract class.
///
/// Methods in the Java implementation that throw exceptions return `Result`s in this implementation.
///
/// Aside from this type tetris, this example is not terribly complicated.
///
/// Java implementation: https://github.com/davecom/ClassicComputerScienceProblemsInJava/blob/master/CCSPiJ/src/chapter3/MapColoringConstraint.java
use std::collections::HashMap;

use classic_computer_science_problems::csp::{Constraint, CSP};

use console::{style, StyledObject};

#[derive(Clone, Copy, Debug)]
struct MapColouringConstraint {
    place_1: &'static str,
    place_2: &'static str,
}

impl MapColouringConstraint {
    fn new(place_1: &'static str, place_2: &'static str) -> Self {
        Self { place_1, place_2 }
    }
}

impl Constraint<&'static str, &'static str> for MapColouringConstraint {
    fn variables(&self) -> Vec<&'static str> {
        vec![self.place_1, self.place_2]
    }

    fn is_satisfied(
        &self,
        assignment: &std::collections::HashMap<&'static str, &'static str>,
    ) -> bool {
        if !assignment.contains_key(&self.place_1) || !assignment.contains_key(&self.place_2) {
            return true;
        }

        let possible_place_1 = assignment.get(&self.place_1);
        let possible_place_2 = assignment.get(&self.place_2);

        if let (Some(place_1), Some(place_2)) = (possible_place_1, possible_place_2) {
            return !(place_1 == place_2);
        }

        false
    }
}

fn pretty_print_colour(colour: &'static str) -> StyledObject<&'static str> {
    match colour {
        "green" => style("green").green(),
        "blue" => style("blue").blue(),
        "red" => style("red").red(),
        _ => unreachable!(),
    }
}

fn main() {
    let variables = vec![
        "Western Australia",
        "Northern Territory",
        "South Australia",
        "Queensland",
        "New South Wales",
        "Victoria",
        "Tasmania",
        // These two can act as good sanity checks.
        // ACT is inside NSW, so any other colour will do.
        // Likewise, JBT is on the coastline surrounded by NSW, so any other colour will do.
        "Australian Capital Territory", // enclave inside New South Wales
        "Jervis Bay Territory",         // territory on the coast of New South Wales
    ];

    let mut domains = HashMap::new();

    for variable in variables.clone() {
        domains.insert(variable, vec!["red", "green", "blue"]);
    }

    let mut csp: CSP<&str, &str, MapColouringConstraint> = CSP::new(variables, domains).unwrap();

    let constraints = vec![
        MapColouringConstraint::new("Western Australia", "Northern Territory"),
        MapColouringConstraint::new("Western Australia", "South Australia"),
        MapColouringConstraint::new("South Australia", "Northern Territory"),
        MapColouringConstraint::new("Queensland", "Northern Territory"),
        MapColouringConstraint::new("Queensland", "South Australia"),
        MapColouringConstraint::new("Queensland", "New South Wales"),
        MapColouringConstraint::new("New South Wales", "South Australia"),
        MapColouringConstraint::new("Victoria", "South Australia"),
        MapColouringConstraint::new("Victoria", "New South Wales"),
        MapColouringConstraint::new("Victoria", "Tasmania"),
        MapColouringConstraint::new("Australian Capital Territory", "New South Wales"),
        MapColouringConstraint::new("Jervis Bay Territory", "New South Wales"),
    ];

    for constraint in constraints {
        // println!("Adding constraint: {} -> {}", constraint.place_1, constraint.place_2);
        csp.add_constraint(constraint).expect("Valid constraint");
    }

    // println!("Variables:");
    // println!("{:?}", csp.variables);
    // println!("Domains:");
    // println!("{:?}", csp.domains);
    // println!("Constraints:");
    // println!("{:?}", csp.constraints);

    let solution = csp.backtracking_search();

    match solution {
        Some(solution) => {
            println!("Found solution:");

            for (k, v) in solution.iter() {
                println!("{} => {}", k, pretty_print_colour(v));
            }
        }
        None => println!("No solution found :-("),
    }
}
