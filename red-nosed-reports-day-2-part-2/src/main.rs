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
        .filter(|report| is_safe_report(report, true))
        .count();

    println!("Number of safe reports: {}", safe_reports);
}

fn is_safe_report(report: &Vec<usize>, can_correct: bool) -> bool {
    let report_len = report.len();
    let mut increasing: Option<bool> = None;

    for i in 0..report_len {
        let level = report[i];
        let prev_level = if i > 0 { Some(report[i - 1]) } else { None };
        let next_level = if (i + 1) < report_len {
            Some(report[i + 1])
        } else {
            None
        };

        if let Some(prev_level) = prev_level {
            if let Some(increasing) = increasing {
                if increasing && prev_level > level {
                    return correct_and_check(report, can_correct);
                }

                if !increasing && prev_level < level {
                    return correct_and_check(report, can_correct);
                }
            } else {
                increasing = Some(prev_level < level);
            }

            if !in_allowed_diff(prev_level, level) {
                return correct_and_check(report, can_correct);
            }
        }

        if let Some(next_level) = next_level {
            if !in_allowed_diff(next_level, level) {
                return correct_and_check(report, can_correct);
            }
        }
    }

    return true;
}

fn correct_and_check(report: &Vec<usize>, can_correct: bool) -> bool {
    if !can_correct {
        return false;
    }

    for i in 0..report.len() {
        let mut corrected_report = report.clone();
        corrected_report.remove(i);

        if is_safe_report(&corrected_report, false) {
            return true;
        }
    }
    false
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
