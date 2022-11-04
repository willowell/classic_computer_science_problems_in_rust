use classic_computer_science_problems::csp::{Constraint, CSP};
use console::style;
/// # Word Search
///
/// Java implementation:
/// 
use std::{fmt, collections::{HashSet, HashMap}};

use rand::{prelude::*, distributions::Uniform};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct GridLocation {
    row: i32,
    column: i32,
}

#[derive(Clone, Debug, Default)]
struct WordGrid {
    rows: usize,
    columns: usize,
    grid: Vec<Vec<char>>,
}

impl WordGrid {
    const ALPHABET_LENGTH: i32 = 26;
    const FIRST_LETTER: char = 'A';

    pub fn new() -> Self {
        Default::default()
    }



    fn new_with_with_and_height(rows: usize, columns: usize) -> Self {
        let grid: Vec<Vec<char>> = vec![vec![Self::FIRST_LETTER; columns]; rows];

        Self {
            rows,
            columns,
            grid,
            ..Default::default()
        }
    }

    fn new_with_setup(
        rows: usize,
        columns: usize,
    ) -> Self {
        let mut maze = Self::new_with_with_and_height(rows, columns);

        maze.randomly_fill_cells();

        maze
    }

    /// Fill cells with randomly selected letters.
    /// Kopec does this inline in `WordGrid`'s constructor, but I have chosen
    /// to do this in a separate function.
    /// This is the `randomly_fill()` method in the Java implementation.
    fn randomly_fill_cells(&mut self) {
        let mut rng = thread_rng();

        let alpha = Uniform::from('A'..='Z');

        for row in &mut self.grid {
            for col in row {
                *col = rng.sample(alpha) as char;
            }
        }
    }

    /// Mark a word in this word grid.
    fn mark_word(&mut self, word: &str, locations: Vec<GridLocation>) {
        let mut i = 0;

        while i < word.len() {
            let location = locations.get(i);

            if let Some(location) = location {
                self.grid[location.row as usize][location.column as usize] = word.chars().nth(i).unwrap();
            }

            i += 1;
        }
    }

    fn generate_domain(&self, word: String) -> Vec<Vec<GridLocation>> {
        let mut domain = Vec::new();
        let word_length = word.len();

        for row in 0..self.rows {
            for column in 0..self.columns {
                if column + word_length <= self.columns {
                    if let Some(right_locations) = Self::get_right_locations(row, column, word_length) {
                        domain.push(right_locations);
                    }

                    if row + word_length <= self.rows {
                        if let Some(diagonal_right_locations) = Self::get_diagonal_right_locations(row, column, word_length) {
                            domain.push(diagonal_right_locations);
                        }
                    }
                }

                if row + word_length <= self.rows {
                    if let Some(down_locations) = Self::get_down_locations(row, column, word_length) {
                        domain.push(down_locations);
                    }

                    if (column as i32 - word_length as i32) >= 0 {
                        if let Some(diagonal_left_locations) = Self::get_diagonal_left_locations(row, column, word_length) {
                            domain.push(diagonal_left_locations);
                        }
                    }

                }
            }
        }

        domain
    }

    fn get_right_locations(row: usize, column: usize, word_length: usize) -> Option<Vec<GridLocation>> {
        let mut locations = Vec::<GridLocation>::new();

        let mut c = column;

        while c < column + word_length {
            locations.push(GridLocation { row: row as i32, column: c as i32 });

            c += 1;
        }

        if !locations.is_empty() {
            Some(locations)
        } else {
            None
        }
    }

    fn get_diagonal_right_locations(row: usize, column: usize, word_length: usize) -> Option<Vec<GridLocation>> {
        let mut locations = Vec::<GridLocation>::new();

        let mut c = column;
        let mut r = row;

        while c < column + word_length {
            locations.push(GridLocation { row: r as i32, column: c as i32 });

            r += 1;
            c += 1;
        }

        if !locations.is_empty() {
            Some(locations)
        } else {
            None
        }
    }

    fn get_down_locations(row: usize, column: usize, word_length: usize) -> Option<Vec<GridLocation>> {
        let mut locations = Vec::<GridLocation>::new();

        let mut r = row;

        while r < row + word_length {
            locations.push(GridLocation { row: r as i32, column: column as i32 });

            r += 1;
        }

        if !locations.is_empty() {
            Some(locations)
        } else {
            None
        }
    }

    fn get_diagonal_left_locations(row: usize, column: usize, word_length: usize) -> Option<Vec<GridLocation>> {
        let mut locations = Vec::<GridLocation>::new();

        let mut c = column;
        let mut r = row;

        while r < row + word_length {
            locations.push(GridLocation { row: r as i32, column: c as i32 });

            r += 1;
            c -= 1;
        }

        if !locations.is_empty() {
            Some(locations)
        } else {
            None
        }
    }
}

impl fmt::Display for WordGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            for col in row {
                write!(
                    f,
                    "{}{}{}",
                    style("[").dim(),
                    col,
                    style("]").dim()
                )?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
struct WordSearchConstraint {
    words: Vec<String>,
}

impl Constraint<String, Vec<GridLocation>> for WordSearchConstraint {
    fn variables(&self) -> Vec<String> {
        self.words.clone()
    }

    fn is_satisfied(&self, assignment: &std::collections::HashMap<String, Vec<GridLocation>>) -> bool {
        // Combine all grid locations into one giant list
        let all_locations: Vec<&GridLocation> = assignment
            .values()
            .flatten()
            .collect();

        let all_locations_set: HashSet<&GridLocation> = HashSet::from_iter(all_locations.iter().cloned());

        all_locations.len() == all_locations_set.len()
    }
}

fn main() {
    let mut wordgrid = WordGrid::new_with_setup(10, 10);

    println!("{wordgrid}");

    let words: Vec<String> = vec![
        "MATTHEW".to_string(),
        "JOE".to_string(),
        "MARY".to_string(),
        "SARAH".to_string(),
        "SALLY".to_string()
    ];

    let mut domains: HashMap<String, Vec<Vec<GridLocation>>> = HashMap::new();

    for word in words.clone() {
        domains.insert(word.clone(), wordgrid.generate_domain(word.clone()));
    }

    let mut csp: CSP<_, _, WordSearchConstraint> = CSP::new(words.clone(), domains).unwrap();

    csp.add_constraint(WordSearchConstraint { words }).expect("valid constraint");

    let solution = csp.backtracking_search();

    match solution {
        Some(mut solution) => {
            println!("Found solution:");

            let mut rng = thread_rng();

            for (k, v) in solution.iter_mut() {
                if rng.gen::<bool>() {
                    v.reverse();
                }

                wordgrid.mark_word(k, v.to_vec());
            }

            println!("{wordgrid}");
        },
        None => println!("No solution found :-("),
    }

    
}
