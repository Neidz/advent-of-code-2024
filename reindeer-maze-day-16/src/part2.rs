use std::collections::{HashMap, HashSet, VecDeque};

use crate::input::INPUT;

type Coordinates = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Reindeer {
    position: Coordinates,
    score: usize,
    direction: Direction,
    rotated_in_place: usize,
    path: Vec<Coordinates>,
}

impl Reindeer {
    pub fn new(position: Coordinates) -> Self {
        Reindeer {
            position,
            direction: Direction::Right,
            score: 0,
            rotated_in_place: 0,
            path: vec![position],
        }
    }

    fn travel(&mut self, walls: &HashSet<Coordinates>) -> Result<(), ()> {
        let (x, y) = self.position;

        let next_postion = match self.direction {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };

        if walls.contains(&next_postion) {
            return Err(());
        }

        self.path.push(next_postion);
        self.position = next_postion;
        self.score += 1;
        self.rotated_in_place = 0;

        Ok(())
    }

    fn rotate_clockwise(&mut self) -> Result<(), ()> {
        if self.rotated_in_place == 1 {
            return Err(());
        }
        self.rotated_in_place += 1;
        self.score += 1000;

        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        };

        Ok(())
    }

    fn rotate_counterclockwise(&mut self) -> Result<(), ()> {
        if self.rotated_in_place == 1 {
            return Err(());
        }
        self.rotated_in_place += 1;
        self.score += 1000;

        self.direction = match self.direction {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        };

        Ok(())
    }
}

pub fn part2() {
    let mut walls: HashSet<Coordinates> = HashSet::new();
    let mut reindeer: Option<Reindeer> = None;
    let mut end_tile: Option<Coordinates> = None;

    for (y, line) in INPUT.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let (x, y): (i32, i32) = (x.try_into().unwrap(), y.try_into().unwrap());

            match c {
                '#' => {
                    walls.insert((x, y));
                }
                'S' => {
                    reindeer = Some(Reindeer::new((x, y)));
                }
                'E' => {
                    end_tile = Some((x, y));
                }
                _ => {}
            };
        }
    }

    let reindeer = reindeer.unwrap();
    let walls = walls;
    let end_tile = end_tile.unwrap();

    let mut best_score = usize::MAX;
    let mut reindeer_queue = VecDeque::new();

    let mut top_facing_reindeer = reindeer.clone();
    top_facing_reindeer.rotate_counterclockwise().unwrap();
    reindeer_queue.push_back(reindeer);
    reindeer_queue.push_back(top_facing_reindeer);

    let mut best_possible_score_at_position: HashMap<(Coordinates, Direction), usize> =
        HashMap::new();

    let mut possible_best_reindeers: Vec<Reindeer> = Vec::new();

    while let Some(mut current_reindeer) = reindeer_queue.pop_front() {
        if current_reindeer.score > best_score {
            continue;
        }

        let possible_lowest_score = best_possible_score_at_position
            .get(&(current_reindeer.position, current_reindeer.direction));

        if let Some(score) = possible_lowest_score {
            if *score < current_reindeer.score {
                continue;
            }
        }
        best_possible_score_at_position.insert(
            (current_reindeer.position, current_reindeer.direction),
            current_reindeer.score,
        );

        match current_reindeer.travel(&walls) {
            Ok(_) => {
                if current_reindeer.position == end_tile {
                    best_score = current_reindeer.score;

                    possible_best_reindeers.push(current_reindeer);

                    continue;
                }

                let mut clockwise_reindeer = current_reindeer.clone();
                if clockwise_reindeer.rotate_clockwise().is_ok() {
                    reindeer_queue.push_back(clockwise_reindeer);
                };

                let mut counterclockwise_reindeer = current_reindeer.clone();
                if counterclockwise_reindeer.rotate_counterclockwise().is_ok() {
                    reindeer_queue.push_back(counterclockwise_reindeer);
                };

                reindeer_queue.push_back(current_reindeer);
            }
            Err(_) => continue,
        }
    }

    let mut tiles_on_best_paths = HashSet::new();

    possible_best_reindeers
        .into_iter()
        .filter(|reindeer| reindeer.score == best_score)
        .for_each(|reindeer| {
            tiles_on_best_paths.extend(reindeer.path);
        });

    println!(
        "Part 2: amount of tiles on best paths is {}",
        tiles_on_best_paths.len()
    );
}
