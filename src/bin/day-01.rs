fn main() {
    let mut lines: Vec<u32> = include_str!("./input-1-1.txt")
        .split("\n\n")
        .map(|x| x.lines().flat_map(|num| num.parse::<u32>()).sum())
        .collect();
    lines.sort_by(|a, b| b.cmp(a));

    println!("Part 1: {:?}", lines[0]);
    println!("Part 2: {:?}", lines.into_iter().take(3).sum::<u32>());
}
