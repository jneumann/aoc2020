use std::collections::HashMap;
use std::fs;

fn validate_byr(year: Option<&String>) -> bool {
    match year {
        Some(s) => {
            match s.parse::<i32>() {
                Ok(p) =>  {
                    return p >= 1920 && p <= 2002
                },
                _ => return false
            }
        },
        None => return false
    }
}

fn validate_iyr(year: Option<&String>) -> bool {
    match year {
        Some(s) => {
            match s.parse::<i32>() {
                Ok(p) =>  {
                    return p >= 2010 && p <= 2020
                },
                _ => return false
            }

        },
        None => return false
    }
}

fn validate_eyr(year: Option<&String>) -> bool {
    match year {
        Some(s) => {
            match s.parse::<i32>() {
                Ok(p) =>  {
                    return p >= 2020 && p <= 2030
                },
                _ => return false
            }

        },
        None => return false
    }
}

fn validate_hgt(year: Option<&String>) -> bool {
    match year {
        Some(s) => {
            if s.ends_with("cm") {
                let height: i32 = s.trim_end_matches("cm").parse::<i32>().unwrap();
                if height >= 150 && height <= 193 {
                    return true;
                }
            }
            if s.ends_with("in") {
                let height: i32 = s.trim_end_matches("in").parse::<i32>().unwrap();
                if height >= 59 && height <= 76 {
                    return true;
                }
            }
            return false;
        },
        None => return false
    }
}

fn validate_hcl(hcl: Option<&String>) -> bool {
    match hcl {
        Some(s) => {
            let (pound, number) = s.split_at(1);
            if pound != "#" {
                return false;
            }

            match i32::from_str_radix(number, 16) {
                Ok(_) => {
                    return true;
                },
                _ => return false
            }
        },
        None => return false
    }
}

fn validate_ecl(ecl: Option<&String>) -> bool {
    match ecl {
        Some(s) => {
            let valid_color = [
                "amb",
                "blu",
                "brn",
                "gry",
                "grn",
                "hzl",
                "oth"
            ].to_vec();

            return valid_color.contains(&s.as_str());
        },
        None => return false
    }
}

fn validate_pid(pid: Option<&String>) -> bool {
    match pid {
        Some(s) => {
            if s.len() == 9 {
                let pid = s.parse::<i32>();

                match pid {
                    Ok(_) => return true,
                    _ => return false
                }
            }

            return false;
        },
        None => return false
    }
}
pub fn main() {
    println!("------------------------------------ DAY 4 ------------------------------------");

    let input = fs::read_to_string("input/day4.txt").unwrap();
    let input_split: Vec<&str> = input
        .split("\n\n")
        .filter(|n| n != &"")
        .collect();

    let mut part1: i32 = 0;
    let mut part2: i32 = 0;

    for i in input_split {
        let passport = i.replace("\n", " ").replace("\r", "");
        let field_list: Vec<&str> = passport.split(" ")
            .filter(|n| n != &"")
            .collect();

        let mut pass_data: HashMap<String, String> = HashMap::new();

        for f in field_list {
            let field: Vec<&str> = f.split(":").collect();
            pass_data.insert(field[0].to_string(), field[1].to_string());
        }

        if pass_data.get("byr") != None &&
            pass_data.get("iyr") != None &&
            pass_data.get("eyr") != None &&
            pass_data.get("hgt") != None &&
            pass_data.get("hcl") != None &&
            pass_data.get("ecl") != None &&
            pass_data.get("pid") != None {
                part1 += 1;
        }

        if validate_byr(pass_data.get("byr")) &&
            validate_iyr(pass_data.get("iyr")) &&
            validate_eyr(pass_data.get("eyr")) &&
            validate_hgt(pass_data.get("hgt")) &&
            validate_hcl(pass_data.get("hcl")) &&
            validate_ecl(pass_data.get("ecl")) &&
            validate_pid(pass_data.get("pid"))
        {
            part2 += 1;
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
