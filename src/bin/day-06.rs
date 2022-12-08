use std::time::Instant;

use anyhow::Result;

fn how_many_until_n_distinct(message: &str, len: usize) -> usize {
    message
        .as_bytes()
        .windows(len)
        .position(|x| {
            let num = x
                .into_iter()
                .fold(0usize, |sum, val| sum | 1 << (val - b'a'));
            usize::count_ones(num) == len as u32
        })
        .map(|x| x + len)
        .expect("No pattern found")
}

fn main() -> Result<()> {
    let start = Instant::now();
    let input = include_str!("input-6.prod");
    let part_1 = how_many_until_n_distinct(input, 4);
    let part_2 = how_many_until_n_distinct(input, 14);
    let duration = start.elapsed();
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
    println!("Duration: {:?}", duration);
    Ok(())
}
