use std::collections::HashMap;

use crate::input::INPUT;

#[derive(Debug)]
enum Direction {
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Letter {
    X,
    M,
    A,
    S,
}

impl Letter {
    pub fn next(&self) -> Option<Self> {
        match self {
            Letter::X => Some(Letter::M),
            Letter::M => Some(Letter::A),
            Letter::A => Some(Letter::S),
            Letter::S => None,
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum State {
    Searching,
    Found,
    Invalid,
}

#[derive(Debug)]
struct SearchAttempt {
    current_letter: Letter,
    current_letter_coordinates: (usize, usize),
    direction: Direction,
    state: State,
}

impl SearchAttempt {
    pub fn new_in_all_directions(coordinates: (usize, usize)) -> Vec<Self> {
        vec![
            Self::new(coordinates, Direction::Top),
            Self::new(coordinates, Direction::TopRight),
            Self::new(coordinates, Direction::Right),
            Self::new(coordinates, Direction::BottomRight),
            Self::new(coordinates, Direction::Bottom),
            Self::new(coordinates, Direction::BottomLeft),
            Self::new(coordinates, Direction::Left),
            Self::new(coordinates, Direction::TopLeft),
        ]
    }

    pub fn advance(&mut self, letters: &HashMap<(usize, usize), Letter>) -> State {
        if self.state != State::Searching {
            return self.state;
        }

        let (x, y) = self.current_letter_coordinates;
        let (x_change, y_change) = match self.direction {
            Direction::Top => (0, -1),
            Direction::TopRight => (1, -1),
            Direction::Right => (1, 0),
            Direction::BottomRight => (1, 1),
            Direction::Bottom => (0, 1),
            Direction::BottomLeft => (-1, 1),
            Direction::Left => (-1, 0),
            Direction::TopLeft => (-1, -1),
        };

        if (x_change == -1 && x == 0) || (y_change == -1 && y == 0) {
            self.state = State::Invalid;
            return self.state;
        };

        let new_x = (x as i32 + x_change) as usize;
        let new_y = (y as i32 + y_change) as usize;

        if let Some(next_valid_letter) = self.current_letter.next() {
            if let Some(letter_at_new_coords) = letters.get(&(new_x, new_y)) {
                if *letter_at_new_coords == next_valid_letter {
                    self.current_letter = *letter_at_new_coords;
                    self.current_letter_coordinates = (new_x, new_y);

                    if self.current_letter == Letter::S {
                        self.state = State::Found;
                    }

                    return self.state;
                }
            };
        }

        self.state = State::Invalid;
        self.state
    }

    fn new(coordinates: (usize, usize), direction: Direction) -> Self {
        SearchAttempt {
            current_letter: Letter::X,
            current_letter_coordinates: coordinates,
            direction,
            state: State::Searching,
        }
    }
}

pub fn part1() {
    let mut letter_coordinates: HashMap<(usize, usize), Letter> = HashMap::new();

    for (line_index, line) in INPUT.lines().enumerate() {
        for (letter_index, letter) in line.chars().enumerate() {
            let parsed_letter = match letter {
                'X' => Letter::X,
                'M' => Letter::M,
                'A' => Letter::A,
                'S' => Letter::S,
                _ => panic!("unexpected letter: {}", letter),
            };

            letter_coordinates.insert((letter_index, line_index), parsed_letter);
        }
    }

    let mut search_attempts: Vec<SearchAttempt> = Vec::new();

    for ((x, y), letter) in letter_coordinates.iter() {
        if *letter == Letter::X {
            let mut search_attempt_in_all_directions =
                SearchAttempt::new_in_all_directions((*x, *y));

            search_attempts.append(&mut search_attempt_in_all_directions);
        }
    }

    let mut xmas_count = 0;

    for _ in 0..3 {
        for attempt in search_attempts.iter_mut() {
            if attempt.state == State::Searching {
                let new_state = attempt.advance(&letter_coordinates);
                if new_state == State::Found {
                    xmas_count += 1;
                }
            }
        }
    }

    println!("Part 1: XMAS found {} times", xmas_count);
}