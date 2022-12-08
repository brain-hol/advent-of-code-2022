use std::{str::FromStr, time::Instant};

use anyhow::Result;

#[derive(Debug, Clone)]
struct Crane {
    stacks: Vec<Vec<char>>,
}

impl Crane {
    fn apply_action(&mut self, action: &CraneAction) {
        for _ in 0..action.count {
            let item = self.stacks[action.from].pop().expect("Must exist");
            self.stacks[action.to].push(item);
        }
    }

    fn apply_part_2_action(&mut self, action: &CraneAction) {
        let temp_stack = (0..action.count)
            .filter_map(|_| self.stacks[action.from].pop())
            .collect::<Vec<_>>();
        for item in temp_stack.into_iter().rev() {
            self.stacks[action.to].push(item);
        }
    }

    fn get_top_letters(&self) -> String {
        self.stacks
            .iter()
            .filter_map(|stack| stack.last())
            .map(|letter| *letter)
            .collect()
    }
}

impl FromStr for Crane {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stacks = Vec::new();
        s.lines().rev().skip(1).for_each(|line| {
            line.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, letter)| *letter != ' ')
                .for_each(|(i, letter)| {
                    if stacks.len() < i + 1 {
                        stacks.push(Vec::new())
                    }
                    stacks[i].push(letter)
                })
        });
        Ok(Crane { stacks })
    }
}

struct CraneAction {
    count: usize,
    from: usize,
    to: usize,
}

impl FromIterator<usize> for CraneAction {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        return CraneAction {
            count: iter.next().expect("Must exist"),
            from: iter.next().expect("Must exist") - 1,
            to: iter.next().expect("Must exist") - 1,
        };
    }
}

impl FromStr for CraneAction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .collect::<CraneAction>())
    }
}

fn main() -> Result<()> {
    let start = Instant::now();
    let input = include_str!("input-5.prod");
    let (crane, actions) = input.split_once("\n\n").expect("Aoc input will not break");
    let mut crane = crane.parse::<Crane>().expect("Must exist");
    let mut crane_2 = crane.clone();
    actions
        .lines()
        .filter_map(|x| x.parse::<CraneAction>().ok())
        .for_each(|action| {
            crane.apply_action(&action);
            crane_2.apply_part_2_action(&action);
        });
    let part_1 = crane.get_top_letters();
    let part_2 = crane_2.get_top_letters();
    let duration = start.elapsed();
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
    println!("Duration: {:?}", duration);
    Ok(())
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    #[test]
    fn test_crane_parse() -> Result<()> {
        let str = include_str!("input-5.test")
            .split("\n\n")
            .collect::<Vec<_>>()[0];
        let crane = str.parse::<super::Crane>()?;
        assert_eq!(crane.get_top_letters(), "NDP");
        Ok(())
    }
}
