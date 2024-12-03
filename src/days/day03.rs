use crate::{Solution, SolutionPair};

use regex::Regex;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day03.txt").unwrap();

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let sol1: u64 = re
        .captures_iter(&input)
        .map(|cap| {
            let x = cap[1].parse::<u64>().unwrap();
            let y = cap[2].parse::<u64>().unwrap();
            x * y
        })
        .sum();

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\)").unwrap();
    let mut mult_enabled = true;
    let sol2: u64 = re
        .captures_iter(&input)
        .map(|cap| match &cap[0] {
            "do()" => {
                mult_enabled = true;
                0
            }
            "don't()" => {
                mult_enabled = false;
                0
            }
            _ => {
                if mult_enabled {
                    let x = cap[1].parse::<u64>().unwrap();
                    let y = cap[2].parse::<u64>().unwrap();
                    x * y
                } else {
                    0
                }
            }
        })
        .sum();

    (Solution::from(sol1), Solution::from(sol2))
}
