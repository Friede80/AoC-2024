use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let file = File::open("input/day02.txt").unwrap();
    let reader = BufReader::new(file);
    let reports = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let sol1 = reports.iter().filter(|r| is_safe(r)).count();
    let sol2 = reports.iter().filter(|r| is_safe_dampened(r)).count();

    (Solution::from(sol1), Solution::from(sol2))
}

fn is_safe(report: &[i64]) -> bool {
    let diffs = report.iter().tuple_windows().map(|(x, y)| x - y);
    let monotonic = diffs.clone().all(|x| x > 0) || diffs.clone().all(|x| x < 0);
    let good_step = diffs.clone().all(|x| (1..=3).contains(&x.abs()));

    monotonic && good_step
}

fn is_safe_dampened(report: &[i64]) -> bool {
    (0..report.len())
        .into_iter()
        .map(|skip_idx| {
            let mut dampened_report = Vec::with_capacity(report.len() - 1);
            for (i, x) in report.iter().enumerate() {
                if i != skip_idx {
                    dampened_report.push(*x);
                }
            }
            dampened_report
        })
        .any(|r| is_safe(&r))
}
