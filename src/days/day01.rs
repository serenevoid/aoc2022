use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...

    let file = read_to_string("./input/day01.txt").expect("cannot read file");
    let mut calories: Vec<i32> = file
        .split("\n\n")
        .map(|blocks| -> i32 { 
            blocks
                .split("\n")
                .map(|calorie| -> i32 {
                    calorie.parse().unwrap_or(0)
                }).sum()
        })
        .collect();
    calories.sort();
    let sol1 = calories.last().expect("cannot find largets element").clone();
    let mut count = 3;
    let mut last_three_total = 0;
    while count > 0 {
        count -= 1;
        last_three_total += calories.pop().unwrap_or(0);
    }
    let sol2 = last_three_total;

    (Solution::from(sol1), Solution::from(sol2))
}
