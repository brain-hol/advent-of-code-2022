use std::time::Instant;

use anyhow::Result;

fn to_index(c: &str) -> usize {
    return match c {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        _ => unreachable!("Unknown input"),
    };
}

fn parse_index_pair(s: &str) -> Result<(usize, usize)> {
    match s.split_once(' ') {
        Some((theirs, mine)) => Ok((to_index(theirs), to_index(mine))),
        None => Err(anyhow::anyhow!("Invalid input")),
    }
}

const POINTS: [[usize; 3]; 3] = [
    [1 + 3, 2 + 6, 3 + 0],
    [1 + 0, 2 + 3, 3 + 6],
    [1 + 6, 2 + 0, 3 + 3],
];

const POINTS_2: [[usize; 3]; 3] = [
    [3 + 0, 1 + 3, 2 + 6],
    [1 + 0, 2 + 3, 3 + 6],
    [2 + 0, 3 + 3, 1 + 6],
];

fn main() -> Result<()> {
    let start = Instant::now();
    let input = include_str!("input-2.prod");
    let mut score = 0;
    let mut score_2 = 0;
    for line in input.lines() {
        if let Ok((t, m)) = parse_index_pair(line) {
            score += POINTS[t][m];
            score_2 += POINTS_2[t][m];
        }
    }
    let duration = start.elapsed();
    println!("Part 1: {}", score);
    println!("Part 2: {}", score_2);
    println!("Time: {:?}", duration);
    Ok(())
}
