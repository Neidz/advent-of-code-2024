use std::{
    collections::HashMap,
    io::{self, BufRead},
    usize,
};

fn main() {
    let num_pairs: Vec<(usize, usize)> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();

            let (left, right) = line.split_once(" ").unwrap();
            let left_num: usize = left.trim().parse().unwrap();
            let right_num: usize = right.trim().parse().unwrap();

            (left_num, right_num)
        })
        .collect();

    let (mut left_list, mut right_list): (Vec<usize>, Vec<usize>) =
        num_pairs.iter().cloned().unzip();

    left_list.sort();
    right_list.sort();

    assert!(left_list.len() == right_list.len());

    let mut diff = vec![0; left_list.len()];

    for i in 0..left_list.len() {
        let left_id = left_list[i];
        let right_id = right_list[i];

        if left_id >= right_id {
            diff[i] = left_id - right_id
        } else {
            diff[i] = right_id - left_id
        }
    }

    let sum: usize = diff.iter().sum();
    println!("Sum of distances is: {}", sum);

    let mut appearances = HashMap::new();

    for id in right_list.iter() {
        *appearances.entry(id).or_insert(0) += 1;
    }

    let mut similarities = vec![0; left_list.len()];

    for i in 0..left_list.len() {
        let left_id = left_list[i];

        let right_appearances = appearances.get(&left_id).unwrap_or(&0);

        let similarity_score = left_id * right_appearances;
        similarities[i] = similarity_score;
    }

    let similarities_sum: usize = similarities.iter().sum();
    println!("Sum of similarity scores is: {}", similarities_sum);
}
