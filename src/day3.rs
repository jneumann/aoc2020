use std::fs;

pub struct Slope {
    x: usize,
    y: usize
}

pub fn get_tree(coords: Vec<Slope>, area_map: &Vec<Vec<bool>>) -> Vec<i64> {
    let mut tree_count: Vec<i64> = Vec::new();

    for c in coords {
        let mut temp_count = 0;
        let x_slope = c.x;
        let y_slope = c.y;
        let mut x = 0;
        let mut y = 0;

        loop {
            if area_map.len() <= y {
                break;
            } else {
                if area_map[y].len() <= x {
                    x = x - area_map[y].len();
                }
            }

            if area_map[y][x] {
                temp_count += 1;
            }

            x += x_slope;
            y += y_slope;
        }

        tree_count.push(temp_count);
    }


    tree_count
}

pub fn main() {
    println!("------------------------------------ DAY 3 ------------------------------------");

    let mut area_map: Vec<Vec<bool>> = Vec::new();

    let input = fs::read_to_string("input/day3.txt").unwrap();
    input.lines().for_each(|l| {
        let mut temp_vec: Vec<bool> = Vec::new();

        l.chars().for_each(|r| {
            match r {
                '.' => temp_vec.push(false),
                '#' => temp_vec.push(true),
                _ => panic!("Unknown character in map: {}", r)
            }
        });

        area_map.push(temp_vec);
    });

    let mut slope_vec: Vec<Slope> = Vec::new();
    slope_vec.push(Slope{x: 3, y: 1});

    let mut part1: i64 = 1;
    let tree_count = get_tree(slope_vec, &area_map);

    for t in tree_count {
        part1 *= t;
    }

    println!("Part 1: {}", part1);

    let mut slope_vec2: Vec<Slope> = Vec::new();
    slope_vec2.push(Slope{x: 1, y: 1});
    slope_vec2.push(Slope{x: 3, y: 1});
    slope_vec2.push(Slope{x: 5, y: 1});
    slope_vec2.push(Slope{x: 7, y: 1});
    slope_vec2.push(Slope{x: 1, y: 2});

    let mut part2: i64 = 1;
    let tree_count2 = get_tree(slope_vec2, &area_map);

    for t in tree_count2 {
        part2 *= t;
    }

    println!("Part 2: {}", part2);
}
