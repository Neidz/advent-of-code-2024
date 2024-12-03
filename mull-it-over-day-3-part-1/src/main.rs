use input::INPUT;
use regex::Regex;

mod input;

fn main() {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let instructions: Vec<usize> = re
        .captures_iter(INPUT)
        .map(|c| c.extract())
        .map(|nums| {
            let (_, [x_str, y_str]) = nums;

            let x: usize = x_str.parse().unwrap();
            let y: usize = y_str.parse().unwrap();

            return x * y;
        })
        .collect();

    let sum: usize = instructions.iter().sum();

    println!("Sum: {}", sum);
}
