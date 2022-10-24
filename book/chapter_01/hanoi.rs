/// # Classic Towers of Hanoi with 3 pegs.
///
/// Java implementation: https://github.com/davecom/ClassicComputerScienceProblemsInJava/blob/master/CCSPiJ/src/chapter1/Hanoi.java
///
/// Note that the Java implementation uses [java.util.Stack](https://docs.oracle.com/en/java/javase/19/docs/api/java.base/java/util/Stack.html).
/// We can just use `std::collections::VecDeque` instead.
///
use std::{collections::VecDeque, fmt::Display};

struct Hanoi {
    num_discs: u64,

    tower_a: VecDeque<u64>,
    tower_b: VecDeque<u64>,
    tower_c: VecDeque<u64>,
}

impl Hanoi {
    fn new(num_discs: u64) -> Self {
        let mut tower_a = VecDeque::new();

        for n in 1..=num_discs {
            tower_a.push_front(n);
        }

        Self {
            num_discs,
            tower_a,
            tower_b: VecDeque::with_capacity(num_discs as usize),
            tower_c: VecDeque::with_capacity(num_discs as usize),
        }
    }

    fn move_discs(
        begin: &mut VecDeque<u64>,
        end: &mut VecDeque<u64>,
        temp: &mut VecDeque<u64>,
        n: u64,
    ) {
        if n == 1 {
            if let Some(begin_front) = begin.pop_front() {
                end.push_front(begin_front);
            }
        } else {
            Self::move_discs(begin, temp, end, n - 1);
            Self::move_discs(begin, end, temp, 1);
            Self::move_discs(temp, end, begin, n - 1);
        }
    }

    fn solve(&mut self) {
        Self::move_discs(
            &mut self.tower_a,
            &mut self.tower_c,
            &mut self.tower_b,
            self.num_discs,
        )
    }
}

impl Display for Hanoi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "============Towers of Hanoi===========")?;
        writeln!(f, "======================================")?;
        writeln!(f, "Higher numbers represent smaller discs")?;

        writeln!(f, "===============Tower A================")?;
        if !self.tower_a.is_empty() {
            self.tower_a.iter().enumerate().try_for_each(|(i, disc)| {
                writeln!(
                    f,
                    "{}[{}{}{}]{}",
                    " ".repeat(self.num_discs as usize - i - 1),
                    " ".repeat((self.num_discs - disc) as usize),
                    disc,
                    " ".repeat((self.num_discs - disc) as usize),
                    " ".repeat(self.num_discs as usize - i - 1),
                )
            })?;
        } else {
            (1..=self.num_discs).try_for_each(|_| {
                writeln!(
                    f,
                    "{}[ ]{}",
                    " ".repeat(self.num_discs as usize - 1),
                    " ".repeat(self.num_discs as usize - 1)
                )
            })?;
        }

        writeln!(f, "===============Tower B================")?;
        if !self.tower_b.is_empty() {
            self.tower_b.iter().enumerate().try_for_each(|(i, disc)| {
                writeln!(
                    f,
                    "{}[{}{}{}]{}",
                    " ".repeat(self.num_discs as usize - i - 1),
                    " ".repeat((self.num_discs - disc) as usize),
                    disc,
                    " ".repeat((self.num_discs - disc) as usize),
                    " ".repeat(self.num_discs as usize - i - 1),
                )
            })?;
        } else {
            (1..=self.num_discs).try_for_each(|_| {
                writeln!(
                    f,
                    "{}[ ]{}",
                    " ".repeat(self.num_discs as usize - 1),
                    " ".repeat(self.num_discs as usize - 1)
                )
            })?;
        }

        writeln!(f, "===============Tower C================")?;
        if !self.tower_c.is_empty() {
            self.tower_c.iter().enumerate().try_for_each(|(i, disc)| {
                writeln!(
                    f,
                    "{}[{}{}{}]{}",
                    " ".repeat(self.num_discs as usize - i - 1),
                    " ".repeat((self.num_discs - disc) as usize),
                    disc,
                    " ".repeat((self.num_discs - disc) as usize),
                    " ".repeat(self.num_discs as usize - i - 1),
                )
            })?;
        } else {
            (1..=self.num_discs).try_for_each(|_| {
                writeln!(
                    f,
                    "{}[ ]{}",
                    " ".repeat(self.num_discs as usize - 1),
                    " ".repeat(self.num_discs as usize - 1)
                )
            })?;
        }

        Ok(())
    }
}

fn main() {
    let mut hanoi = Hanoi::new(5);

    println!("{}", hanoi);

    hanoi.solve();

    println!("{}", hanoi);
}
