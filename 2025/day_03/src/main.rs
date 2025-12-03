//! Advent of Code 2025 - Day 03
//!
//! Puzzle: https://adventofcode.com/2025/day/3

use anyhow::Result;
use common::input::read_lines;

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

fn read_input(path: &str) -> Result<Vec<Vec<i64>>> {
    read_lines(path).map(|lines| {
        lines
            .iter()
            .map(|s| {
                s.chars()
                    .filter_map(|c| c.to_digit(10))
                    .map(|d| d as i64)
                    .collect()
            })
            .collect()
    })
}

// Part 1, first iteration. Eventually found out greedy stack fits both and is better.
#[allow(dead_code)]
fn find_bank_max(battery_bank: &Vec<i64>) -> Result<i64> {
    let mut bank_iterable = battery_bank.iter().peekable();
    let mut first: i64 = -1;
    let mut second: i64 = -1;
    while let Some(&i) = bank_iterable.next() {
        if let Some(_) = bank_iterable.peek() {
            if i > first {
                first = i;
                second = -1;
            } else if i > second {
                second = i;
            }
        } else {
            if i > second {
                second = i;
            }
        }
    }
    let combined = format!("{}{}", first, second).parse::<i64>()?;
    Ok(combined)
}

fn find_bank_max_greedy_n(bank: &Vec<i64>, n: usize) -> Result<i64> {
    let mut drop = bank.len() - n;
    let mut stack: Vec<i64> = Vec::new();
    bank.iter().for_each(|d| {
        while drop > 0 && !stack.is_empty() && d > &*stack.last().unwrap_or(&-1i64) {
            stack.pop();
            drop -= 1;
        }
        stack.push(*d);
    });
    let result: i64 = stack
        .iter()
        .take(n)
        .map(|d| d.to_string())
        .collect::<String>()
        .parse::<i64>()?;
    Ok(result)
}

fn part1(path: &str) -> Result<i64> {
    let banks = read_input(path)?;
    let bank_maxs = banks
        .iter()
        .map(|bank| find_bank_max_greedy_n(bank, 2))
        .collect::<Result<Vec<i64>>>()?;
    let result = bank_maxs.iter().sum::<i64>();
    Ok(result)
}

fn part2(path: &str) -> Result<i64> {
    let banks = read_input(path)?;
    let bank_maxs = banks
        .iter()
        .map(|bank| find_bank_max_greedy_n(bank, 12))
        .collect::<Result<Vec<i64>>>()?;
    let result = bank_maxs.iter().sum::<i64>();
    Ok(result)
}

fn main() -> Result<()> {
    println!("=== Advent of Code 2025 - Day 03 ===\n");
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
