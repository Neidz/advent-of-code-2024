use crate::input::INPUT;

const MAP_WIDTH: i32 = 101;
const MAP_HEIGHT: i32 = 103;
const SECONDS_ELAPSED: i32 = 100;

type Coordinates = (i32, i32);
type Velocity = (i32, i32);

#[derive(Debug)]
struct Robot {
    position: Coordinates,
    velocity: Velocity,
}

impl Robot {
    pub fn calculate_position_after(&self, seconds: i32) -> Coordinates {
        let (position_x, position_y) = self.position;
        let (velocity_x, velocity_y) = self.velocity;

        let final_x_without_wrapping = position_x + velocity_x * seconds;
        let final_y_without_wrapping = position_y + velocity_y * seconds;

        let final_x = ((final_x_without_wrapping % MAP_WIDTH) + MAP_WIDTH) % MAP_WIDTH;
        let final_y = ((final_y_without_wrapping % MAP_HEIGHT) + MAP_HEIGHT) % MAP_HEIGHT;

        return (final_x, final_y);
    }
}

pub fn part1() {
    let robots: Vec<Robot> = INPUT
        .lines()
        .map(|line| {
            let (position_part, velocity_part) = line.split_once(" ").unwrap();

            let (position_x, position_y) = position_part[2..].split_once(",").unwrap();
            let (velocity_x, velocity_y) = velocity_part[2..].split_once(",").unwrap();

            Robot {
                position: (position_x.parse().unwrap(), position_y.parse().unwrap()),
                velocity: (velocity_x.parse().unwrap(), velocity_y.parse().unwrap()),
            }
        })
        .collect();

    let mut quadrant_amount = [0; 4];

    robots.iter().for_each(|robot| {
        let (final_x, final_y) = robot.calculate_position_after(SECONDS_ELAPSED);
        let middle_x = (MAP_WIDTH - 1) / 2;
        let middle_y = (MAP_HEIGHT - 1) / 2;

        if final_x < middle_x && final_y < middle_y {
            quadrant_amount[0] += 1;
        } else if final_x > middle_x && final_y < middle_y {
            quadrant_amount[1] += 1;
        } else if final_x < middle_x && final_y > middle_y {
            quadrant_amount[2] += 1;
        } else if final_x != middle_x && final_y != middle_y {
            quadrant_amount[3] += 1;
        }
    });

    let safety_factor = quadrant_amount
        .into_iter()
        .reduce(|acc, quadrant| acc * quadrant)
        .unwrap();

    println!("Part 1: safety factor is {}", safety_factor);
}
