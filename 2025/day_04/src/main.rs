//! Advent of Code 2025 - Day 04
//!
//! Puzzle: https://adventofcode.com/2025/day/4

use anyhow::Result;
use common::input::read_grid;
use common::{Coord, Grid};

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

fn update_moveable_toilet_paper(grid: Grid<char>, set: char) -> (bool, Grid<char>) {
    let mut grid_clone = grid.clone();
    let mut changed = false;
    grid.coords().for_each(|coord| {
        let current = grid.get(coord).unwrap();
        if *current == '@' {
            let surrounding_toilet_paper: Vec<Coord> = grid
                .neighbors_all(coord)
                .iter()
                .filter(|&&c| *(c.1) == '@')
                .map(|(coord, _)| *coord)
                .collect();
            (surrounding_toilet_paper.len() < 4).then(|| {
                grid_clone.set(coord, set);
                changed = true;
            });
        }
    });
    (changed, grid_clone)
}

fn part1(path: &str) -> Result<i64> {
    let grid = read_grid(path)?;
    let marker = 'x';
    let result = update_moveable_toilet_paper(grid, marker)
        .1
        .iter()
        .filter_map(|c| (*c.1 == marker).then(|| 1))
        .sum::<i64>();
    Ok(result)
}

fn part2(path: &str) -> Result<i64> {
    let mut grid = read_grid(path)?;
    let mut changed = true;
    let marker = 'x';
    while changed {
        let (new_changed, new_grid) = update_moveable_toilet_paper(grid.clone(), marker);
        grid = new_grid;
        changed = new_changed;
    }
    let result = grid
        .coords()
        .map(|coords| grid.get(coords).unwrap())
        .filter_map(|c| (*c == marker).then(|| 1))
        .sum::<i64>();
    Ok(result)
}

fn main() -> Result<()> {
    println!("=== Advent of Code 2025 - Day 04 ===\n");
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
