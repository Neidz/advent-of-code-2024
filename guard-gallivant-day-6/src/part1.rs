use std::collections::HashSet;

use crate::input::INPUT;

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Coordinates = (usize, usize);

pub fn part1() {
    let mut obstructions: HashSet<Coordinates> = HashSet::new();
    let mut guard: Option<(Coordinates, Direction)> = None;
    let mut map_width: Option<usize> = None;
    let mut map_height: usize = 0;

    for (y, line) in INPUT.lines().enumerate() {
        if map_width.is_none() {
            map_width = Some(line.len());
        }
        map_height += 1;

        for (x, entity) in line.chars().enumerate() {
            match entity {
                '#' => {
                    obstructions.insert((x, y));
                }
                '^' => {
                    guard = Some(((x, y), Direction::Up));
                }
                'v' => {
                    guard = Some(((x, y), Direction::Down));
                }
                '>' => {
                    guard = Some(((x, y), Direction::Right));
                }
                '<' => {
                    guard = Some(((x, y), Direction::Left));
                }
                '.' => {}
                _ => panic!("Unexpected character: {}", entity),
            }
        }
    }

    let map_width = map_width.unwrap();

    let guard = guard.unwrap();
    let (mut guard_x, mut guard_y) = guard.0;
    let mut guard_direction = guard.1;

    let mut locations_visited: HashSet<Coordinates> = HashSet::new();
    locations_visited.insert((guard_x, guard_y));

    loop {
        match guard_direction {
            Direction::Up => {
                if guard_y == 0 {
                    break;
                }

                let (next_guard_x, next_guard_y) = (guard_x, guard_y - 1);

                let is_obstructed = obstructions.get(&(next_guard_x, next_guard_y)).is_some();

                if is_obstructed {
                    guard_direction = Direction::Right;
                    continue;
                }

                guard_x = next_guard_x;
                guard_y = next_guard_y;
                locations_visited.insert((guard_x, guard_y));
            }
            Direction::Down => {
                let (next_guard_x, next_guard_y) = (guard_x, guard_y + 1);

                if next_guard_y == map_height {
                    break;
                }

                let is_obstructed = obstructions.get(&(next_guard_x, next_guard_y)).is_some();

                if is_obstructed {
                    guard_direction = Direction::Left;
                    continue;
                }

                guard_x = next_guard_x;
                guard_y = next_guard_y;
                locations_visited.insert((guard_x, guard_y));
            }
            Direction::Left => {
                if guard_x == 0 {
                    break;
                }

                let (next_guard_x, next_guard_y) = (guard_x - 1, guard_y);

                let is_obstructed = obstructions.get(&(next_guard_x, next_guard_y)).is_some();

                if is_obstructed {
                    guard_direction = Direction::Up;
                    continue;
                }

                guard_x = next_guard_x;
                guard_y = next_guard_y;
                locations_visited.insert((guard_x, guard_y));
            }
            Direction::Right => {
                let (next_guard_x, next_guard_y) = (guard_x + 1, guard_y);

                if next_guard_x == map_width {
                    break;
                }

                let is_obstructed = obstructions.get(&(next_guard_x, next_guard_y)).is_some();

                if is_obstructed {
                    guard_direction = Direction::Down;
                    continue;
                }

                guard_x = next_guard_x;
                guard_y = next_guard_y;
                locations_visited.insert((guard_x, guard_y));
            }
        };
    }

    println!("Part 1: locations visited: {}", locations_visited.len());
}
