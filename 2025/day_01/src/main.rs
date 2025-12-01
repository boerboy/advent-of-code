//! Advent of Code 2025 - Day 01
//!
//! Puzzle: https://adventofcode.com/2025/day/1

use anyhow::Result;
use common::prelude::*;
use std::fmt::{Debug, Display, Formatter};
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

const PART_1_STARTING_POSITION: i64 = 50;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct VaultRotation {
    direction: Direction,
    count: i64,
}

impl FromStr for VaultRotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if s.is_empty() {
            return Err("Empty string".to_string());
        }

        let direction_char = s.chars().next().unwrap();
        let count_str = &s[1..];

        let direction = match direction_char {
            'L' => Direction::East, // Left
            'R' => Direction::West, // Right
            _ => return Err(format!("Invalid direction: {}", direction_char)),
        };

        let count = count_str
            .parse::<i64>()
            .map_err(|e| format!("Failed to parse count: {}", e))?;

        Ok(VaultRotation { direction, count })
    }
}

fn read_directions(lines: Vec<String>) -> Vec<VaultRotation> {
    lines
        .iter()
        .map(|line| {
            line.parse::<VaultRotation>()
                .expect("Failed to parse rotation")
        })
        .collect()
}

fn part1(path: &str) -> Result<i64> {
    let lines = read_lines(path)?;
    let vault_directions = read_directions(lines);

    let (_, tracking_map) = vault_directions.iter().try_fold(
        (PART_1_STARTING_POSITION, HashMap::new()),
        |(current_position, mut tracking_map), rotation| -> Result<_, anyhow::Error> {
            // Get direction multiplier (-1 for left/East, 1 for right/West)
            let direction = match rotation.direction {
                dir @ (Direction::East | Direction::West) => Ok(dir.to_coord().x),
                _ => Err(anyhow!("Invalid direction")),
            }?;

            // Calculate new position
            let new_position =
                (current_position + (rotation.count as i64 * direction)).rem_euclid(100);

            // Update tracking map
            *tracking_map.entry(new_position).or_insert(0) += 1;

            Ok((new_position, tracking_map))
        },
    )?;

    // Get max of times dial pointed at n
    let password = tracking_map.values().max().copied().unwrap_or(-1);
    Ok(password)
}

fn part2(path: &str) -> Result<i64> {
    let lines = read_lines(path)?;
    let vault_directions = read_directions(lines);

    let zero_count = vault_directions.iter().try_fold(
        (PART_1_STARTING_POSITION, 0),
        |(current_position, count), rotation| -> Result<_> {
            let direction = match rotation.direction {
                Direction::East => -1,
                Direction::West => 1,
                _ => return Err(anyhow!("Invalid direction")),
            };

            let new_position = (current_position + (rotation.count * direction)).rem_euclid(100);

            // Count how many times we pass through 0 during this rotation
            // How many complete cycles (0-99)?
            let full_cycles = rotation.count / 100;
            let remainder_movement = current_position + (rotation.count % 100) * direction;
            let moved_over_0 = (current_position != 0
                && (remainder_movement > 100 || remainder_movement < 0))
                .then(|| 1)
                .unwrap_or(0);
            let is_zero = (new_position == 0).then(|| 1).unwrap_or(0);
            let passes_through_zero = full_cycles + moved_over_0 + is_zero;
            Ok((new_position, count + passes_through_zero))
        },
    )?;

    Ok(zero_count.1)
}

fn main() -> Result<()> {
    println!("=== Advent of Code 2025 - Day 01 ===\n");
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
