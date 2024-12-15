use std::collections::{HashMap, HashSet};

use crate::input::{INPUT_MAP, INPUT_MOVES};

type Coordinates = (i32, i32);

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum BoxPart {
    Left,
    Right,
}

pub fn part2() {
    let mut box_part_coordinates: HashMap<Coordinates, BoxPart> = HashMap::new();
    let mut wall_coordinates: HashSet<Coordinates> = HashSet::new();
    let mut robot_coordinates: Option<Coordinates> = None;

    for (y, line) in INPUT_MAP.lines().enumerate() {
        for (x, entity) in line.chars().enumerate() {
            let wide_x: i32 = (x * 2).try_into().unwrap();
            let y: i32 = y.try_into().unwrap();

            match entity {
                '.' => {}
                '#' => {
                    wall_coordinates.insert((wide_x, y));
                    wall_coordinates.insert((wide_x + 1, y));
                }
                '@' => {
                    robot_coordinates = Some((wide_x, y));
                }
                'O' => {
                    let box_left_x = wide_x;
                    let box_y = y;

                    box_part_coordinates.insert((box_left_x, box_y), BoxPart::Left);
                    box_part_coordinates.insert((box_left_x + 1, box_y), BoxPart::Right);
                }
                _ => panic!("Unexpected character"),
            }
        }
    }

    let mut robot_coordinates = robot_coordinates.unwrap();

    print_map(&box_part_coordinates, &wall_coordinates, robot_coordinates);

    for direction in INPUT_MOVES.chars() {
        if char::is_whitespace(direction) {
            continue;
        }

        let (x_change, y_change) = match direction {
            '^' => (0, -1),
            'v' => (0, 1),
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => panic!("Unexpected character"),
        };

        let target_robot_coordinates = (
            robot_coordinates.0 + x_change,
            robot_coordinates.1 + y_change,
        );

        println!("Move: {}", direction);

        if can_move(
            target_robot_coordinates,
            x_change,
            y_change,
            &box_part_coordinates,
            &wall_coordinates,
        ) {
            robot_coordinates = target_robot_coordinates;

            let mut visited: HashSet<Coordinates> = HashSet::new();

            move_boxes(
                robot_coordinates,
                x_change,
                y_change,
                &mut box_part_coordinates,
                &mut visited,
            );

            print_map(&box_part_coordinates, &wall_coordinates, robot_coordinates);
        }
    }

    let sum: i32 = box_part_coordinates
        .into_iter()
        .filter(|(_, part)| *part == BoxPart::Left)
        .map(|((x, y), _)| 100 * y + x)
        .sum();

    println!("Part 2: sum of gps coordinates is {}", sum);
}

fn can_move(
    entity_coordinates: Coordinates,
    x_change: i32,
    y_change: i32,
    box_part_coordinates: &HashMap<Coordinates, BoxPart>,
    wall_coordinates: &HashSet<Coordinates>,
) -> bool {
    let (x, y) = entity_coordinates;

    if wall_coordinates.contains(&(x, y)) {
        return false;
    }

    match box_part_coordinates.get(&(x, y)) {
        Some(box_part) => match x_change {
            1 => {
                if *box_part == BoxPart::Left {
                    return can_move(
                        (x + x_change * 2, y),
                        x_change,
                        y_change,
                        box_part_coordinates,
                        wall_coordinates,
                    );
                }
                return can_move(
                    (x + x_change, y),
                    x_change,
                    y_change,
                    box_part_coordinates,
                    wall_coordinates,
                );
            }
            -1 => {
                if *box_part == BoxPart::Left {
                    return can_move(
                        (x + x_change, y),
                        x_change,
                        y_change,
                        box_part_coordinates,
                        wall_coordinates,
                    );
                }
                return can_move(
                    (x + x_change * 2, y),
                    x_change,
                    y_change,
                    box_part_coordinates,
                    wall_coordinates,
                );
            }
            _ => {
                let other_part_x = if *box_part == BoxPart::Left {
                    x + 1
                } else {
                    x - 1
                };

                return can_move(
                    (x, y + y_change),
                    x_change,
                    y_change,
                    box_part_coordinates,
                    wall_coordinates,
                ) && can_move(
                    (other_part_x, y + y_change),
                    x_change,
                    y_change,
                    box_part_coordinates,
                    wall_coordinates,
                );
            }
        },
        None => true,
    }
}

fn move_boxes(
    coordinates: Coordinates,
    x_change: i32,
    y_change: i32,
    box_part_coordinates: &mut HashMap<Coordinates, BoxPart>,
    visited: &mut HashSet<Coordinates>,
) {
    let (x, y) = coordinates;

    if !visited.insert((x, y)) {
        return;
    }

    let box_part = match box_part_coordinates.get(&(x, y)) {
        Some(box_part) => *box_part,
        None => return,
    };

    let (other_part_x, other_part_y) = match box_part {
        BoxPart::Left => (x + 1, y),
        BoxPart::Right => (x - 1, y),
    };

    if !visited.insert((other_part_x, other_part_y)) {
        return;
    }

    let next_x = x + x_change;
    let next_y = y + y_change;

    let other_next_x = other_part_x + x_change;
    let other_next_y = other_part_y + y_change;

    move_boxes(
        (next_x, next_y),
        x_change,
        y_change,
        box_part_coordinates,
        visited,
    );

    move_boxes(
        (other_next_x, other_next_y),
        x_change,
        y_change,
        box_part_coordinates,
        visited,
    );

    box_part_coordinates.remove(&(x, y));
    box_part_coordinates.remove(&(other_part_x, other_part_y));

    if box_part == BoxPart::Left {
        box_part_coordinates.insert((next_x, next_y), BoxPart::Left);
        box_part_coordinates.insert((other_next_x, other_next_y), BoxPart::Right);
    } else {
        box_part_coordinates.insert((next_x, next_y), BoxPart::Right);
        box_part_coordinates.insert((other_next_x, other_next_y), BoxPart::Left);
    }
}

fn print_map(
    box_part_coordinates: &HashMap<Coordinates, BoxPart>,
    wall_coordinates: &HashSet<Coordinates>,
    robot_coordinates: Coordinates,
) {
    for y in 0..10 {
        for x in 0..20 {
            if let Some(box_part) = box_part_coordinates.get(&(x, y)) {
                match box_part {
                    BoxPart::Left => print!("["),
                    BoxPart::Right => print!("]"),
                }
            } else if wall_coordinates.get(&(x, y)).is_some() {
                print!("#")
            } else if robot_coordinates == (x, y) {
                print!("@")
            } else {
                print!(".");
            }
        }
        print!("\n");
    }

    println!("\n\n");
}
