//! Advent of Code 2025 - Day 12
//!
//! Puzzle: https://adventofcode.com/2025/day/12

use common::prelude::*;
use std::collections::HashSet;

// ============================================================================
// Input Parsing
// ============================================================================

fn resource(filename: &str) -> String {
    format!("{}/resources/{}", env!("CARGO_MANIFEST_DIR"), filename)
}
fn input() -> String { resource("input.txt") }
fn test_input() -> String { resource("test.txt") }

type Shape = Vec<(i32, i32)>;

fn parse_input(path: &str) -> Result<(Vec<Vec<Shape>>, Vec<(usize, usize, Vec<usize>)>)> {
    let content = std::fs::read_to_string(path)?;
    let sections: Vec<&str> = content.split("\n\n").collect();
    
    let mut base_shapes: Vec<Shape> = Vec::new();
    
    for section in sections.iter() {
        let lines: Vec<&str> = section.lines().collect();
        if lines.is_empty() { continue; }
        
        if let Some(first_line) = lines.first() {
            if first_line.contains(':') && !first_line.contains('x') {
                let mut shape: Shape = Vec::new();
                for (row, line) in lines.iter().skip(1).enumerate() {
                    for (col, ch) in line.chars().enumerate() {
                        if ch == '#' {
                            shape.push((row as i32, col as i32));
                        }
                    }
                }
                if !shape.is_empty() {
                    base_shapes.push(shape);
                }
            }
        }
    }
    
    let all_orientations: Vec<Vec<Shape>> = base_shapes
        .iter()
        .map(|shape| generate_orientations(shape))
        .collect();
    
    let mut regions: Vec<(usize, usize, Vec<usize>)> = Vec::new();
    
    for section in sections.iter() {
        for line in section.lines() {
            if line.contains('x') && line.contains(':') {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() != 2 { continue; }
                
                let dims: Vec<&str> = parts[0].trim().split('x').collect();
                if dims.len() != 2 { continue; }
                
                let width: usize = dims[0].parse().unwrap_or(0);
                let height: usize = dims[1].parse().unwrap_or(0);
                
                let counts: Vec<usize> = parts[1]
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
                
                if width > 0 && height > 0 && !counts.is_empty() {
                    regions.push((width, height, counts));
                }
            }
        }
    }
    
    Ok((all_orientations, regions))
}

fn normalize(shape: &Shape) -> Shape {
    if shape.is_empty() { return shape.clone(); }
    
    let min_row = shape.iter().map(|(r, _)| *r).min().unwrap();
    let min_col = shape.iter().map(|(_, c)| *c).min().unwrap();
    
    let mut normalized: Shape = shape.iter()
        .map(|(r, c)| (r - min_row, c - min_col))
        .collect();
    normalized.sort();
    normalized
}

fn rotate_90(shape: &Shape) -> Shape {
    normalize(&shape.iter().map(|(r, c)| (*c, -r)).collect())
}

fn flip_horizontal(shape: &Shape) -> Shape {
    normalize(&shape.iter().map(|(r, c)| (*r, -c)).collect())
}

fn generate_orientations(shape: &Shape) -> Vec<Shape> {
    let mut orientations: HashSet<Vec<(i32, i32)>> = HashSet::new();
    let mut current = normalize(shape);
    
    for _ in 0..4 {
        orientations.insert(current.clone());
        current = rotate_90(&current);
    }
    
    current = flip_horizontal(&normalize(shape));
    for _ in 0..4 {
        orientations.insert(current.clone());
        current = rotate_90(&current);
    }
    
    orientations.into_iter().collect()
}

// ============================================================================
// Part 1: Efficient bin packing with bitmask grid
// ============================================================================

fn can_place_bitmask(grid: &[u64], bitmask: &[u64]) -> bool {
    for (row, &mask) in bitmask.iter().enumerate() {
        if grid[row] & mask != 0 {
            return false;
        }
    }
    true
}

fn place_bitmask(grid: &mut [u64], bitmask: &[u64]) {
    for (row, &mask) in bitmask.iter().enumerate() {
        grid[row] |= mask;
    }
}

fn unplace_bitmask(grid: &mut [u64], bitmask: &[u64]) {
    for (row, &mask) in bitmask.iter().enumerate() {
        grid[row] &= !mask;
    }
}

