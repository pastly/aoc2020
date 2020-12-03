//!  Advent of Code 2020: Day 2
//!  Matt Traudt (pastly)
//!  License: WTFPL
//!
//!  Ugly parsing code is in [parse_line]. After parsing the input lines into [Line]s, the vector
//!  of Lines is iterated, filtered, and counted for both part 1 and 2, with the filtering being
//!  the "is a valid password" criteria.
use std::io::{self, BufRead};

/// Helper type to store a parsed line into its important components.
///
///     1. The min occurrences (part 1) or first char index (part 2)
///     2. The max occurrences (part 1) or second char index (part 2)
///     3. The important char
///     4. The String to be tested
pub type Line = (u8, u8, char, String);

/// Parse an input line into its four components. Example lines:
///
///     1-3 a: abcde
///     1-3 b: cdefg
///     2-9 c: ccccccccc
///
/// Advent of Code input is known to be good, so lazy/liberal use of unwrap is used >:)
pub fn parse_line(l: String) -> Line {
    // parse min from front of string to first '-' char
    let min: u8 = l[..l.find('-').unwrap()].parse().unwrap();
    // max is right after first '-' char and until first ' '
    let max: u8 = l[l.find('-').unwrap() + 1..l.find(' ').unwrap()]
        .parse()
        .unwrap();
    // for the rest, iterate over the chars in the String
    let mut chars = l.chars();
    // the given char is right after the first space
    let c = chars.nth(l.find(' ').unwrap() + 1).unwrap();
    // skip two unimportant chars ':' and ' '
    chars.next();
    chars.next();
    // return with the remaining chars collected into a String
    (min, max, c, chars.collect())
}

/// In part one, a String is valid if the important char occurs [min, max] times
fn part_one_is_valid(l: &Line) -> bool {
    let num = l.3.chars().filter(|c| *c == l.2).count() as u8;
    num >= l.0 && num <= l.1
}

/// In part two, a String is valid if the important char occurs in the first index XOR in the
/// second index.
fn part_two_is_valid(l: &Line) -> bool {
    let mut chars = l.3.chars();
    let first = chars.nth(l.0 as usize - 1).unwrap();
    let second = chars.nth(l.1 as usize - l.0 as usize - 1).unwrap();
    (first == l.2) ^ (second == l.2)
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let lines: Vec<Line> = handle.lines().map(|l| parse_line(l.unwrap())).collect();
    let part_one_count = lines.iter().filter(|l| part_one_is_valid(l)).count();
    let part_two_count = lines.iter().filter(|l| part_two_is_valid(l)).count();
    println!("Part 1: {}", part_one_count);
    println!("Part 2: {}", part_two_count);
}
