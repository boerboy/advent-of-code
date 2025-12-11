//! Common utilities for Advent of Code 2025
//!
//! This crate provides shared functionality for solving AoC puzzles:
//! - `coords`: 2D coordinate handling with directional constants
//! - `grid`: 2D grid operations for map-based puzzles
//! - `input`: Input parsing utilities
//! - `math`: Mathematical helpers
//! - `search`: BFS, DFS, and flood fill algorithms

pub mod coords;
pub mod grid;
pub mod input;
pub mod math;
pub mod range;
pub mod search;

// Re-export commonly used types at crate root
pub use coords::Coord;
pub use grid::Grid;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::coords::{Coord, Direction};
    pub use crate::grid::Grid;
    pub use crate::input::{read_lines, read_grid, read_csv, read_string};
    pub use crate::math::gcd;
    pub use crate::search::{bfs, bfs_levels, bfs_path, dfs_count_paths, dfs_all_paths, dfs_find_path, flood_fill, BfsResult};
    pub use anyhow::{anyhow, bail, Context, Result};
    pub use itertools::Itertools;
    pub use std::collections::{HashMap, HashSet, VecDeque};
    pub use crate::range::Range;
}

