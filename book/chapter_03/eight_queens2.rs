/// # Eight Queens, using the `libpcp` crate.
/// 
/// This example is straight from libpcp's own examples!
/// You can find it here: https://github.com/ptal/pcp/blob/master/example/src/nqueens.rs
/// 
/// To be clear, a backtracking search is not the only way to solve a constraint-satisfaction problem.
/// I chose libpcp to demonstrate another approach; libpcp uses a more sophiscated approach and can therefore
/// provide a different perspective and act as a touchstone for people interested in exploring this topic further.
/// 
/// Interestingly, this implementation produces a different but still valid solution compared to the backtracking search solution.
/// 
/// You might also be interested in learning about [Prolog, a logic programming language and environment](https://www.swi-prolog.org/).
/// In a way, everything in Prolog is a CSP. You tell Prolog some facts and rules about these facts, and then ask it a question based on this information.
/// In fact, here is a cool paper I found by Markus Triska, a researcher: https://www.metalevel.at/queens/.
/// Their website looks to be a great place to learn more about Prolog

use console::style;

use pcp::kernel::*;
use pcp::propagators::*;
use pcp::variable::ops::*;
use pcp::term::*;
use pcp::search::search_tree_visitor::Status::*;
use pcp::search::*;
use pcp::concept::*;
use interval::ops::Range;
use interval::interval_set::*;
use gcollections::ops::*;

fn display_board(board: Vec<i32>) {
    for row in 1..=8 {
        for col in 1..=8 {
            let qc = board.get(col);

            print!(
                "{}{}{}",
                style("[").dim(),
                match qc {
                    Some(&r) => {
                        if r == row as i32 {
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

pub fn nqueens(n: usize) {
    let mut space = FDSpace::empty();
  
    let mut queens = vec![];
    // 2 queens can't share the same line.
    for _ in 0..n {
      queens.push(Box::new(space.vstore.alloc(IntervalSet::new(1, n as i32))) as Var<VStore>);
    }
    for i in 0..n-1 {
      for j in i + 1..n {
        // 2 queens can't share the same diagonal.
        let q1 = (i + 1) as i32;
        let q2 = (j + 1) as i32;
        // Xi + i != Xj + j reformulated as: Xi != Xj + j - i
        space.cstore.alloc(Box::new(XNeqY::new(
          queens[i].bclone(), Box::new(Addition::new(queens[j].bclone(), q2 - q1)) as Var<VStore>)));
        // Xi - i != Xj - j reformulated as: Xi != Xj - j + i
        space.cstore.alloc(Box::new(XNeqY::new(
          queens[i].bclone(), Box::new(Addition::new(queens[j].bclone(), -q2 + q1)) as Var<VStore>)));
      }
    }
    // 2 queens can't share the same column.
    join_distinct(&mut space.vstore, &mut space.cstore, queens);
    // space.cstore.alloc(Box::new(Distinct::new(queens)));
  
    // Search step.
    let mut search = one_solution_engine();
    search.start(&space);
    let (frozen_space, status) = search.enter(space);
    let space = frozen_space.unfreeze();
  
    // Print result.
    match status {
      Satisfiable => {
        print!("{}-queens problem is satisfiable. The first solution is:\n[", n);
        for dom in space.vstore.iter() {
          // At this stage, dom.lower() == dom.upper().
          print!("{}, ", dom.lower());
        }
        println!("]");

        display_board(Vec::from_iter(space.vstore.iter().map(|v| v.lower())));
      }
      Unsatisfiable => println!("{}-queens problem is unsatisfiable.", n),
      EndOfSearch => println!("Search terminated or was interrupted."),
      Unknown(_) => unreachable!(
        "After the search step, the problem instance should be either satisfiable or unsatisfiable.")
    }
  }

fn main() {
    nqueens(8);
}
