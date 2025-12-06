//! Advent of Code 2025 - Day 06
//!
//! Puzzle: https://adventofcode.com/2025/day/6

use anyhow::Result;
use common::input::read_grid;
use common::prelude::Itertools;
use common::range::Range;
use common::Grid;

/// Get path relative to this crate's directory
fn resource(filename: &str) -> String {
    format!("{}/resources/{}", env!("CARGO_MANIFEST_DIR"), filename)
}

fn input() -> String {
    resource("input.txt")
}
fn test_input() -> String {
    resource("test.txt")
}

/// A single math problem with numbers and an operation
#[derive(Clone, Debug)]
pub struct Problem {
    pub numbers: Vec<String>,
    pub operation: char, // '*' or '+'
}

/// Parsed input containing all problems from the worksheet
#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub problems: Vec<Problem>,
}

fn parse_input(path: &str) -> Result<PuzzleInput> {
    let grid = read_grid(path)?;
    let operator_row_idx = grid.height() - 1;
    let operator_row = grid.row(operator_row_idx).unwrap();

    // Find operator positions - each marks the start of a problem column
    let operator_positions: Vec<(usize, char)> = operator_row
        .iter()
        .enumerate()
        .filter(|(_, &c)| c != ' ')
        .map(|(i, &c)| (i, c))
        .collect();
    let operator_ranges: Vec<(char, Range)> = operator_positions
        .clone()
        .iter()
        .sorted_by(|&&a, &&b| a.0.cmp(&b.0))
        .map(|x| *x)
        .collect::<Vec<(usize, char)>>()
        .windows(2)
        .map(|window| {
            let (pos_x, char) = window[0];
            let (pos_y, _) = window[1];
            (
                char,
                Range {
                    start: pos_x as i64,
                    end: (pos_y - 1) as i64,
                },
            )
        })
        .collect();
    let last_range = operator_ranges.last().unwrap().clone();
    let last_problem_range = (
        operator_row[(last_range.1.end + 1) as usize],
        Range {
            start: last_range.1.end + 1,
            end: operator_row.len() as i64 - 1,
        },
    );
    let operator_ranges = [operator_ranges, vec![last_problem_range]].concat();
    let problems = operator_ranges
        .iter()
        .map(|(char, range)| {
            let numbers: Vec<String> = grid
                .rows()
                .map(|row| {
                    range
                        .range_iter()
                        .map(|idx| row[idx as usize])
                        .collect::<String>()
                })
                .collect();
            Problem {
                operation: *char,
                numbers,
            }
        })
        .collect();

    Ok(PuzzleInput { problems })
}

fn get_operation(op: char) -> fn(i64, i64) -> i64 {
    match op {
        '+' => |x, y| x + y,
        '*' => |x, y| if x == 0 { 1 * y } else { x * y },
        _ => panic!("Unknown operator: {}", op),
    }
}

fn part1(path: &str) -> Result<i64> {
    let input = parse_input(path)?;
    let result: i64 = input
        .problems
        .iter()
        .map(|problem| {
            problem
                .numbers
                .iter()
                .filter_map(|i| i.clone().trim().parse::<i64>().ok())
                .fold(0, |acc, i| get_operation(problem.operation)(acc, i))
        })
        .sum();
    Ok(result)
}

fn part2(path: &str) -> Result<i64> {
    let input = parse_input(path)?;
    let result: i64 = input
        .problems
        .iter()
        .map(|problem| {
            let numbers: Vec<i64> = Grid::from_lines(problem.clone().numbers)
                .columns()
                .filter_map(|col| {
                    col.iter()
                        .map(|c| **c)
                        .filter(|&c| c !='*' && c !='+')
                        .collect::<String>()
                        .trim()
                        .parse::<i64>()
                        .ok()
                })
                .collect();
            numbers
                .iter()
                .fold(0, |acc, i| get_operation(problem.operation)(acc, *i))
        })
        .sum();
    Ok(result)
}

fn main() -> Result<()> {
    println!("=== Advent of Code 2025 - Day 06 ===\n");
    // Run with test input first
    println!("--- Test Input ---");
    match part1(&test_input()) {
        Ok(result) => println!("Part 1 (test): {}", result),
        Err(e) => println!("Part 1 (test) error: {}", e),
    }
    match part2(&test_input()) {
        Ok(result) => println!("Part 2 (test): {}", result),
        Err(e) => println!("Part 2 (test) error: {}", e),
    }

    println!("\n--- Puzzle Input ---");
    match part1(&input()) {
        Ok(result) => println!("Part 1: {}", result),
        Err(e) => println!("Part 1 error: {}", e),
    }
    match part2(&input()) {
        Ok(result) => println!("Part 2: {}", result),
        Err(e) => println!("Part 2 error: {}", e),
    }

    Ok(())
}
