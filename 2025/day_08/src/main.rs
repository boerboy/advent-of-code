//! Advent of Code 2025 - Day 08
//!
//! Puzzle: https://adventofcode.com/2025/day/8

use anyhow::Result;
use common::input::read_csv;
use common::prelude::Itertools;
use std::collections::HashMap;

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

/// Union-Find data structure for tracking connected components
struct UnionFind {
    parent: Vec<usize>,
    num_circuits: usize,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            num_circuits: size,
        }
    }

    fn find(&mut self, idx: usize) -> usize {
        if self.parent[idx] != idx {
            self.parent[idx] = self.find(self.parent[idx]);
        }
        self.parent[idx]
    }

    /// Union two elements, returns true if they were in different circuits
    fn union(&mut self, i: usize, j: usize) -> bool {
        let (root_i, root_j) = (self.find(i), self.find(j));
        if root_i != root_j {
            self.parent[root_i] = root_j;
            self.num_circuits -= 1;
            true
        } else {
            false
        }
    }

    fn circuits(&mut self) -> Vec<Vec<usize>> {
        let size = self.parent.len();
        (0..size)
            .fold(HashMap::new(), |mut map, idx| {
                map.entry(self.find(idx))
                    .or_insert_with(Vec::new)
                    .push(idx);
                map
            })
            .into_values()
            .collect()
    }
}

/// Calculate Euclidean distance between two 3D points
fn distance(point1: &[i64], point2: &[i64]) -> f64 {
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

/// Calculate distances between all pairs of points
fn distances(points: &[Vec<i64>]) -> Vec<(usize, usize, f64)> {
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

/// Sort distances by length (ascending)
fn sorted_distances(points: &[Vec<i64>]) -> Vec<(usize, usize, f64)> {
    distances(points)
        .into_iter()
        .sorted_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
        .collect()
}

fn part1(path: &str, n: usize) -> Result<i64> {
    let input: Vec<Vec<i64>> =
        read_csv(path, b',').map_err(|err| anyhow::anyhow!(err.to_string()))?;

    let shortest = sorted_distances(&input).into_iter().take(n);

    let mut uf = UnionFind::new(input.len());
    shortest.for_each(|(i, j, _)| {
        uf.union(i, j);
    });

    let result = uf
        .circuits()
        .iter()
        .map(|x| x.len() as i64)
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .product();

    Ok(result)
}

fn part2(path: &str) -> Result<i64> {
    let input: Vec<Vec<i64>> =
        read_csv(path, b',').map_err(|err| anyhow::anyhow!(err.to_string()))?;

    let all_distances = sorted_distances(&input);

    let mut uf = UnionFind::new(input.len());
    let (idx1, idx2) = all_distances
        .iter()
        .find(|&&(i, j, _)| uf.union(i, j) && uf.num_circuits == 1)
        .map(|&(i, j, _)| (i, j))
        .ok_or_else(|| anyhow::anyhow!("No final connection found"))?;

    Ok(input[idx1][0] * input[idx2][0])
}

fn main() -> Result<()> {
    println!("=== Advent of Code 2025 - Day 08 ===\n");
    // Fell a bit behind on time so used a decent amount of LLM to get to solution
    // Run with test input first
    println!("--- Test Input ---");
    match part1(&test_input(), 10) {
        Ok(result) => println!("Part 1 (test): {}", result),
        Err(e) => println!("Part 1 (test) error: {}", e),
    }
    match part2(&test_input()) {
        Ok(result) => println!("Part 2 (test): {}", result),
        Err(e) => println!("Part 2 (test) error: {}", e),
    }
    // Run with input
    println!("\n--- Puzzle Input ---");
    match part1(&input(), 1000) {
        Ok(result) => println!("Part 1: {}", result),
        Err(e) => println!("Part 1 error: {}", e),
    }
    match part2(&input()) {
        Ok(result) => println!("Part 2: {}", result),
        Err(e) => println!("Part 2 error: {}", e),
    }

    Ok(())
}
