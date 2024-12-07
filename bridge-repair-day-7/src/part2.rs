use crate::input::INPUT;

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq)]
enum Operator {
    Add,
    Multiply,
    Concatenation,
}

impl Operator {
    pub fn get_permutations(len: usize) -> Vec<Vec<Operator>> {
        let mut permutations: Vec<Vec<Operator>> = Vec::new();

        let mut permutation = vec![Operator::Add; len];

        loop {
            permutations.push(permutation.clone());

            if !permutation.contains(&Operator::Add) && !permutation.contains(&Operator::Multiply) {
                break;
            }

            for i in 0..permutation.len() {
                let operator = permutation[i];

                match operator {
                    Operator::Add => {
                        permutation[i] = Operator::Multiply;
                        break;
                    }
                    Operator::Multiply => {
                        permutation[i] = Operator::Concatenation;
                        break;
                    }
                    Operator::Concatenation => {
                        permutation[i] = Operator::Add;
                    }
                }
            }
        }

        permutations
    }
}

pub fn part2() {
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
                    Operator::Concatenation => {
                        let mut concatenation_result_str = operator_result.to_string();
                        concatenation_result_str.push_str(&next_num.to_string());
                        operator_result = concatenation_result_str.parse::<usize>().unwrap();
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

    println!("Part 2: can be solved: {}", can_be_solved_sum);
}
