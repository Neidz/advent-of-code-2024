use std::collections::HashMap;

use crate::input::INPUT;

const ITERATIONS: usize = 75;

pub fn part2() {
    let nums: Vec<usize> = INPUT.split(" ").map(|el| el.parse().unwrap()).collect();
    let mut sum_memo: HashMap<(usize, usize), usize> = HashMap::new();

    let sum: usize = nums
        .iter()
        .map(|num| split_and_sum_stone_amount(*num, ITERATIONS, &mut sum_memo))
        .sum();

    println!("Part 2: number of stones is {}", sum);
}

fn split_and_sum_stone_amount(
    num: usize,
    iterations: usize,
    sum_memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if iterations == 0 {
        return 1;
    }

    let sum_from_memo = sum_memo.get(&(num, iterations));

    if let Some(sum) = sum_from_memo {
        return *sum;
    }

    let sum = match num {
        0 => split_and_sum_stone_amount(1, iterations - 1, sum_memo),
        n if n.to_string().len() % 2 == 0 && n.to_string().len() > 1 => {
            let num_str = n.to_string();
            let (first, second) = num_str.split_at(num_str.len() / 2);

            let first = first.parse().unwrap();
            let second = second.parse().unwrap();

            split_and_sum_stone_amount(first, iterations - 1, sum_memo)
                + split_and_sum_stone_amount(second, iterations - 1, sum_memo)
        }
        n => split_and_sum_stone_amount(n * 2024, iterations - 1, sum_memo),
    };

    let _ = sum_memo.insert((num, iterations), sum);

    sum
}
