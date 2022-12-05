use std::time::Instant;

struct Item {
    value: usize,
}

impl From<&u8> for Item {
    fn from(value: &u8) -> Self {
        let value = if *value > b'a' {
            *value as u8 - b'a' + 1
        } else {
            *value as u8 - b'A' + 27
        };
        Item {
            value: value as usize,
        }
    }
}

fn main() {
    let start = Instant::now();
    let lines = include_str!("input-3.prod").lines().collect();
    let answer_1 = part_1(&lines);
    let answer_2 = part_2(&lines);
    let duration = start.elapsed();
    println!("Part 1: {}", answer_1);
    println!("Part 2: {}", answer_2);
    println!("Duration: {:?}", duration);
}

fn part_1(input: &Vec<&str>) -> usize {
    let mut sum = 0usize;

    for values in input {
        let (first, second) = values.as_bytes().split_at(values.len() / 2);

        // Fill occurrences array with the first half to allow O(1) lookups for
        // the second group
        let mut occurrences: [bool; 53] = [false; 53];
        for x in first {
            let item = Item::from(x);
            occurrences[item.value] = true;
        }

        for x in second {
            let item = Item::from(x);
            if occurrences[item.value] {
                sum += item.value;
                break;
            }
        }
    }
    sum
}

fn part_2(input: &Vec<&str>) -> usize {
    input.chunks(3).map(check_group).sum()
}

fn check_group(chunk: &[&str]) -> usize {
    let mut occurrences: [u8; 53] = [0; 53];
    let mut sum = 0;

    for (row_index, rucksack) in chunk.into_iter().enumerate() {
        for x in rucksack.as_bytes() {
            let item = Item::from(x);

            occurrences[item.value] |= 1 << row_index;
            if occurrences[item.value] == 0b111 {
                sum += item.value;
                break;
            }
        }
    }
    sum
}
