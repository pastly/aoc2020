//!  Advent of Code 2020: Day 3
//!  Matt Traudt (pastly)
//!  License: WTFPL
//!
//! Much like [day 2](../day2), the idea is to iterate over the lines and filter/map them. As the
//! problem is laid out as if a skier is skiing down the slop (list of lines), it made intuitive
//! sense to construct a solution that followed the skier's path.
//!
//! Parsing of a line happens in [Line::new], and the map is represented as a vector of [Line]s.
//! The interesting "algorithm" stuff is in [count_trees].
//!
use std::io::{self, BufRead};

/// When parsing a line, this char represents a tree. Any other char is a non-tree and safe to ski
/// through. The only other expected char is '.'
const TREE: char = '#';

/// State needed to represent an infinitely long (because it repeats) horizontal line on the map.
#[derive(Debug)]
pub struct Line {
    /// Needed so if we're asked about a position farther to the right than our line length, we can
    /// mod (%) it and tell whether a tree exists there or not.
    width: usize,
    /// A list of the locations (0-indexed) of tree. If an index isn't in this list, then there's
    /// no tree there.
    trees: Vec<usize>,
}

impl Line {
    /// Parse a single line of text into a [Line].
    pub fn new(s: String) -> Line {
        let w = s.len();
        Line {
            width: w,
            trees: s
                .char_indices()
                .filter_map(|(i, c)| if c == TREE { Some(i) } else { None })
                .collect(),
        }
    }

    /// Return whether or not there is a tree at this position (0-indexed). If the given position
    /// is larger than our width, mod it with our width. This is done because the map extends
    /// infinitely in the width direction.
    fn is_tree_at(&self, pos: usize) -> bool {
        let mod_pos = pos % self.width;
        self.trees.contains(&(mod_pos))
    }
}

/// Given a map (list of [Line]s), determine how many trees would be hit were the skier to take a
/// diagonal path in steps with size step_right and step_down. See the in-code comments for details
/// on what the iterator functions are doing.
pub fn count_trees(lines: &[Line], step_right: usize, step_down: usize) -> usize {
    lines
        .iter()
        // Instead of iterating over every line, only look at the ones that match our downward step
        // size. 
        .step_by(step_down)
        // We now only have the lines that we know we will be stepping on. This calculates how far
        // to the right we will be on the line and ads it as something we're iterating over.
        // Assuming step_right is 3 and step_down is 2. We are currently iterating over this:
        //
        //     line_0, line_2, line_4, ...
        // 
        // But after this zip, we'll be iterating over:
        //
        //     (line_0, 0), (line_2, 3), (line_4, 6), ...
        .zip((0..).map(|i| i * step_right))
        // Finally, return *something* if there's a tree at this position on the line, else None.
        // Count how many times we returned something.
        .filter_map(|(line, pos_x)| {
            if line.is_tree_at(pos_x) {
                Some(())
            } else {
                None
            }
        })
        .count()
}

fn part_one(lines: &[Line]) {
    let c = count_trees(lines, 3, 1);
    println!("Part 1: {}", c);
}

fn part_two(lines: &[Line]) {
    let c: usize = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(step_right, step_down)| count_trees(lines, *step_right, *step_down))
        .product();
    println!("Part 2: {}", c);
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let lines: Vec<Line> = handle.lines().map(|l| Line::new(l.unwrap())).collect();
    part_one(&lines);
    part_two(&lines);
}
