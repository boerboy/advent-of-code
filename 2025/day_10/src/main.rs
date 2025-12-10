//! Advent of Code 2025 - Day 10
//!
//! Puzzle: https://adventofcode.com/2025/day/10

use anyhow::{Context, Result};
use common::input::read_lines;
use std::collections::HashSet;

// ============================================================================
// Input Parsing (shared)
// ============================================================================

fn resource(filename: &str) -> String {
    format!("{}/resources/{}", env!("CARGO_MANIFEST_DIR"), filename)
}
fn input() -> String { resource("input.txt") }
fn test_input() -> String { resource("test.txt") }

#[derive(Debug, Clone)]
struct Pattern { lights: Vec<bool> }

impl Pattern {
    fn from_str(s: &str) -> Result<Self> {
        let lights = s.chars()
            .map(|c| match c {
                '#' => Ok(true),
                '.' => Ok(false),
                _ => anyhow::bail!("Invalid pattern char: {}", c),
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { lights })
    }
    
    fn to_bitmask(&self) -> u64 {
        self.lights.iter().enumerate()
            .filter(|(_, &on)| on)
            .fold(0u64, |acc, (i, _)| acc | (1 << i))
    }
}

#[derive(Debug, Clone)]
struct IndexGroup { indices: Vec<usize> }

impl IndexGroup {
    fn from_str(s: &str) -> Result<Self> {
        let indices = s.split(',')
            .map(|n| n.trim().parse::<usize>().context("Invalid index"))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { indices })
    }
    
    fn to_bitmask(&self) -> u64 {
        self.indices.iter().fold(0u64, |acc, &i| acc | (1 << i))
    }
}

#[derive(Debug, Clone)]
struct Machine {
    pattern: Pattern,
    buttons: Vec<IndexGroup>,
    joltages: Vec<i64>,
}

impl Machine {
    fn from_line(line: &str) -> Result<Self> {
        let line = line.trim();
        let bracket_start = line.find('[').context("Missing '['")?;
        let bracket_end = line.find(']').context("Missing ']'")?;
        let pattern = Pattern::from_str(&line[bracket_start + 1..bracket_end])?;
        
        let brace_start = line.rfind('{').context("Missing '{'")?;
        let brace_end = line.rfind('}').context("Missing '}'")?;
        let joltages: Vec<i64> = line[brace_start + 1..brace_end]
            .split(',')
            .map(|n| n.trim().parse::<i64>().context("Invalid joltage"))
            .collect::<Result<Vec<_>>>()?;
        
        let buttons: Vec<IndexGroup> = line[bracket_end + 1..brace_start]
            .split(')')
            .filter_map(|part| {
                part.trim().find('(').and_then(|start| {
                    let inner = &part.trim()[start + 1..];
                    (!inner.is_empty()).then(|| IndexGroup::from_str(inner))
                })
            })
            .collect::<Result<Vec<_>>>()?;
        
        Ok(Self { pattern, buttons, joltages })
    }
}

fn parse_input(path: &str) -> Result<Vec<Machine>> {
    read_lines(path)?.iter()
        .filter(|line| !line.is_empty())
        .map(|line| Machine::from_line(line))
        .collect()
}

// ============================================================================
// Part 1: BFS over bitmask states (toggle/XOR)
// ============================================================================

fn solve_lights_bfs(machine: &Machine) -> Option<u64> {
    let target = machine.pattern.to_bitmask();
    let button_masks: Vec<u64> = machine.buttons.iter().map(|b| b.to_bitmask()).collect();
    
    let expand = |frontier: &HashSet<u64>, visited: &HashSet<u64>| -> HashSet<u64> {
        frontier.iter()
            .flat_map(|&state| button_masks.iter().map(move |&mask| state ^ mask))
            .filter(|state| !visited.contains(state))
            .collect()
    };
    
    let initial: HashSet<u64> = HashSet::from([0u64]);
    
    std::iter::successors(
        Some((initial.clone(), initial)),
        |(frontier, visited)| {
            let next = expand(frontier, visited);
            (!next.is_empty()).then(|| {
                let new_visited = visited.union(&next).copied().collect();
                (next, new_visited)
            })
        },
    )
    .enumerate()
    .find_map(|(depth, (frontier, _))| frontier.contains(&target).then_some(depth as u64))
}

// ============================================================================
// Part 2: Gaussian Elimination with Rational Arithmetic
// ============================================================================

#[derive(Clone, Debug)]
struct Rat(i64, i64);

impl Rat {
    fn new(n: i64, d: i64) -> Self {
        if d == 0 { return Self(n, d); }
        let g = gcd(n.abs(), d.abs());
        let sign = if d < 0 { -1 } else { 1 };
        Self(sign * n / g, sign * d / g)
    }
    fn zero() -> Self { Self(0, 1) }
    fn one() -> Self { Self(1, 1) }
    fn is_zero(&self) -> bool { self.0 == 0 }
    fn sub(&self, o: &Self) -> Self { Rat::new(self.0 * o.1 - o.0 * self.1, self.1 * o.1) }
    fn mul(&self, o: &Self) -> Self { Rat::new(self.0 * o.0, self.1 * o.1) }
    fn div(&self, o: &Self) -> Self { Rat::new(self.0 * o.1, self.1 * o.0) }
    fn to_i64(&self) -> Option<i64> {
        if self.1 == 1 { Some(self.0) }
        else if self.1 != 0 && self.0 % self.1 == 0 { Some(self.0 / self.1) }
        else { None }
    }
}

