use std::collections::{HashMap, HashSet};

use crate::input::INPUT;

type AntenaType = char;
type Coordinate = (i32, i32);

pub fn part2() {
    let mut antenas: HashMap<Coordinate, AntenaType> = HashMap::new();
    let mut width: Option<i32> = None;
    let mut height = 0i32;

    for (y, line) in INPUT.lines().enumerate() {
        if width.is_none() {
            width = Some(line.len() as i32);
        }
        height += 1;

        for (x, antena_type) in line.chars().enumerate() {
            if antena_type == '.' {
                continue;
            }

            antenas.insert((x as i32, y as i32), antena_type);
        }
    }

    let width = width.unwrap();

    let mut antinodes: HashSet<Coordinate> = HashSet::new();

    for ((x, y), antena_type) in &antenas {
        antinodes.insert((*x, *y));
        for ((second_x, second_y), second_antena_type) in &antenas {
            if x == second_x && y == second_y {
                continue;
            }

            if antena_type != second_antena_type {
                continue;
            }

            let x_diff = *x - *second_x;
            let y_diff = *y - *second_y;

            let mut antinode_x = *x;
            let mut antinode_y = *y;

            loop {
                antinode_x += x_diff;
                antinode_y += y_diff;

                let valid =
                    antinode_x >= 0 && antinode_x < width && antinode_y >= 0 && antinode_y < height;

                if !valid {
                    break;
                }

                antinodes.insert((antinode_x, antinode_y));
            }
        }
    }

    println!(
        "Part 2: amount of antinodes with resonances is {}",
        antinodes.len()
    );
}
