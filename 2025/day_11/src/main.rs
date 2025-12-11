//! Advent of Code 2025 - Day 11
//!
//! Puzzle: https://adventofcode.com/2025/day/11

use common::prelude::*;

// ============================================================================
// Input Parsing
// ============================================================================

fn resource(filename: &str) -> String {
    format!("{}/resources/{}", env!("CARGO_MANIFEST_DIR"), filename)
}
fn input() -> String { resource("input.txt") }
fn test_input() -> String { resource("test.txt") }
fn test_input2() -> String { resource("test2.txt") }

fn parse_graph(path: &str) -> Result<HashMap<String, Vec<String>>> {
    let lines = read_lines(path)?;
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    
    for line in lines.iter().filter(|l| !l.is_empty()) {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 { continue; }
        
        let node = parts[0].trim().to_string();
        let children: Vec<String> = parts[1]
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        
        graph.insert(node, children);
    }
    
    Ok(graph)
}

// ============================================================================
// Part 1: Count all paths from "you" to "out" using DFS
// ============================================================================

fn part1(path: &str) -> Result<u64> {
    let graph = parse_graph(path)?;
    let count = dfs_count_paths(
        "you".to_string(),
        "out".to_string(),
        |node| graph.get(node).cloned().unwrap_or_default(),
    );
    Ok(count)
}

// ============================================================================
// Part 2: Count paths from "svr" to "out" visiting both "dac" and "fft"
// ============================================================================

fn count_paths_through(graph: &HashMap<String, Vec<String>>, from: &str, to: &str) -> u64 {
    dfs_count_paths(
        from.to_string(),
        to.to_string(),
        |node| graph.get(node).cloned().unwrap_or_default(),
    )
}

fn part2(path: &str) -> Result<u64> {
    let graph = parse_graph(path)?;

    let dac_then_fft = count_paths_through(&graph, "svr", "dac")
        * count_paths_through(&graph, "dac", "fft")
        * count_paths_through(&graph, "fft", "out");
    
    let fft_then_dac = count_paths_through(&graph, "svr", "fft")
        * count_paths_through(&graph, "fft", "dac")
        * count_paths_through(&graph, "dac", "out");
    
    Ok(dac_then_fft + fft_then_dac)
}

// ============================================================================
// Main
// ============================================================================

fn main() -> Result<()> {
    println!("=== Advent of Code 2025 - Day 11 ===\n");
    
    println!("--- Test Input ---");
    match part1(&test_input()) {
        Ok(result) => println!("Part 1 (test): {}", result),
        Err(e) => println!("Part 1 (test) error: {}", e),
    }
    match part2(&test_input2()) {
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
