use std::collections::HashMap;

use crate::input::INPUT;

type Coordinate = (i32, i32);

const DIRECTION_OFFSETS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn traverse(
    current_value: i32,
    current_coords: Coordinate,
    coords: &HashMap<Coordinate, i32>,
    sum: &mut i32,
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
                *sum += 1;
                continue;
            }

            traverse(value_at_offset, (x_with_offset, y_with_offset), coords, sum);
        }
    }
}

pub fn part2() {
    let mut coordinates: HashMap<Coordinate, i32> = HashMap::new();

    for (y, line) in INPUT.lines().enumerate() {
        for (x, num) in line.chars().enumerate() {
            coordinates.insert(
                (x.try_into().unwrap(), y.try_into().unwrap()),
                num.to_digit(10).unwrap().try_into().unwrap(),
            );
        }
    }

    let mut sum = 0i32;

    for ((x, y), num) in coordinates.iter() {
        if *num != 0 {
            continue;
        }

        traverse(*num, (*x, *y), &coordinates, &mut sum);
    }

    println!("Part2: sum is {}", sum);
}
