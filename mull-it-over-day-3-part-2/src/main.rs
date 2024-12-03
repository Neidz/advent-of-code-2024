use input::INPUT;
use regex::Regex;

mod input;

enum Instruction {
    Dont,
    Do,
    Nums(usize, usize),
}

fn main() {
    let re = Regex::new(r"mul\((\d{1,3},\d{1,3})\)|(do\(\))|(don't\(\))").unwrap();

    let instructions: Vec<Instruction> = re
        .captures_iter(INPUT)
        .map(|c| c.extract())
        .map(|raw_instructions| {
            let (_, [raw_instruction]) = raw_instructions;

            match raw_instruction {
                "don't()" => Instruction::Dont,
                "do()" => Instruction::Do,
                _ => {
                    let (x_str, y_str) = raw_instruction.split_once(",").unwrap();
                    let x: usize = x_str.parse().unwrap();
                    let y: usize = y_str.parse().unwrap();

                    Instruction::Nums(x, y)
                }
            }
        })
        .collect();

    let mut sum = 0;
    let mut enabled = true;

    for instruction in instructions.iter() {
        match instruction {
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
            Instruction::Nums(x, y) => {
                if enabled {
                    sum += x * y
                }
            }
        }
    }

    println!("Sum: {}", sum);
}
