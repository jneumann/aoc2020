use std::fs;

pub fn first(input: String) -> String {
    transform(&input).max().unwrap().to_string()
}

pub fn second(input: String) -> String {
    let mut boarding_passes: Vec<u16> = transform(&input).collect::<Vec<u16>>();
    boarding_passes.sort();

    let (prior_seat, _) = boarding_passes
        .iter()
        .zip(boarding_passes.iter().skip(1))
        .find(|(b0, b1)| *b1 - *b0 > 1)
        .unwrap();

    (prior_seat + 1).to_string()
}

fn transform(input: &str) -> impl Iterator<Item = u16> + '_ {
    input.lines().map(|line| seat_id(line))
}

fn seat_id(input: &str) -> u16 {
    row_num(input) * 8 + col_num(input)
}

fn row_num(input: &str) -> u16 {
    let binary: String = input[..=6].replace("F", "0").replace("B", "1");
    u16::from_str_radix(&binary, 2).unwrap()
}

fn col_num(input: &str) -> u16 {
    let binary: String = input[7..].replace("L", "0").replace("R", "1");
    u16::from_str_radix(&binary, 2).unwrap()
}

pub fn main() {
    println!("------------------------------------ DAY 5 ------------------------------------");

    let input = fs::read_to_string("input/day5.txt").unwrap();

    println!("Part 1: {}", first(input.clone()));
    println!("Part 2: {}", second(input));
}