/// All valid placements for each shape, indexed by position for pruning
fn compute_all_placements(
    orientations: &[Vec<Shape>],
    width: usize,
    height: usize,
) -> Vec<Vec<(usize, Vec<u64>)>> {
    // placements[shape_idx] = list of (position, bitmask)
    let mut placements: Vec<Vec<(usize, Vec<u64>)>> = vec![Vec::new(); orientations.len()];
    
    for (shape_idx, shape_orientations) in orientations.iter().enumerate() {
        for shape in shape_orientations.iter() {
            for base_row in 0..height as i32 {
                for base_col in 0..width as i32 {
                    let mut valid = true;
                    let mut bitmask: Vec<u64> = vec![0; height];
                    let mut min_pos = usize::MAX;
                    
                    for (dr, dc) in shape {
                        let nr = base_row + dr;
                        let nc = base_col + dc;
                        
                        if nr < 0 || nc < 0 || nr >= height as i32 || nc >= width as i32 {
                            valid = false;
                            break;
                        }
                        
                        let pos = nr as usize * width + nc as usize;
                        min_pos = min_pos.min(pos);
                        bitmask[nr as usize] |= 1u64 << nc;
                    }
                    
                    if valid {
                        placements[shape_idx].push((min_pos, bitmask));
                    }
                }
            }
        }
        // Sort by position for consistent ordering
        placements[shape_idx].sort_by_key(|(pos, _)| *pos);
        // Deduplicate (same bitmask from different orientations)
        placements[shape_idx].dedup_by(|a, b| a.1 == b.1);
    }
    
    placements
}

fn solve_packing(
    grid: &mut Vec<u64>,
    remaining: &mut Vec<usize>,
    placements: &[Vec<(usize, Vec<u64>)>],
    width: usize,
    height: usize,
) -> bool {
    // Find first shape type with remaining pieces
    let shape_idx = match remaining.iter().position(|&c| c > 0) {
        Some(idx) => idx,
        None => return true,  // All pieces placed!
    };
    
    // Try each placement for this shape
    for (_, bitmask) in &placements[shape_idx] {
        if can_place_bitmask(grid, bitmask) {
            place_bitmask(grid, bitmask);
            remaining[shape_idx] -= 1;
            
            if solve_packing(grid, remaining, placements, width, height) {
                return true;
            }
            
            remaining[shape_idx] += 1;
            unplace_bitmask(grid, bitmask);
        }
    }
    
    false
}

fn can_fit_all(width: usize, height: usize, counts: &[usize], all_orientations: &[Vec<Shape>]) -> bool {
    // Quick check: total cells needed vs available
    let total_cells: usize = counts.iter()
        .enumerate()
        .map(|(idx, &count)| {
            if idx < all_orientations.len() && !all_orientations[idx].is_empty() {
                count * all_orientations[idx][0].len()
            } else { 0 }
        })
        .sum();
    
    if total_cells > width * height {
        return false;  // Can't fit more cells than available
    }
    
    if counts.iter().all(|&c| c == 0) {
        return true;  // No pieces to place
    }
    
    let placements = compute_all_placements(all_orientations, width, height);
    let mut grid: Vec<u64> = vec![0; height];
    let mut remaining: Vec<usize> = counts.to_vec();
    
    solve_packing(&mut grid, &mut remaining, &placements, width, height)
}

fn part1(path: &str) -> Result<u64> {
    let (all_orientations, regions) = parse_input(path)?;
    
    let count = regions.iter()
        .filter(|(width, height, counts)| {
            can_fit_all(*width, *height, counts, &all_orientations)
        })
        .count();
    
    Ok(count as u64)
}

// ============================================================================
// Main
// ============================================================================

fn main() -> Result<()> {
    println!("=== Advent of Code 2025 - Day 12 ===\n");
    
    println!("--- Test Input ---");
    match part1(&test_input()) {
        Ok(result) => println!("Part 1 (test): {}", result),
        Err(e) => println!("Part 1 (test) error: {}", e),
    }

    println!("\n--- Puzzle Input ---");
    match part1(&input()) {
        Ok(result) => println!("Part 1: {}", result),
        Err(e) => println!("Part 1 error: {}", e),
    }

    Ok(())
}
