use std::fs;

pub fn main() {
    println!("------------------------------------ DAY 2 ------------------------------------");

    let input = fs::read_to_string("input/day2.txt").unwrap();
    let input_split: Vec<&str> = input
        .split("\n")
        .filter(|n| n != &"")
        .collect();

    let mut number_valid1 = 0;
    let mut number_valid2 = 0;

    for s in &input_split {
        let raw_string = s.replace(":", "");
        let pass_string: Vec<&str>= raw_string
            .split(" ")
            .collect();

        let min_max: Vec<i32> = pass_string[0]
            .split("-")
            .map(|n| n.parse().unwrap())
            .collect();

        let min = min_max[0];
        let max = min_max[1];

        let policy = pass_string[1];
        let password_attempt: Vec<&str> = pass_string[2]
            .split("")
            .filter(|n| n != &"")
            .collect();

        let mut num_policy = 0;

        for a in &password_attempt {
            if a == &policy {
                num_policy += 1;
            }
        }

        if num_policy >= min && num_policy <= max {
            number_valid1 += 1;
        }

        if (password_attempt[(min - 1) as usize] == policy || password_attempt[(max - 1) as usize] == policy) &&
         !(password_attempt[(min - 1) as usize] == policy && password_attempt[(max - 1) as usize] == policy) {
                number_valid2 += 1;
            }
    }

    println!("Part 1: {}", number_valid1);
    println!("Part 2: {}", number_valid2);
}
