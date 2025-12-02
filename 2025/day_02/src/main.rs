//! Advent of Code 2025 - Day 02
//!
//! Puzzle: https://adventofcode.com/2025/day/2

use anyhow::Result;
use common::input::read_delimited;
use std::fmt::Debug;
use std::str::FromStr;

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

#[derive(Debug, PartialEq)]
struct Range {
    start: i64,
    end: i64,
}

impl FromStr for Range {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(r"^(\d+)-(\d+)$").map_err(|e| format!("Regex error: {}", e))?;
        let caps = re
            .captures(s)
            .ok_or_else(|| format!("Invalid range format: {}", s))?;
        let start = caps[1]
            .parse::<i64>()
            .map_err(|e| format!("Failed to parse start: {}", e))?;
        let end = caps[2]
            .parse::<i64>()
            .map_err(|e| format!("Failed to parse end: {}", e))?;
        Ok(Range { start, end })
    }
}

fn is_invalid_id(id: i64, repetitions: i64) -> bool {
    let str_id = id.to_string();
    let str_len = str_id.len();
    str_id.starts_with('0').then(|| true).unwrap_or(
        (str_len % repetitions as usize == 0)
            .then(|| {
                let pattern_len = str_len / repetitions as usize;
                let pattern = &str_id[..pattern_len];
                (0..repetitions).all(|i| {
                    let start = i as usize * pattern_len;
                    let end = start + pattern_len;
                    &str_id[start..end] == pattern
                })
            })
            .unwrap_or(false),
    )
}

fn part1(path: &str) -> Result<i64> {
    let ranges = read_delimited::<Range, &str>(path, ',')?;
    let sum = ranges
        .iter()
        .map(|range| {
            (range.start..=range.end)
                .filter(|&id| is_invalid_id(id, 2))
                .sum::<i64>()
        })
        .sum();
    Ok(sum)
}

fn part2(path: &str) -> Result<i64> {
    let ranges = read_delimited::<Range, &str>(path, ',')?;
    let sum = ranges
        .iter()
        .map(|range| {
            (range.start..=range.end)
                // Just chose random number of repetitions to search for, could add a minor optimization function based off range boundary lengths
                .filter(|&id| (2i64..=10).find(|&reps| is_invalid_id(id, reps)).is_some())
                .sum::<i64>()
        })
        .sum();
    Ok(sum)
}

fn main() -> Result<()> {
    println!("=== Advent of Code 2025 - Day 02 ===\n");
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
