/// # Missionaries and Cannibals
/// 
/// Nothing really special going on with this implementation compared to the Java one.
/// 
/// Java implementation: https://github.com/davecom/ClassicComputerScienceProblemsInJava/blob/master/CCSPiJ/src/chapter2/MCState.java
use std::fmt;

use classic_computer_science_problems::generic_search::bfs;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct MCState {
    wm: i32,
    wc: i32,
    em: i32,
    ec: i32,
    is_boat_on_west_bank: bool,
}

const MAX_NUM: i32 = 3;

impl MCState {
    fn new(missionaries: i32, cannibals: i32, is_boat_on_west_bank: bool) -> Self {
        Self {
            wm: missionaries,
            wc: cannibals,
            em: MAX_NUM - missionaries,
            ec: MAX_NUM - cannibals,
            is_boat_on_west_bank,
        }
    }

    fn is_allowed_state(&self) -> bool {
        // If west bank cannibals outnumber west bank missionaries
        // AND there is at least one missionary left,
        // then that missionary is in danger.
        if self.wm < self.wc && self.wm > 0 {
            return false;
        }

        // Likewise for the east bank.
        if self.em < self.ec && self.em > 0 {
            return false;
        }

        true
    }

    fn test_goal(&self) -> bool {
        self.is_allowed_state()
        // All of the missionaries and cannibals are now on the east bank.
        // Side note: wouldn't it be best if they were on opposite sides of the river?
        && self.em == MAX_NUM && self.ec == MAX_NUM
    }

    fn get_successors(mcs: MCState) -> Vec<MCState> {
        let mut sucs: Vec<MCState> = Vec::new();

        if mcs.is_boat_on_west_bank {
            if mcs.wm > 1 {
                sucs.push(MCState::new(mcs.wm - 2, mcs.wc, !mcs.is_boat_on_west_bank));
            }
            if mcs.wm > 0 {
                sucs.push(MCState::new(mcs.wm - 1, mcs.wc, !mcs.is_boat_on_west_bank));
            }
            if mcs.wc > 1 {
                sucs.push(MCState::new(mcs.wm, mcs.wc - 2, !mcs.is_boat_on_west_bank));
            }
            if mcs.wc > 0 {
                sucs.push(MCState::new(mcs.wm, mcs.wc - 1, !mcs.is_boat_on_west_bank));
            }
            if mcs.wc > 0 && mcs.wm > 0 {
                sucs.push(MCState::new(
                    mcs.wm - 1,
                    mcs.wc - 1,
                    !mcs.is_boat_on_west_bank,
                ));
            }
        } else {
            if mcs.em > 1 {
                sucs.push(MCState::new(mcs.wm + 2, mcs.wc, !mcs.is_boat_on_west_bank));
            }
            if mcs.em > 0 {
                sucs.push(MCState::new(mcs.wm + 1, mcs.wc, !mcs.is_boat_on_west_bank));
            }
            if mcs.ec > 1 {
                sucs.push(MCState::new(mcs.wm, mcs.wc + 2, !mcs.is_boat_on_west_bank));
            }
            if mcs.ec > 0 {
                sucs.push(MCState::new(mcs.wm, mcs.wc + 1, !mcs.is_boat_on_west_bank));
            }
            if mcs.ec > 0 && mcs.em > 0 {
                sucs.push(MCState::new(
                    mcs.wm + 1,
                    mcs.wc + 1,
                    !mcs.is_boat_on_west_bank,
                ));
            }
        }

        // Remove invalid states. In other words, the states where the missionaries are in danger!
        sucs.retain(|s| MCState::is_allowed_state(s));

        sucs
    }

    fn display_solution(path: Vec<MCState>) {
        if path.len() == 0 {
            return;
        }

        let mut old_state = &path[0];

        println!("{old_state}");

        for current_state in path[1..].iter() {
            if current_state.is_boat_on_west_bank {
                println!(
                    "{} missionaries and {} cannibals moved from the east bank to the west bank.",
                    old_state.em - current_state.em,
                    old_state.ec - current_state.ec
                );
            } else {
                println!(
                    "{} missionaries and {} cannibals moved from the west bank to the east bank.",
                    old_state.wm - current_state.wm,
                    old_state.wc - current_state.wc
                );
            }

            println!("{current_state}");

            old_state = current_state;
        }
    }
}

impl fmt::Display for MCState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "On the west bank, there are {} missionaries and {} cannibals.",
            self.wm, self.wc
        )?;
        writeln!(
            f,
            "On the east bank, there are {} missionaries and {} cannibals.",
            self.em, self.ec
        )?;
        writeln!(
            f,
            "The boat is on the {} bank",
            if self.is_boat_on_west_bank {
                "west"
            } else {
                "east"
            }
        )
    }
}

fn main() {
    let start = MCState::new(MAX_NUM, MAX_NUM, true);

    let solution = bfs(
        start,
        |mcs| MCState::test_goal(&mcs),
        MCState::get_successors,
    );

    match solution {
        Some(solution) => {
            let path = solution.to_path();

            MCState::display_solution(path.clone().into());

            println!("Took {} steps.", path.len());
        }
        None => {
            println!("No solution found! :-(");
        }
    }
}
