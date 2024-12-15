use std::collections::HashSet;

use crate::input::{INPUT_MAP, INPUT_MOVES};

type Coordinates = (i32, i32);

pub fn part1() {
    let mut box_coordinates: HashSet<Coordinates> = HashSet::new();
    let mut wall_coordinates: HashSet<Coordinates> = HashSet::new();
    let mut robot_coordinates: Option<Coordinates> = None;

    for (y, line) in INPUT_MAP.lines().enumerate() {
        for (x, entity) in line.chars().enumerate() {
            match entity {
                '.' => {}
                '#' => {
                    wall_coordinates.insert((x.try_into().unwrap(), y.try_into().unwrap()));
                }
                '@' => {
                    robot_coordinates = Some((x.try_into().unwrap(), y.try_into().unwrap()));
                }
                'O' => {
                    box_coordinates.insert((x.try_into().unwrap(), y.try_into().unwrap()));
                }
                _ => panic!("Unexpected character"),
            }
        }
    }

    let mut robot_coordinates = robot_coordinates.unwrap();

    for robot_move in INPUT_MOVES.chars() {
        if char::is_whitespace(robot_move) {
            continue;
        }

        if can_move(
            robot_coordinates,
            robot_move,
            &box_coordinates,
            &wall_coordinates,
        ) {
            move_entities(&mut robot_coordinates, robot_move, &mut box_coordinates);
        }
    }

    let sum: i32 = box_coordinates.into_iter().map(|(x, y)| 100 * y + x).sum();

    println!("Part 1: sum of gps coordinates is {}", sum);
}

fn can_move(
    robot_coordinates: Coordinates,
    direction: char,
    box_coordinates: &HashSet<Coordinates>,
    wall_coordinates: &HashSet<Coordinates>,
) -> bool {
    let (mut x, mut y) = robot_coordinates;

    let (x_change, y_change) = match direction {
        '^' => (0, -1),
        'v' => (0, 1),
        '<' => (-1, 0),
        '>' => (1, 0),
        _ => panic!("Unexpected character"),
    };

    loop {
        x += x_change;
        y += y_change;

        if wall_coordinates.contains(&(x, y)) {
            return false;
        }

        if !box_coordinates.contains(&(x, y)) {
            return true;
        }
    }
}

fn move_entities(
    robot_coordinates: &mut Coordinates,
    direction: char,
    box_coordinates: &mut HashSet<Coordinates>,
) {
    let (mut x, mut y) = *robot_coordinates;

    let (x_change, y_change) = match direction {
        '^' => (0, -1),
        'v' => (0, 1),
        '<' => (-1, 0),
        '>' => (1, 0),
        _ => panic!("Unexpected character"),
    };

    *robot_coordinates = (
        robot_coordinates.0 + x_change,
        robot_coordinates.1 + y_change,
    );

    let mut box_coords_to_insert: Vec<(i32, i32)> = Vec::new();

    loop {
        x += x_change;
        y += y_change;

        if box_coordinates.contains(&(x, y)) {
            box_coordinates.remove(&(x, y));
            box_coords_to_insert.push((x + x_change, y + y_change));
        } else {
            break;
        }
    }

    box_coords_to_insert.into_iter().for_each(|(box_x, box_y)| {
        box_coordinates.insert((box_x, box_y));
    });
}
