use std::fmt;

use console::{style, StyledObject};

use rand::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    Blocked,
    Start,
    Goal,
    Path,
}

impl Cell {
    fn to_character(&self) -> &str {
        match &self {
            Cell::Empty => " ",
            Cell::Blocked => "◼︎",
            Cell::Start => "▶︎",
            Cell::Goal => "★",
            Cell::Path => "●",
        }
    }

    fn to_styled_character(&self) -> StyledObject<&str> {
        match &self {
            Cell::Empty => style(" "),
            Cell::Blocked => style("◼︎").red(),
            Cell::Start => style("▶︎").green(),
            Cell::Goal => style("★").yellow(),
            Cell::Path => style("●").blue().bright(),
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Empty
    }
}

// Unlike the Java implementation, we can automatically derive `Hash` and `PartialEq`.
// No need to handwrite any `hashCode()` or `equals()` here.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MazeLocation {
    pub row: i32,
    pub column: i32,
}

impl MazeLocation {
    fn euclidean_distance_to(&self, dest: MazeLocation) -> f64 {
        let x_distance: i64 = (dest.column as i64) - (self.column as i64);
        let y_distance: i64 = (dest.row as i64) - (self.row as i64);

        let distance_squared = (x_distance * x_distance) + (y_distance * y_distance);

        (distance_squared as f64).sqrt()
    }

    fn manhattan_distance_to(&self, dest: MazeLocation) -> u32 {
        let x_distance = (dest.column - self.column).abs() as u32;
        let y_distance = (dest.row - self.row).abs() as u32;

        x_distance + y_distance
    }
}

#[derive(Clone, Debug, Default)]
pub struct Maze {
    pub rows: usize,
    pub columns: usize,
    pub start: MazeLocation,
    pub goal: MazeLocation,
    grid: Vec<Vec<Cell>>,
}

impl Maze {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_with_with_and_height(rows: usize, columns: usize) -> Self {
        let grid: Vec<Vec<Cell>> = vec![vec![Cell::Empty; columns]; rows];

        Self {
            rows,
            columns,
            grid,
            ..Default::default()
        }
    }

    pub fn new_with_setup(
        rows: usize,
        columns: usize,
        start: MazeLocation,
        goal: MazeLocation,
        sparseness: f64,
    ) -> Self {
        let mut maze = Self::new_with_with_and_height(rows, columns);

        maze.randomly_block_cells(sparseness);

        maze.grid[start.row as usize][start.column as usize] = Cell::Start;
        maze.grid[goal.row as usize][goal.column as usize] = Cell::Goal;

        Self {
            start,
            goal,
            ..maze
        }
    }

    /// Randomly block cells in this maze, given a threshold sparseness.
    /// This is the `randomly_fill()` method in the Java implementation.
    fn randomly_block_cells(&mut self, sparseness: f64) {
        let mut rng = thread_rng();

        for row in &mut self.grid {
            for col in row {
                if rng.gen::<f64>() < sparseness {
                    *col = Cell::Blocked;
                }
            }
        }
    }

    pub fn test_goal(&self, loc: MazeLocation) -> bool {
        self.goal == loc
    }

    fn is_location_valid(&self, loc: &MazeLocation) -> bool {
        loc.row >= 0
            && loc.row < self.rows as i32
            && loc.column >= 0
            && loc.column < self.columns as i32
    }

    pub fn get_successors(&self, from_loc: MazeLocation) -> Vec<MazeLocation> {
        let mut successors = Vec::new();

        for dx in -1i32..=1 {
            for dy in -1i32..=1 {
                if (dx + dy).abs() != 1 {
                    continue;
                }

                let new_position = MazeLocation {
                    row: from_loc.row + dx,
                    column: from_loc.column + dy,
                };

                //println!("Testing location {:?}", new_position);

                if !&self.is_location_valid(&new_position) {
                    //println!("Location {:?} is not valid!", new_position);
                    continue;
                }

                let maze_value = self.grid[new_position.row as usize][new_position.column as usize];

                if maze_value != Cell::Blocked {
                    successors.push(new_position);
                }
            }
        }

        //println!("Successors: {:?}", successors);

        successors
    }

    pub fn mark_start_and_goal(&mut self) {
        self.grid[self.start.row as usize][self.start.column as usize] = Cell::Start;
        self.grid[self.goal.row as usize][self.goal.column as usize] = Cell::Goal;
    }

    pub fn mark_path(&mut self, path: Vec<MazeLocation>) {
        for loc in path {
            self.grid[loc.row as usize][loc.column as usize] = Cell::Path;
        }
        self.mark_start_and_goal();
    }

    pub fn clear_path(&mut self) {
        for row in &mut self.grid {
            for col in row {
                if *col == Cell::Path {
                    *col = Cell::Empty;
                }
            }
        }
        self.mark_start_and_goal();
    }

    pub fn distance_to_goal(&self, loc: MazeLocation) -> u32 {
        self.goal.manhattan_distance_to(loc)
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            for col in row {
                write!(
                    f,
                    "{}{}{}",
                    style("[").dim(),
                    col.to_styled_character(),
                    style("]").dim()
                )?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
