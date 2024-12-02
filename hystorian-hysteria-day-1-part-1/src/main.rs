use std::{
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
}
