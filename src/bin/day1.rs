//! Advent of Code 2020: Day 1
//! Matt Traudt (pastly)
//! License: WTFPL
//!
//! See the function-specific documentation for information on how this works. The main idea is in
//! [find_two_with_sum].
use std::io::{self, BufRead};
use std::cmp::Ordering;

const MAIN_TARGET: u32 = 2020;

/// Find two ints in the given slice that sum to the given target. If none do so, return `None`.
///
/// Assume the list is already sorted.
///
/// Consider the following list and a target value of 13.
///
///     1, 3, 5, 6, 8, 9
///
/// Consider two numbers at a time, starting with the right-most and the left-most. Fix on the
/// right-most, and consider increasingly larger left numbers until the sum of right+left is either
/// the target or more than the target. If it is more than the target, you know that continuing to
/// consider the current right and increasingly larger left values will never produce the correct
/// answer, so start over with the next-largest right value fixed and reset the left value to the
/// smallest.
///
///     1, 9 --> 10, too small,
///     3, 9 --> 12, too small,
///     5, 9 --> 14, too big, adjust
///     1, 8 --> 9, too small,
///     3, 8 --> 11, too small,
///     5, 8 --> 13 DONE
///
///  This code works for the Advent of Code input, but I may not have thought about all possible
///  no-solution exit conditions (i.e. I think maybe there should be more times when `break None;`
///  is needed).
pub fn find_two_with_sum(vals: &[u32], target: u32) -> Option<(u32, u32)> {
    let mut left_idx = 1;
    let mut right_idx = vals.len() - 1;
    loop {
        let sum = vals[left_idx] + vals[right_idx];
        match sum.cmp(&target) {
            Ordering::Equal => break Some((vals[left_idx], vals[right_idx])),
            Ordering::Greater => {
                left_idx = 0;
                right_idx -= 1;
                if right_idx == 0 {
                    break None;
                }
            },
            Ordering::Less => left_idx += 1,
        }
    }
}

/// Part 1 challenge is to find two ints in a list that sum to the target, and produce as the
/// answer their product.
///
/// The work is done in [find_two_with_sum].
fn part_one(vals: &[u32]) {
    let (small, large) = find_two_with_sum(vals, MAIN_TARGET).unwrap();
    println!("Part 1");
    println!("{} + {} = {}", small, large, MAIN_TARGET);
    println!("{} * {} = {}", small, large, small * large);
}

/// Like Part 1, but with three ints.
///
/// The best I could come up with is simply doing the exact same thing as Part 1, but with a target
/// value that is reduced by the first of three ints. Said another way: take the smallest int out
/// of the list, subtract its value from the target, and run the Part 1 algorithm on the remaining
/// ints. If no solution is found, take the next smallest int out of the list and repeat.
///
/// It's probable that I didn't consider all no-solution exit conditions.
fn part_two(vals: &[u32]) {
    let mut fixed_idx = 0;
    let (fixed, left, right) = loop {
        let fixed = vals[fixed_idx];
        if let Some((left, right)) = find_two_with_sum(&vals[fixed_idx + 1..], MAIN_TARGET - fixed)
        {
            break (fixed, left, right);
        }
        fixed_idx += 1;
    };
    println!("Part 2");
    println!("{} + {} + {} = {}", fixed, left, right, MAIN_TARGET);
    println!(
        "{} * {} * {} = {}",
        fixed,
        left,
        right,
        fixed * left * right
    );
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut vals: Vec<u32> = vec![];
    for line in handle.lines() {
        let line = line.unwrap();
        vals.push(line.parse().unwrap());
    }
    vals.sort_unstable();
    part_one(&vals);
    part_two(&vals);
}
