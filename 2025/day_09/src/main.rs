//! Advent of Code 2025 - Day 09
//!
//! Puzzle: https://adventofcode.com/2025/day/9

use anyhow::Result;
use common::input::read_csv;
use common::prelude::Itertools;
use common::Coord;

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

/// Calculate rectangle areas between all pairs of points
fn areas(points: &Vec<Coord>) -> Vec<(Coord, Coord, i64)> {
    points
        .iter()
        .enumerate()
        .flat_map(|(idx, &position1)| {
            points[idx + 1..]
                .iter()
                .map(move |&position2| {
                    (position1, position2, position1.rectangle_area(&position2))
                })
        })
        .collect()
}

/// Sort by rectangle area (descending)
fn sorted_by_area(points: &Vec<Coord>) -> Vec<(Coord, Coord, i64)> {
    areas(points)
        .into_iter()
        .sorted_by(|a, b| b.2.cmp(&a.2))
        .collect()
}

/// Vertical edge: (x, y_min, y_max)
type VerticalEdge = (i64, i64, i64);
/// Horizontal edge: (y, x_min, x_max)
type HorizontalEdge = (i64, i64, i64);

/// Extract edges from the polygon boundary
fn build_edges(red_tiles: &[Coord]) -> (Vec<VerticalEdge>, Vec<HorizontalEdge>) {
    let mut v_edges = Vec::new();
    let mut h_edges = Vec::new();
    for i in 0..red_tiles.len() {
        let next = (i + 1) % red_tiles.len();
        let a = &red_tiles[i];
        let b = &red_tiles[next];
        if a.x == b.x {
            // Vertical segment
            let y_min = a.y.min(b.y);
            let y_max = a.y.max(b.y);
            v_edges.push((a.x, y_min, y_max));
        } else {
            // Horizontal segment
            let x_min = a.x.min(b.x);
            let x_max = a.x.max(b.x);
            h_edges.push((a.y, x_min, x_max));
        }
    }
    (v_edges, h_edges)
}

/// Compressed representation of polygon using scanline bands + boundary edges
struct Polygon {
    /// Interior bands: (y_start, y_end, inside_x_ranges)
    bands: Vec<(i64, i64, Vec<(i64, i64)>)>,
    /// Horizontal boundary edges: (y, x_min, x_max)
    h_edges: Vec<HorizontalEdge>,
}

/// Merge overlapping/adjacent ranges into disjoint sorted ranges
fn merge_ranges(mut ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    if ranges.is_empty() {
        return ranges;
    }
    ranges.sort_by_key(|r| r.0);
    let mut merged = vec![ranges[0]];
    for (start, end) in ranges.into_iter().skip(1) {
        let last = merged.last_mut().unwrap();
        if start <= last.1 + 1 {
            // Overlapping or adjacent - extend
            last.1 = last.1.max(end);
        } else {
            merged.push((start, end));
        }
    }
    merged
}

impl Polygon {
    fn new(v_edges: &[VerticalEdge], h_edges: Vec<HorizontalEdge>) -> Self {
        // Collect all unique y-coordinates where vertical edges start or end
        let mut y_events: Vec<i64> = v_edges
            .iter()
            .flat_map(|&(_, y_min, y_max)| vec![y_min, y_max + 1])
            .collect();
        y_events.sort();
        y_events.dedup();

        let mut bands = Vec::new();
        for window in y_events.windows(2) {
            let y_start = window[0];
            let y_end = window[1] - 1;

            // Find all vertical edges active at this y (half-open interval)
            let mut x_crossings: Vec<i64> = v_edges
                .iter()
                .filter(|&&(_, y_min, y_max)| y_min <= y_start && y_start < y_max)
                .map(|&(x, _, _)| x)
                .collect();
            x_crossings.sort();

            // Even-odd rule: pair up crossings to get inside ranges
            let ranges: Vec<(i64, i64)> = x_crossings
                .chunks(2)
                .filter_map(|chunk| {
                    if chunk.len() == 2 {
                        Some((chunk[0], chunk[1]))
                    } else {
                        None
                    }
                })
                .collect();

            bands.push((y_start, y_end, ranges));
        }
        Self { bands, h_edges }
    }

    /// Get all coverage ranges at a given y (interior bands + horizontal edges merged)
    fn coverage_at_y(&self, y: i64) -> Vec<(i64, i64)> {
        let mut ranges = Vec::new();

        // Add interior band ranges
        for &(band_start, band_end, ref band_ranges) in &self.bands {
            if band_start <= y && y <= band_end {
                ranges.extend(band_ranges.iter().copied());
                break;
            }
        }

        // Add horizontal edge ranges at this y
        for &(edge_y, x_min, x_max) in &self.h_edges {
            if edge_y == y {
                ranges.push((x_min, x_max));
            }
        }

        merge_ranges(ranges)
    }

    /// Check if a rectangle is entirely inside the polygon (boundary + interior)
    fn contains_rectangle(&self, x_min: i64, x_max: i64, y_min: i64, y_max: i64) -> bool {
        let mut y = y_min;
        while y <= y_max {
            let coverage = self.coverage_at_y(y);

            // Check if required x-range is within any merged coverage range
            let covered = coverage
                .iter()
                .any(|&(r_min, r_max)| r_min <= x_min && x_max <= r_max);

            if !covered {
                return false;
            }

            // Optimization: find how far we can skip with same coverage
            // Find the band that covers this y to determine skip distance
            let mut skip_to = y;
            for &(band_start, band_end, ref ranges) in &self.bands {
                if band_start <= y && y <= band_end {
                    // Check if band's interior alone covers our x-range
                    if ranges.iter().any(|&(r_min, r_max)| r_min <= x_min && x_max <= r_max) {
                        skip_to = band_end.min(y_max);
                    }
                    break;
                }
            }
            y = skip_to + 1;
        }
        true
    }
}

fn part1(path: &str) -> Result<i64> {
    let input: &Vec<Coord> =
        &read_csv(path, b',').map_err(|err| anyhow::anyhow!(err.to_string()))?;
    let sorted = sorted_by_area(input);
    let result = sorted.first().map(|(_, _, area)| *area).unwrap_or(0);
    Ok(result)
}

fn part2(path: &str) -> Result<i64> {
    let red_tiles: Vec<Coord> =
        read_csv(path, b',').map_err(|err| anyhow::anyhow!(err.to_string()))?;

    // Build edges and polygon representation
    let (v_edges, h_edges) = build_edges(&red_tiles);
    let polygon = Polygon::new(&v_edges, h_edges);

    // Find largest valid rectangle (sorted by area descending, first match wins)
    let sorted = sorted_by_area(&red_tiles);
    let result = sorted
        .iter()
        .filter(|(c1, c2, _)| {
            let x_min = c1.x.min(c2.x);
            let x_max = c1.x.max(c2.x);
            let y_min = c1.y.min(c2.y);
            let y_max = c1.y.max(c2.y);
            polygon.contains_rectangle(x_min, x_max, y_min, y_max)
        })
        .map(|(_, _, area)| *area)
        .next()
        .unwrap_or(0);

    Ok(result)
}

fn main() -> Result<()> {
    println!("=== Advent of Code 2025 - Day 09 ===\n");
    // Run with test input first
    // Did part 1 completely solo, part 2 ran out of time for coding in the day and used some llm to wack it down
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
