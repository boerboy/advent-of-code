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
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.parse::<Range>())
        .collect::<Result<Vec<_>>>()?;

    // Parse numbers (second section)
    let numbers: Vec<i64> = numbers_section
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.trim()
                .parse::<i64>()
                .map_err(|e| anyhow!("Failed to parse number '{}': {}", line, e))
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(PuzzleInput { ranges, numbers })
}

fn part1(path: &str) -> Result<i64> {
    let input = parse_input(path)?;

    println!("Parsed {} ranges:", input.ranges.len());
    for range in &input.ranges {
        println!("  {}-{}", range.start, range.end);
    }

    println!("Parsed {} numbers:", input.numbers.len());
    for num in &input.numbers {
        println!("  {}", num);
    }

    // TODO: Implement part 1 solution
    Ok(0)
}

fn part2(path: &str) -> Result<i64> {
    let input = parse_input(path)?;

    // TODO: Implement part 2 solution
    Ok(0)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_parsing() {
        let range: Range = "3-5".parse().unwrap();
        assert_eq!(range.start, 3);
        assert_eq!(range.end, 5);
    }

    #[test]
    fn test_range_contains() {
        let range = Range::new(3, 5);
        assert!(range.contains(3));
        assert!(range.contains(4));
        assert!(range.contains(5));
        assert!(!range.contains(2));
        assert!(!range.contains(6));
    }

    #[test]
    fn test_range_overlaps() {
        let r1 = Range::new(3, 5);
        let r2 = Range::new(10, 14);
        let r3 = Range::new(4, 12);

        assert!(!r1.overlaps(&r2)); // No overlap
        assert!(r1.overlaps(&r3)); // Overlaps at 4-5
        assert!(r2.overlaps(&r3)); // Overlaps at 10-12
    }

    #[test]
    fn test_parse_input() {
        let input = parse_input(&super::test_input()).unwrap();
        assert!(!input.ranges.is_empty());
        assert!(!input.numbers.is_empty());
    }
}
