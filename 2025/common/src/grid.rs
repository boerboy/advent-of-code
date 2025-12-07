//! 2D grid utilities for map-based puzzles

use crate::coords::Coord;
use std::fmt::{self, Debug, Display};
use std::ops::{Index, IndexMut};

/// A 2D grid backed by a vector of vectors
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Grid<T> {
    cells: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    /// Create a new grid from a 2D vector
    pub fn new(cells: Vec<Vec<T>>) -> Self {
        let height = cells.len();
        let width = cells.first().map_or(0, |row| row.len());
        Self {
            cells,
            width,
            height,
        }
    }

    /// Grid width
    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    /// Grid height
    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    /// Check if coordinate is within bounds
    #[inline]
    pub fn in_bounds(&self, coord: Coord) -> bool {
        coord.x >= 0
            && coord.y >= 0
            && (coord.x as usize) < self.width
            && (coord.y as usize) < self.height
    }

    /// Get value at coordinate (returns None if out of bounds)
    pub fn get(&self, coord: Coord) -> Option<&T> {
        if self.in_bounds(coord) {
            Some(&self.cells[coord.y as usize][coord.x as usize])
        } else {
            None
        }
    }

    /// Get mutable value at coordinate (returns None if out of bounds)
    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut T> {
        if self.in_bounds(coord) {
            Some(&mut self.cells[coord.y as usize][coord.x as usize])
        } else {
            None
        }
    }

    /// Set value at coordinate (returns false if out of bounds)
    pub fn set(&mut self, coord: Coord, value: T) -> bool {
        if self.in_bounds(coord) {
            self.cells[coord.y as usize][coord.x as usize] = value;
            true
        } else {
            false
        }
    }

    /// Iterate over all coordinates in row-major order
    pub fn coords(&self) -> impl Iterator<Item = Coord> + '_ {
        (0..self.height)
            .flat_map(move |y| (0..self.width).map(move |x| Coord::new(x as i64, y as i64)))
    }

    /// Iterate over all (coordinate, value) pairs
    pub fn iter(&self) -> impl Iterator<Item = (Coord, &T)> + '_ {
        self.coords()
            .map(|c| (c, &self.cells[c.y as usize][c.x as usize]))
    }

    /// Iterate over all values mutably
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Coord, &mut T)> + '_ {
        self.cells.iter_mut().enumerate().flat_map(|(y, row)| {
            row.iter_mut()
                .enumerate()
                .map(move |(x, val)| (Coord::new(x as i64, y as i64), val))
        })
    }

    /// Get row as slice
    pub fn row(&self, y: usize) -> Option<&[T]> {
        self.cells.get(y).map(|r| r.as_slice())
    }

    /// Iterate over rows
    pub fn rows(&self) -> impl Iterator<Item = &[T]> + '_ {
        self.cells.iter().map(|r| r.as_slice())
    }

    /// Iterate over columns (returns cloned values since columns aren't contiguous)
    pub fn columns(&self) -> impl Iterator<Item = Vec<&T>> + '_ {
        (0..self.width).map(move |x| self.cells.iter().map(|row| &row[x]).collect())
    }
}

impl<T: Clone> Grid<T> {
    /// Create a grid filled with a default value
    pub fn filled(width: usize, height: usize, value: T) -> Self {
        Self {
            cells: vec![vec![value; width]; height],
            width,
            height,
        }
    }

    /// Get column as Vec (cloned)
    pub fn column(&self, x: usize) -> Option<Vec<T>> {
        if x < self.width {
            Some(self.cells.iter().map(|row| row[x].clone()).collect())
        } else {
            None
        }
    }

    /// Transpose the grid (swap rows and columns)
    pub fn transpose(&self) -> Self {
        let cells: Vec<Vec<T>> = (0..self.width)
            .map(|x| self.cells.iter().map(|row| row[x].clone()).collect())
            .collect();
        Self::new(cells)
    }
}

impl<T: PartialEq> Grid<T> {
    /// Find first coordinate containing the given value
    pub fn find(&self, value: &T) -> Option<Coord> {
        self.iter().find(|(_, v)| *v == value).map(|(c, _)| c)
    }

