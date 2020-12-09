use std::collections::HashMap;
use std::fs;

pub fn main() {
    println!("------------------------------------ DAY 6 ------------------------------------");

    let mut input = fs::read_to_string("input/day6.txt").unwrap();
    input = input.replace("\r", "");
    let input_split: Vec<&str> = input
        .split("\n\n")
        .filter(|n| n != &"")
        .collect();

    let mut part1 = 0;
    let mut part2 = 0;

    for i in &input_split {
        let answers = i.replace("\n", "");

        let mut answer_list: Vec<char> = answers.as_str().chars().collect();

        answer_list.sort();
        answer_list.dedup();

        part1 += answer_list.len();
    }

    for group in input_split {
        let mut answers : Vec<_> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

        for person in group.lines() {
            let personanswers : Vec<_> = person.chars().collect();

            answers.retain(|&x| personanswers.contains(&x));
        }

        part2 += answers.len();
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
