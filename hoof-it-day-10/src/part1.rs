use std::collections::{HashMap, HashSet};

use crate::input::INPUT;

type Coordinate = (i32, i32);

const DIRECTION_OFFSETS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn traverse(
    original_coords: Coordinate,
    current_value: i32,
    current_coords: Coordinate,
    coords: &HashMap<Coordinate, i32>,
    valid_start_and_end: &mut HashSet<(Coordinate, Coordinate)>,
) {
    for (x_offset, y_offset) in DIRECTION_OFFSETS {
        let x_with_offset = current_coords.0 + x_offset;
        let y_with_offset = current_coords.1 + y_offset;

        let value_at_offset = match coords.get(&(x_with_offset, y_with_offset)) {
            Some(val) => *val,
            None => continue,
        };

        if value_at_offset == current_value + 1 {
            if value_at_offset == 9 {
                valid_start_and_end.insert((original_coords, (x_with_offset, y_with_offset)));
                continue;
            }

            traverse(
                original_coords,
                value_at_offset,
                (x_with_offset, y_with_offset),
                coords,
                valid_start_and_end,
            );
        }
    }
}

pub fn part1() {
    let mut coordinates: HashMap<Coordinate, i32> = HashMap::new();

    for (y, line) in INPUT.lines().enumerate() {
        for (x, num) in line.chars().enumerate() {
            coordinates.insert(
                (x.try_into().unwrap(), y.try_into().unwrap()),
                num.to_digit(10).unwrap().try_into().unwrap(),
            );
        }
    }

    let mut valid_start_and_end: HashSet<(Coordinate, Coordinate)> = HashSet::new();

    for ((x, y), num) in coordinates.iter() {
        if *num != 0 {
            continue;
        }

        traverse(
            (*x, *y),
            *num,
            (*x, *y),
            &coordinates,
            &mut valid_start_and_end,
        );
    }

    println!("Part1: sum is {}", valid_start_and_end.len());
}
