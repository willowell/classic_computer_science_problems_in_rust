/// # Maze Search using the `pathfinding` crate for the search algorithms.
///
/// Java implementation: https://github.com/davecom/ClassicComputerScienceProblemsInJava/blob/master/CCSPiJ/src/chapter2/Maze.java
///

use classic_computer_science_problems::maze::{Maze, MazeLocation};

use pathfinding::prelude::*;

fn main() {
    let rows = 16;
    let columns = 16;
    let start = MazeLocation { row: 0, column: 0 };
    let goal = MazeLocation {
        row: 15,
        column: 15,
    };

    let mut maze = Maze::new_with_setup(rows, columns, start, goal, 0.3);

    println!("{}", maze);

    let solution = astar(
        &maze.start,
        |loc| {
            Maze::get_successors(&maze, *loc)
                .into_iter()
                .map(|p| (p, 1))
                .collect::<Vec<(MazeLocation, u32)>>()
        },
        |loc| &maze.distance_to_goal(*loc) / 3,
        |loc| Maze::test_goal(&maze, *loc),
    );

    match solution {
        Some((solution, cost)) => {
            println!("Found solution with A*!");

            maze.mark_path(Vec::from(solution.clone()));

            println!("Solution path:");
            println!("{}", maze);
            println!("Took {} steps", solution.len());
            println!("Total cost: {}", cost);
        }
        None => {
            println!("No solution found :-(");
            println!("Perhaps the start or the goal are blocked off?");
        }
    }

    println!("{:=^width$}", "=", width = rows * 3);

    maze.clear_path();

    let solution = bfs(
        &maze.start,
        |loc| Maze::get_successors(&maze, *loc),
        |loc| Maze::test_goal(&maze, *loc),
    );

    match solution {
        Some(solution) => {
            println!("Found solution with BFS!");

            //let path = solution.to_path();

            maze.mark_path(Vec::from(solution.clone()));

            println!("Solution path:");
            println!("{}", maze);
            println!("Took {} steps", solution.len());
        }
        None => {
            println!("No solution found :-(");
            println!("Perhaps the start or the goal are blocked off?");
        }
    }

    println!("{:=^width$}", "=", width = rows * 3);

    maze.clear_path();

    let solution = dfs(
        maze.start,
        |loc| Maze::get_successors(&maze, *loc),
        |loc| Maze::test_goal(&maze, *loc),
    );

    match solution {
        Some(solution) => {
            println!("Found solution with DFS!");

            //let path = solution.to_path();

            // for step in &solution {
            //     println!("Step: ({}, {})", step.row, step.column);
            // }

            maze.mark_path(Vec::from(solution.clone()));

            println!("Solution path:");
            println!("{}", maze);
            println!("Took {} steps", solution.len());
        }
        None => {
            println!("No solution found :-(");
            println!("Perhaps the start or the goal are blocked off?");
        }
    }
}
