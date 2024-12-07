use crate::input::{INPUT_RULES, INPUT_UPDATES};

pub fn part2() {
    let mut wrong_updates: Vec<String> = Vec::new();

    'updates: for update in INPUT_UPDATES.lines() {
        for rule in INPUT_RULES.lines() {
            let (first_rule_num, second_rule_num) = rule.split_once('|').unwrap();

            let first_index = update.find(first_rule_num);
            let second_index = update.find(second_rule_num);

            if let (Some(first_index), Some(second_index)) = (first_index, second_index) {
                if first_index > second_index {
                    wrong_updates.push(update.to_owned());

                    continue 'updates;
                }
            }
        }
    }

    let mut fixed_updates: Vec<Vec<usize>> = Vec::new();

    for update in wrong_updates {
        let mut pages: Vec<usize> = update
            .split(",")
            .map(|page| page.parse().unwrap())
            .collect();

        'rule_loop: loop {
            for rule in INPUT_RULES.lines() {
                let (first_rule, second_rule) = rule.split_once('|').unwrap();
                let first_rule_num: usize = first_rule.parse().unwrap();
                let second_rule_num: usize = second_rule.parse().unwrap();

                let first_index = pages.iter().position(|num| *num == first_rule_num);
                let second_index = pages.iter().position(|num| *num == second_rule_num);

                if let (Some(first_index), Some(second_index)) = (first_index, second_index) {
                    if first_index > second_index {
                        let wrong_num = pages.remove(second_index);
                        pages.push(wrong_num);

                        continue 'rule_loop;
                    }
                }
            }

            fixed_updates.push(pages);
            break;
        }
    }

    let mut sum_of_middle = 0;

    for pages in fixed_updates.iter() {
        let middle_index = pages.len() / 2;

        let middle_page_num = pages[middle_index];

        sum_of_middle += middle_page_num;
    }

    println!("Part 2: sum of fixed middle pages: {}", sum_of_middle);
}
