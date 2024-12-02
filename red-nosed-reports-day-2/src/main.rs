use std::io::{self, BufRead};

const MIN_DIFF: usize = 1;
const MAX_DIFF: usize = 3;

fn main() {
    let reports: Vec<Vec<usize>> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();

            line.split(" ")
                .map(|num| num.trim().parse().unwrap())
                .collect()
        })
        .collect();

    let safe_reports = reports
        .iter()
        .filter(|report| is_safe_report(report))
        .count();

    println!("Number of safe reports: {}", safe_reports);
}

fn is_safe_report(report: &Vec<usize>) -> bool {
    let mut increasing = true;

    if report[0] == report[1] {
        return false;
    }
    if report[0] > report[1] {
        increasing = false;
    }

    let mut last_level = None;
    let mut peekable_report = report.iter().peekable();

    while let Some(&level) = peekable_report.next() {
        if let Some(last_level) = last_level {
            if increasing && last_level > level {
                return false;
            }

            if !increasing && last_level < level {
                return false;
            }

            if !in_allowed_diff(last_level, level) {
                return false;
            }
        }

        if let Some(&&next_level) = peekable_report.peek() {
            if !in_allowed_diff(next_level, level) {
                return false;
            }
        }

        last_level = Some(level);
    }

    return true;
}

fn in_allowed_diff(first: usize, second: usize) -> bool {
    let diff = if first > second {
        first - second
    } else {
        second - first
    };

    if diff < MIN_DIFF || diff > MAX_DIFF {
        return false;
    }
    true
}
