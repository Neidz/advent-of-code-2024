use regex::Regex;

use crate::input::INPUT;

const A_PRESS_TOKEN_PRICE: i32 = 3;
const B_PRESS_TOKEN_PRICE: i32 = 1;

type Coordinates = (i32, i32);

#[derive(Debug)]
struct Game {
    a_change: Coordinates,
    b_change: Coordinates,
    prize_location: Coordinates,
}

impl Game {
    pub fn tokens_to_win(&self) -> Option<i32> {
        let (a_change_x, a_change_y) = self.a_change;
        let (b_change_x, b_change_y) = self.b_change;
        let (prize_location_x, prize_location_y) = self.prize_location;

        let a_presses = (prize_location_x * b_change_y - prize_location_y * b_change_x)
            / (a_change_x * b_change_y - a_change_y * b_change_x);
        let b_presses = (a_change_x * prize_location_y - a_change_y * prize_location_x)
            / (a_change_x * b_change_y - a_change_y * b_change_x);

        if a_presses * a_change_x + b_presses * b_change_x == prize_location_x
            && a_presses * a_change_y + b_presses * b_change_y == prize_location_y
        {
            return Some(a_presses * A_PRESS_TOKEN_PRICE + b_presses * B_PRESS_TOKEN_PRICE);
        }

        None
    }
}

pub fn part1() {
    let re =
        Regex::new(r".+X\+(\d+), Y\+(\d+)\n.+X\+(\d+), Y\+(\d+)\n.+X\=(\d+), Y\=(\d+)").unwrap();

    let games: Vec<Game> = re
        .captures_iter(INPUT)
        .map(|c| c.extract())
        .map(|(_, [a_x, a_y, b_x, b_y, prize_x, prize_y])| Game {
            a_change: (a_x.parse().unwrap(), a_y.parse().unwrap()),
            b_change: (b_x.parse().unwrap(), b_y.parse().unwrap()),
            prize_location: (prize_x.parse().unwrap(), prize_y.parse().unwrap()),
        })
        .collect();

    let tokens: i32 = games.iter().filter_map(|game| game.tokens_to_win()).sum();

    println!("Part 1: tokens to win {}", tokens);
}
