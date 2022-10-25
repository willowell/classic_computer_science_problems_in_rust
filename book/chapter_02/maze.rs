/// # Maze Search
///
/// This example is especially interesting because it reveals some assumptions that the Java implementation makes and has
/// to account for them.
///
/// For instance, the Java implementation uses a `double` (`f64`) as the value for
/// its `java.util.PriorityQueue` (`std::collections::BinaryHeap, using `std::cmp::Reverse` to create a min-heap).
///
/// Rust does not allow using floats in a Binary Heap as floats are not totally ordered because `NaN != NaN`.
/// Instead, we can either use an integer type, or create a float newtype that excludes `NaN`, therefore allowing us
/// to implement `Ord`.
///
/// You will also notice some `if-let` statements and guards against `None`s. For instance, `pop`ping a value off of a container
/// returns an `Option` in Rust. Java's `Stack::pop()` instead throws an exception if the stack is empty - [docs for the `pop()` method here](https://docs.oracle.com/en/java/javase/19/docs/api/java.base/java/util/Stack.html#pop())
///
/// Since Rust encodes this case in the return type, we have to handle this accordingly. In a way, the return type is overly cautious - if we first check that the stack is not empty,
/// it is then safe to `pop()` a value off of it, and we can reason that the `None` variant is unreachable.
///
/// Java implementation: https://github.com/davecom/ClassicComputerScienceProblemsInJava/blob/master/CCSPiJ/src/chapter2/Maze.java
///
use classic_computer_science_problems::{
    generic_search::{astar, bfs, dfs},
    maze::{Maze, MazeLocation},
};
use ordered_float::OrderedFloat;

fn main() {
    let rows = 16;
    let columns = 16;
    let start = MazeLocation { row: 0, column: 0 };
    let goal = MazeLocation {
        row: 15,
        column: 15,
    };

    let mut maze = Maze::new_with_setup(rows, columns, start, goal, 0.3);

    /*
    Since we are working with a 2D grid of tiles, A* and BFS should return similar results.
    A* may use the same number of steps as BFS but take a more direct route or not go as far as BFS in one direction.

    DFS may even sometimes use as few steps as A* and BFS but take a slightly different route as well.
    But,
    */

    let solution = astar(
        maze.start,
        |loc| Maze::test_goal(&maze, loc),
        |loc| Maze::get_successors(&maze, loc),
        |loc| OrderedFloat(Maze::distance_to_goal(&maze, loc).into()),
    );

    println!("{}", maze);

    match solution {
        Some(solution) => {
            println!("Found solution with A*!");

            let path = solution.to_path();

            maze.mark_path(Vec::from(path.clone()));

            println!("Solution path:");
            println!("{}", maze);
            println!("Took {} steps", path.len());
        }
        None => {
            println!("No solution found :-(");
            println!("Perhaps the start or the goal are blocked off?");
        }
    }

    println!("{:=^width$}", "=", width = rows * 3);

    maze.clear_path();

    let solution = bfs(
        maze.start,
        |loc| Maze::test_goal(&maze, loc),
        |loc| Maze::get_successors(&maze, loc),
    );

    match solution {
        Some(solution) => {
            println!("Found solution with BFS!");

            let path = solution.to_path();

            maze.mark_path(Vec::from(path.clone()));

            println!("Solution path:");
            println!("{}", maze);
            println!("Took {} steps", path.len());
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
        |loc| Maze::test_goal(&maze, loc),
        |loc| Maze::get_successors(&maze, loc),
    );

    match solution {
        Some(solution) => {
            println!("Found solution with DFS!");

            let path = solution.to_path();

            maze.mark_path(Vec::from(path.clone()));

            println!("Solution path:");
            println!("{}", maze);
            println!("Took {} steps", path.len());
        }
        None => {
            println!("No solution found :-(");
            println!("Perhaps the start or the goal are blocked off?");
        }
    }
}
