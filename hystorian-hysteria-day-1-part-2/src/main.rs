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

    let (left_list, right_list): (Vec<usize>, Vec<usize>) = num_pairs.iter().cloned().unzip();

    assert!(left_list.len() == right_list.len());

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