    /// Find all coordinates containing the given value
    pub fn find_all(&self, value: &T) -> Vec<Coord> {
        self.iter()
            .filter(|(_, v)| *v == value)
            .map(|(c, _)| c)
            .collect()
    }
}

impl<T> Grid<T> {
    /// Find coordinates matching a predicate
    pub fn find_by<F>(&self, predicate: F) -> Vec<Coord>
    where
        F: Fn(&T) -> bool,
    {
        self.iter()
            .filter(|(_, v)| predicate(v))
            .map(|(c, _)| c)
            .collect()
    }

    /// Map each cell to a new value
    pub fn map<U, F>(&self, f: F) -> Grid<U>
    where
        F: Fn(&T) -> U,
    {
        Grid::new(
            self.cells
                .iter()
                .map(|row| row.iter().map(&f).collect())
                .collect(),
        )
    }

    /// Get cardinal neighbors that are in bounds
    pub fn neighbors(&self, coord: Coord) -> Vec<(Coord, &T)> {
        Coord::CARDINALS
            .iter()
            .map(|d| coord + *d)
            .filter(|c| self.in_bounds(*c))
            .map(|c| (c, &self.cells[c.y as usize][c.x as usize]))
            .collect()
    }

    /// Get all 8 neighbors that are in bounds
    pub fn neighbors_all(&self, coord: Coord) -> Vec<(Coord, &T)> {
        Coord::ALL_DIRECTIONS
            .iter()
            .map(|d| coord + *d)
            .filter(|c| self.in_bounds(*c))
            .map(|c| (c, &self.cells[c.y as usize][c.x as usize]))
            .collect()
    }

    pub fn visited_tracker(&self) -> VisitedTracker {
        VisitedTracker {
            data: vec![0; self.width() * self.height()],
            width: self.width(),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct VisitedTracker {
    pub data: Vec<i64>,
    width: usize,
}

impl VisitedTracker {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![0; width * height],
            width,
        }
    }

    pub fn visit(&mut self, c: Coord) -> bool {
        let idx = c.idx(self.width);
        let prev = self.data[idx];
        self.data[idx] += 1;
        prev == 0
    }

    pub fn is_visited(&self, c: Coord) -> bool {
        self.data[c.y as usize * self.width + c.x as usize] >= 1
    }

    pub fn add(&mut self, c: Coord, visits: i64) {
        let idx = c.y as usize * self.width + c.x as usize;
        self.data[idx] += visits;
    }

    pub fn get(&self, c: Coord) -> i64 {
        self.data[c.y as usize * self.width + c.x as usize]
    }

    pub fn set(&mut self, c: Coord, visits: i64) {
        self.data[c.y as usize * self.width + c.x as usize] = visits;
    }
}

impl<T: Display> Grid<T> {
    /// Pretty print the grid
    pub fn print(&self) {
        for row in &self.cells {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }

    /// Convert grid to string representation
    pub fn to_string_grid(&self) -> String {
        self.cells
            .iter()
            .map(|row| row.iter().map(|c| c.to_string()).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Grid<char> {
    /// Parse a grid from a string (each line becomes a row)
    pub fn from_str(s: &str) -> Self {
        let cells: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
        Self::new(cells)
    }

    /// Create a grid from a Vec of Strings (each String becomes a row)
    pub fn from_lines(lines: Vec<String>) -> Self {
        let cells: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
        Self::new(cells)
    }
}

impl<T> Index<Coord> for Grid<T> {
    type Output = T;

    fn index(&self, coord: Coord) -> &Self::Output {
        &self.cells[coord.y as usize][coord.x as usize]
    }
}

impl<T> IndexMut<Coord> for Grid<T> {
    fn index_mut(&mut self, coord: Coord) -> &mut Self::Output {
        &mut self.cells[coord.y as usize][coord.x as usize]
    }
}

impl<T: Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.cells.iter().enumerate() {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            if i < self.height - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
