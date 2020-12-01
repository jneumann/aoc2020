use std::fs;

pub fn main() {
    println!("------------------------------------ DAY 1 ------------------------------------");

    let input = fs::read_to_string("input/day1.txt").unwrap();
    let input_split: Vec<i32> = input
        .split("\n")
        .filter(|n| n != &"")
        .map(|n| n.parse().unwrap())
        .collect();

    let mut part1: bool = false;
    let mut part2: bool = false;

    for s in &input_split {
        let num_to_find = 2020 - s;

        if input_split.contains(&num_to_find) && !part1 {
            println!("Part 1: {}", s * num_to_find);
            part1 = true;
        }
    }

    for x in &input_split {
        for y in &input_split {
            for z in &input_split {
                if (x + y + z) == 2020 && !part2 {
                    println!("Part 2: {}", x * y * z);
                    part2 = true;
                }
            }
        }
    }
}
