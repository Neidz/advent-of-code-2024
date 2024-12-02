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

            let (first, second) = line.split_once(" ").unwrap();
            let first_num: usize = first.trim().parse().unwrap();
            let second_num: usize = second.trim().parse().unwrap();

            (first_num, second_num)
        })
        .collect();

    let (mut first_list, mut second_list): (Vec<usize>, Vec<usize>) =
        num_pairs.iter().cloned().unzip();

    first_list.sort();
    second_list.sort();

    assert!(first_list.len() == second_list.len());

    let mut diff = vec![0; first_list.len()];

    for i in 0..first_list.len() {
        let first_id = first_list[i];
        let second_id = second_list[i];

        if first_id >= second_id {
            diff[i] = first_id - second_id
        } else {
            diff[i] = second_id - first_id
        }
    }

    let sum: usize = diff.iter().sum();

    println!("Sum of distances is: {}", sum);
}
