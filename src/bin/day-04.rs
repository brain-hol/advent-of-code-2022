use std::{str::FromStr, time::Instant};

use anyhow::Result;

struct Task {
    start: usize,
    end: usize,
}

impl Task {
    pub fn contains(&self, other: &Task) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn contains_some(&self, other: &Task) -> bool {
        self.start <= other.start && self.end >= other.end
            || self.end >= other.end && self.start <= other.end
    }
}

impl FromStr for Task {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once("-").expect("Input is incorrect");
        Ok(Task {
            start: a.parse()?,
            end: b.parse()?,
        })
    }
}

struct Tasks {
    left: Task,
    right: Task,
}

impl Tasks {
    pub fn one_task_contains_the_other(&self) -> bool {
        self.left.contains(&self.right) || self.right.contains(&self.left)
    }

    pub fn one_task_contains_some_of_the_other(&self) -> bool {
        self.left.contains_some(&self.right) || self.right.contains_some(&self.left)
    }
}

impl FromStr for Tasks {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(",").expect("Input is incorrect");
        Ok(Tasks {
            right: right.parse()?,
            left: left.parse()?,
        })
    }
}

fn main() -> Result<()> {
    let start = Instant::now();
    let input = include_str!("input-4.prod");
    let part_1 = input
        .lines()
        .flat_map(|line| line.parse::<Tasks>())
        .filter(|tasks| tasks.one_task_contains_the_other())
        .count();
    let part_2 = input
        .lines()
        .flat_map(|line| line.parse::<Tasks>())
        .filter(|tasks| tasks.one_task_contains_some_of_the_other())
        .count();
    let duration = start.elapsed();
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
    println!("Duration: {:?}", duration);
    Ok(())
}
