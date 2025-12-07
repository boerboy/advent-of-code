//! Advent of Code 2025 - Day 07
//!
//! Puzzle: https://adventofcode.com/2025/day/7

use anyhow::{anyhow, Result};
use common::grid::VisitedTracker;
use common::input::read_grid;
use common::{Coord, Grid};
use std::collections::HashSet;
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

fn find_splits(grid: Grid<char>) -> Option<i64> {
    fn rec(
        grid: &Grid<char>,
        current: Coord,
        acc: &mut HashSet<Coord>,
        visited: &mut VisitedTracker,
    ) {
        let next_value = current + Coord::SOUTH;
        if !next_value.in_grid_bounds(grid) || !visited.visit(next_value) {
            ()
        } else {
            let &next_char = grid.get(next_value).unwrap();
            if next_char == '^' {
                acc.insert(next_value);
                rec(grid, next_value + Coord::WEST, acc, visited);
                rec(grid, next_value + Coord::EAST, acc, visited);
            } else {
                rec(grid, next_value, acc, visited)
            }
        }
    }
    let start = grid.find(&'S')?;
    let visited = &mut grid.visited_tracker();
    let acc = &mut HashSet::new();
    rec(&grid, start, acc, visited);
    Some(acc.iter().len() as i64)
}

fn find_quantum_splits(grid: Grid<char>) -> Option<i64> {
    fn rec(
        grid: &Grid<char>,
        current: Coord,
        acc: &mut HashSet<Coord>,
        visited: &mut VisitedTracker,
    ) {
        let next_value = current + Coord::SOUTH;
        if !next_value.in_grid_bounds(grid) || !visited.visit(next_value) {
            ()
        } else {
            let &next_char = grid.get(next_value).unwrap();
            if next_char == '^' {
                acc.insert(next_value);
                rec(grid, next_value + Coord::WEST, acc, visited);
                rec(grid, next_value + Coord::EAST, acc, visited);
            } else {
                rec(grid, next_value, acc, visited)
            }
        }
    }
    let start = grid.find(&'S')?;
    let visited = &mut grid.visited_tracker();
    let acc = &mut HashSet::new();
    rec(&grid, start, acc, visited);
    println!("filtered: {:?}", grid.cells.iter().filter(|row| row.iter().filter(|c| )c == '^' && visited.is_visited(coord)).collect_vec().len());
    println!("{:?}", visited);
    Some(acc.iter().len() as i64)
}

fn part1(path: &str) -> Result<i64> {
    let input = read_grid(path)?;
    let result = find_splits(input)
        .map(|i| Ok(i))
        .unwrap_or(Err(anyhow!("No result")))?;
    Ok(result)
}

fn part2(path: &str) -> Result<i64> {
    let input = read_grid(path)?;
    let result = find_quantum_splits(input)
        .map(|i| Ok(i))
        .unwrap_or(Err(anyhow!("No result")))?;
    Ok(result)
}

fn main() -> Result<()> {
    println!("=== Advent of Code 2025 - Day 07 ===\n");
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