fn gcd(a: i64, b: i64) -> i64 { if b == 0 { a } else { gcd(b, a % b) } }

struct GaussResult {
    matrix: Vec<Vec<Rat>>,
    pivot_cols: Vec<usize>,
    free_cols: Vec<usize>,
    num_vars: usize,
}

fn gaussian_eliminate(mut matrix: Vec<Vec<Rat>>, num_vars: usize) -> GaussResult {
    let num_rows = matrix.len();
    let num_cols = num_vars + 1;
    let mut pivot_col_for_row = vec![None; num_rows];
    let mut current_row = 0;

    for col in 0..num_vars {
        if let Some(pr) = (current_row..num_rows).find(|&r| !matrix[r][col].is_zero()) {
            matrix.swap(current_row, pr);
            pivot_col_for_row[current_row] = Some(col);

            let pivot_val = matrix[current_row][col].clone();
            for c in 0..num_cols {
                matrix[current_row][c] = matrix[current_row][c].div(&pivot_val);
            }

            for r in 0..num_rows {
                if r != current_row && !matrix[r][col].is_zero() {
                    let factor = matrix[r][col].clone();
                    for c in 0..num_cols {
                        let sub_val = factor.mul(&matrix[current_row][c]);
                        matrix[r][c] = matrix[r][c].sub(&sub_val);
                    }
                }
            }
            current_row += 1;
        }
    }

    let pivot_cols: Vec<usize> = pivot_col_for_row.into_iter().flatten().collect();
    let free_cols: Vec<usize> = (0..num_vars).filter(|c| !pivot_cols.contains(c)).collect();

    GaussResult { matrix, pivot_cols, free_cols, num_vars }
}

fn back_substitute(g: &GaussResult, free_vals: &[i64]) -> Option<Vec<i64>> {
    let mut x = vec![0i64; g.num_vars];
    for (i, &fc) in g.free_cols.iter().enumerate() {
        x[fc] = free_vals[i];
    }
    for (row, &pc) in g.pivot_cols.iter().enumerate() {
        let mut val = g.matrix[row][g.num_vars].clone();
        for &fc in &g.free_cols {
            val = val.sub(&g.matrix[row][fc].mul(&Rat::new(x[fc], 1)));
        }
        x[pc] = val.to_i64()?;
        if x[pc] < 0 { return None; }
    }
    Some(x)
}

fn solve_joltage_gauss(machine: &Machine) -> Option<u64> {
    let buttons = &machine.buttons;
    let targets = &machine.joltages;
    let num_counters = targets.len();
    let num_buttons = buttons.len();

    // Build augmented matrix
    let matrix: Vec<Vec<Rat>> = (0..num_counters)
        .map(|row| {
            let mut row_vec: Vec<Rat> = buttons.iter()
                .map(|b| if b.indices.contains(&row) { Rat::one() } else { Rat::zero() })
                .collect();
            row_vec.push(Rat::new(targets[row], 1));
            row_vec
        })
        .collect();

    let gauss = gaussian_eliminate(matrix, num_buttons);
    let max_free = targets.iter().copied().max().unwrap_or(0);

    // Search over free variables
    fn search(idx: usize, vals: &mut Vec<i64>, g: &GaussResult, max_val: i64, best: &mut Option<u64>) {
        let sum: i64 = vals.iter().sum();
        if best.map_or(false, |b| sum as u64 >= b) { return; }

        if idx == g.free_cols.len() {
            if let Some(x) = back_substitute(g, vals) {
                let total = x.iter().sum::<i64>() as u64;
                if best.map_or(true, |b| total < b) {
                    *best = Some(total);
                }
            }
            return;
        }

        for v in 0..=max_val {
            vals.push(v);
            search(idx + 1, vals, g, max_val, best);
            vals.pop();
        }
    }

    let mut best = None;
    search(0, &mut Vec::new(), &gauss, max_free, &mut best);
    best
}

// ============================================================================
// Main
// ============================================================================

fn part1(path: &str) -> Result<i64> {
    Ok(parse_input(path)?.iter().filter_map(solve_lights_bfs).sum::<u64>() as i64)
}

fn part2(path: &str) -> Result<i64> {
    Ok(parse_input(path)?.iter().filter_map(solve_joltage_gauss).sum::<u64>() as i64)
}

fn main() -> Result<()> {
    println!("=== Advent of Code 2025 - Day 10 ===\n");
    // Run with test input first
    // Probably need to extract BFS and Gaussian elimination for next year for similar problems.
    // LLM's again today
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
