use crate::input::INPUT;

const ITERATIONS: usize = 25;

pub fn part1() {
    let mut nums: Vec<usize> = INPUT.split(" ").map(|el| el.parse().unwrap()).collect();

    for _ in 0..ITERATIONS {
        nums = nums
            .iter()
            .flat_map(|num| match num {
                0 => vec![1],
                n if n.to_string().len() % 2 == 0 && n.to_string().len() > 1 => {
                    let num_str = n.to_string();
                    let (first, second) = num_str.split_at(num_str.len() / 2);

                    vec![first.parse().unwrap(), second.parse().unwrap()]
                }
                n => vec![n * 2024],
            })
            .collect();
    }

    println!("Part 1: number of stones is {}", nums.len());
}
