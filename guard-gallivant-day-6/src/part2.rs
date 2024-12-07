use std::collections::HashSet;

use crate::input::INPUT;

const MOVES_LIMIT: usize = 1_000_000;

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Coordinates = (usize, usize);

pub fn part2() {
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
    let (guard_starting_x, guard_starting_y) = guard.0;

    let mut infinite_moves: usize = 0;

    for y in 0..map_height {
        for x in 0..map_width {
            let is_obstructed = obstructions.get(&(x, y)).is_some();
            let is_guard = x == guard_starting_x && y == guard_starting_y;

            if !is_obstructed && !is_guard {
                let mut expanded_obstructions = obstructions.clone();
                expanded_obstructions.insert((x, y));

                let is_infinite =
                    check_if_infinite(guard, map_width, map_height, expanded_obstructions);

                if is_infinite {
                    infinite_moves += 1;
                }
            }
        }
    }

    println!(
        "Part 2: possible new obstructions for infinite loop of moves: {}",
        infinite_moves
    );
}

fn check_if_infinite(
    guard: (Coordinates, Direction),
    map_width: usize,
    map_height: usize,
    obstructions: HashSet<Coordinates>,
) -> bool {
    let mut moves: usize = 0;

    let (mut guard_x, mut guard_y) = guard.0;
    let mut guard_direction = guard.1;

    let mut locations_visited: HashSet<Coordinates> = HashSet::new();
    locations_visited.insert((guard_x, guard_y));

    loop {
        moves += 1;
        if moves > MOVES_LIMIT {
            return true;
        }

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

    return false;
}
