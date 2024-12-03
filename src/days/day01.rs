use itertools::Itertools;

use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // let file = File::open("input/day01.txt").unwrap();
    // let reader = BufReader::new(file);
    let input = read_to_string("input/day01.txt").unwrap();
    let (mut c1, mut c2): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip();

    c1.sort();
    c2.sort();

    let sol1: usize = c1.iter().zip(c2.iter()).map(|(x, y)| x.abs_diff(*y)).sum();

    let frequency: HashMap<usize, usize> =
        c2.iter().dedup_with_count().map(|(x, y)| (*y, x)).collect();

    let sol2: usize = c1
        .into_iter()
        .map(|x| x * frequency.get(&x).map_or(0, |y| *y))
        .sum();

    (Solution::from(sol1), Solution::from(sol2))
}
