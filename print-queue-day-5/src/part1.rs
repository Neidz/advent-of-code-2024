use crate::input::{INPUT_RULES, INPUT_UPDATES};

pub fn part1() {
    let mut correct_updates: Vec<String> = Vec::new();

    'updates: for update in INPUT_UPDATES.lines() {
        for rule in INPUT_RULES.lines() {
            let (first_rule_num, second_rule_num) = rule.split_once('|').unwrap();

            let first_index = update.find(first_rule_num);
            let second_index = update.find(second_rule_num);

            if let (Some(first_index), Some(second_index)) = (first_index, second_index) {
                if first_index > second_index {
                    continue 'updates;
                }
            }
        }

        correct_updates.push(update.to_owned());
    }

    let mut sum_of_middle = 0;

    for update in correct_updates.iter() {
        let pages: Vec<&str> = update.split(",").collect();

        let middle_index = pages.len() / 2;

        let middle_page_num: usize = pages[middle_index].parse().unwrap();

        sum_of_middle += middle_page_num;
    }

    println!("Part 1: sum of middle pages: {}", sum_of_middle);
}
