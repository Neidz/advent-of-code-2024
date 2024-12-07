use crate::input::INPUT;

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    pub fn get_permutations(len: usize) -> Vec<Vec<Operator>> {
        let mut permutations: Vec<Vec<Operator>> = Vec::new();

        let mut permutation = vec![Operator::Add; len];

        loop {
            permutations.push(permutation.clone());

            if !permutation.contains(&Operator::Add) {
                break;
            }

            for i in 0..permutation.len() {
                let operator = permutation[i];

                if operator == Operator::Add {
                    permutation[i] = Operator::Multiply;
                    break;
                }

                permutation[i] = Operator::Add;
            }
        }

        permutations
    }
}

pub fn part1() {
    let lines: Vec<(usize, Vec<usize>)> = INPUT
        .lines()
        .map(|line| {
            let (result, rest) = line.split_once(":").unwrap();

            let result: usize = result.parse().unwrap();
            let rest: Vec<usize> = rest
                .trim_start()
                .split(" ")
                .map(|el| el.parse().unwrap())
                .collect();

            (result, rest)
        })
        .collect();

    let mut can_be_solved_sum = 0;

    for (result, nums) in lines {
        let operator_permutations = Operator::get_permutations(nums.len() - 1);

        for operators in operator_permutations {
            let mut operator_result = nums[0];

            for (operator_index, operator) in operators.iter().enumerate() {
                let next_num = nums[operator_index + 1];

                match *operator {
                    Operator::Add => {
                        operator_result += next_num;
                    }
                    Operator::Multiply => {
                        operator_result = operator_result * next_num;
                    }
                }

                if operator_result > result {
                    break;
                }
            }

            if operator_result == result {
                can_be_solved_sum += result;
                break;
            }
        }
    }

    println!("Part 1: can be solved: {}", can_be_solved_sum);
}
