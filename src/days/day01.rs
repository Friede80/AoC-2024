use itertools::Itertools;

use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let file = File::open("input/day01.txt").unwrap();
    let reader = BufReader::new(file);
    let (mut c1, mut c2): (Vec<_>, Vec<_>) = reader
        .lines()
        .map(|line| parse_tuple(&line.unwrap()))
        .unzip();

    c1.sort();
    c2.sort();

    let sol1: u64 = c1.iter().zip(c2.iter()).map(|(x, y)| x.abs_diff(*y)).sum();

    let frequency: HashMap<u64, usize> =
        c2.iter().dedup_with_count().map(|(x, y)| (*y, x)).collect();

    let sol2: u64 = c1
        .into_iter()
        .map(|x| x * (frequency.get(&x).map_or(0, |y| *y) as u64))
        .sum();

    (Solution::from(sol1), Solution::from(sol2))
}

fn parse_tuple(line: &str) -> (u64, u64) {
    let mut iter = line.split_whitespace().map(|x| x.parse::<u64>());
    (iter.next().unwrap().unwrap(), iter.next().unwrap().unwrap())
}
