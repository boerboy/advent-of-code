//! Advent of Code 2025 - Day 08
//!
//! Puzzle: https://adventofcode.com/2025/day/8

use anyhow::Result;
use common::grid::VisitedTracker;
use common::input::{read_csv, read_grid};
use common::prelude::Itertools;

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

fn distance(point1: &Vec<i64>, point2: &Vec<i64>) -> f64 {
    point1
        .iter()
        .zip(point2)
        .map(|(coord1, coord2)| {
            let delta = coord2 - coord1;
            (delta * delta) as f64
        })
        .sum::<f64>()
        .sqrt()
}

fn distances(points: &Vec<Vec<i64>>) -> Vec<(usize, usize, f64)> {
    points
        .iter()
        .enumerate()
        .flat_map(|(idx, position1)| {
            points[idx + 1..]
                .iter()
                .enumerate()
                .map(move |(j, position2)| (idx, idx + j + 1, distance(position1, position2)))
        })
        .collect()
}

fn create_circuits(distances: Vec<(usize, usize, f64)>) -> Vec<i64>{
 ???
}

fn part1(path: &str, n: usize) -> Result<i64> {
    let input: Vec<Vec<i64>> =
        read_csv(path, b',').map_err(|err| anyhow::anyhow!(err.to_string()))?;
    let distances = distances(&input);
    let circuits = create_circuits(distances);
    let result = circuits.iter().sorted_by(|a, b| b.cmp(&a)).take(n).sum();
    Ok(result)
}

fn part2(path: &str) -> Result<i64> {
    let input = read_grid(path)?;
    Ok(0)
}

fn main() -> Result<()> {
    println!("=== Advent of Code 2025 - Day 08 ===\n");
    // Run with test input first
    println!("--- Test Input ---");
    match part1(&test_input(), 3) {
        Ok(result) => println!("Part 1 (test): {}", result),
        Err(e) => println!("Part 1 (test) error: {}", e),
    }
    match part2(&test_input()) {
        Ok(result) => println!("Part 2 (test): {}", result),
        Err(e) => println!("Part 2 (test) error: {}", e),
    }
    //
    // println!("\n--- Puzzle Input ---");
    // match part1(&input(), 1000) {
    //     Ok(result) => println!("Part 1: {}", result),
    //     Err(e) => println!("Part 1 error: {}", e),
    // }
    match part2(&input()) {
        Ok(result) => println!("Part 2: {}", result),
        Err(e) => println!("Part 2 error: {}", e),
    }

    Ok(())
}
