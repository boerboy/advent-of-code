//! Advent of Code 2025 - Day 05
//!
//! Puzzle: https://adventofcode.com/2025/day/5

use anyhow::{anyhow, Result};
use common::input::{read_string, split_sections};
use common::prelude::*;

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

/// Parsed input containing ranges and numbers
#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub ranges: Vec<Range>,
    pub numbers: Vec<i64>,
}

/// Parse the puzzle input from a file
fn parse_input(path: &str) -> Result<PuzzleInput> {
    let content = read_string(path)?;
    let (ranges_section, numbers_section) = split_sections(&content);

    // Parse ranges (first section)
    let ranges: Vec<Range> = ranges_section
        .lines()
        .map(|line| line.parse::<Range>())
        .collect::<Result<Vec<_>>>()?;

    // Parse numbers (second section)
    let numbers: Vec<i64> = numbers_section
        .lines()
        .map(|line| {
            line.trim()
                .parse::<i64>()
                .map_err(|e| anyhow!("Failed to parse number '{}': {}", line, e))
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(PuzzleInput { ranges, numbers })
}

fn find_fresh_ingredient_count(input: PuzzleInput) -> i64 {
    input
        .numbers
        .iter()
        .filter(|&&n| {
            input
                .ranges
                .iter()
                .find(|range| range.contains(n))
                .is_some()
        })
        .collect_vec()
        .len() as i64
}

fn find_fresh_ingredient_id_count(puzzle_input: PuzzleInput) -> i64 {
    let mut acc = HashSet::<i64>::new();
    puzzle_input.ranges.sort_by(|a, b|)
    puzzle_input.ranges.iter().for_each(|range| {
        range.range_iter().for_each(|i| {
            acc.insert(i);
        });
    });
    acc.len() as i64
}

fn part1(path: &str) -> Result<i64> {
    let input = parse_input(path)?;
    let result = find_fresh_ingredient_count(input);
    Ok(result)
}

fn part2(path: &str) -> Result<i64> {
    let input = parse_input(path)?;
    let result = find_fresh_ingredient_id_count(input);
    Ok(result)
}

fn main() -> Result<()> {
    println!("=== Advent of Code 2025 - Day 05 ===\n");
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

    // println!("\n--- Puzzle Input ---");
    // match part1(&input()) {
    //     Ok(result) => println!("Part 1: {}", result),
    //     Err(e) => println!("Part 1 error: {}", e),
    // }
    // match part2(&input()) {
    //     Ok(result) => println!("Part 2: {}", result),
    //     Err(e) => println!("Part 2 error: {}", e),
    // }

    Ok(())
}
