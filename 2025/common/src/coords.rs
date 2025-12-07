//! 2D coordinate utilities for grid-based puzzles

use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

/// A 2D coordinate with signed integer components
#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Coord {
    pub x: i64,
    pub y: i64,
}

impl Coord {
    /// Create a new coordinate
    #[inline]
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    /// Origin point (0, 0)
    pub const ORIGIN: Coord = Coord { x: 0, y: 0 };

    /// Cardinal directions
    pub const NORTH: Coord = Coord { x: 0, y: -1 };
    pub const EAST: Coord = Coord { x: 1, y: 0 };
    pub const SOUTH: Coord = Coord { x: 0, y: 1 };
    pub const WEST: Coord = Coord { x: -1, y: 0 };

    /// Diagonal directions
    pub const NORTH_EAST: Coord = Coord { x: 1, y: -1 };
    pub const NORTH_WEST: Coord = Coord { x: -1, y: -1 };
    pub const SOUTH_EAST: Coord = Coord { x: 1, y: 1 };
    pub const SOUTH_WEST: Coord = Coord { x: -1, y: 1 };

    /// Cardinal directions array (N, E, S, W)
    pub const CARDINALS: [Coord; 4] = [Self::NORTH, Self::EAST, Self::SOUTH, Self::WEST];

    /// Diagonal directions array
    pub const DIAGONALS: [Coord; 4] = [
        Self::NORTH_EAST,
        Self::NORTH_WEST,
        Self::SOUTH_EAST,
        Self::SOUTH_WEST,
    ];

    /// All 8 directions (cardinals + diagonals)
    pub const ALL_DIRECTIONS: [Coord; 8] = [
        Self::NORTH,
        Self::EAST,
        Self::SOUTH,
        Self::WEST,
        Self::NORTH_EAST,
        Self::NORTH_WEST,
        Self::SOUTH_EAST,
        Self::SOUTH_WEST,
    ];

    // Unique id for Coord
    pub fn idx(&self, width: usize) -> usize {
        self.y as usize * width + self.x as usize
    }
    /// Get cardinal neighbors (4-connected)
    pub fn neighbors(&self) -> impl Iterator<Item = Coord> + '_ {
        Self::CARDINALS.iter().map(|d| *self + *d)
    }

    /// Get all neighbors including diagonals (8-connected)
    pub fn neighbors_all(&self) -> impl Iterator<Item = Coord> + '_ {
        Self::ALL_DIRECTIONS.iter().map(|d| *self + *d)
    }

    /// Manhattan distance to another coordinate
    #[inline]
    pub fn manhattan_distance(&self, other: Coord) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    /// Manhattan distance to origin
    #[inline]
    pub fn manhattan_length(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }

    /// Wrap coordinate within bounds (modular arithmetic)
    #[inline]
    pub fn wrap(&self, bounds: Coord) -> Coord {
        Coord {
            x: self.x.rem_euclid(bounds.x),
            y: self.y.rem_euclid(bounds.y),
        }
    }

    /// Check if coordinate is within bounds (0..max_x, 0..max_y)
    #[inline]
    pub fn in_bounds(&self, width: i64, height: i64) -> bool {
        self.x >= 0 && self.x < width && self.y >= 0 && self.y < height
    }

    /// Check if coordinate is within grid bounds
    #[inline]
    pub fn in_grid_bounds<T>(&self, grid: &crate::grid::Grid<T>) -> bool {
        self.in_bounds(grid.width() as i64, grid.height() as i64)
    }

    /// Rotate 90 degrees clockwise
    #[inline]
    pub fn rotate_cw(&self) -> Coord {
        Coord {
            x: -self.y,
            y: self.x,
        }
    }

    /// Rotate 90 degrees counter-clockwise
    #[inline]
    pub fn rotate_ccw(&self) -> Coord {
        Coord {
            x: self.y,
            y: -self.x,
        }
    }

    /// Rotate 180 degrees
    #[inline]
    pub fn rotate_180(&self) -> Coord {
        Coord {
            x: -self.x,
            y: -self.y,
        }
    }

    /// Scale by a constant
    #[inline]
    pub fn scale(&self, factor: i64) -> Coord {
        Coord {
            x: self.x * factor,
            y: self.y * factor,
        }
    }

    /// Convert to (usize, usize) tuple for indexing (returns None if negative)
    pub fn to_usize(&self) -> Option<(usize, usize)> {
        if self.x >= 0 && self.y >= 0 {
            Some((self.x as usize, self.y as usize))
        } else {
            None
        }
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(i64, i64)> for Coord {
    fn from((x, y): (i64, i64)) -> Self {
        Coord { x, y }
    }
}

impl From<(i32, i32)> for Coord {
    fn from((x, y): (i32, i32)) -> Self {
        Coord {
            x: x as i64,
            y: y as i64,
        }
    }
}

impl From<(usize, usize)> for Coord {
    fn from((x, y): (usize, usize)) -> Self {
        Coord {
            x: x as i64,
            y: y as i64,
        }
    }
}

impl Add for Coord {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Coord {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Coord {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Coord {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Neg for Coord {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Coord {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Mul<i64> for Coord {
    type Output = Self;
    #[inline]
    fn mul(self, scalar: i64) -> Self {
        Coord {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

/// Named directions for readability
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Direction {
    /// All cardinal directions
    pub const ALL: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    /// Convert to coordinate delta
    pub fn to_coord(self) -> Coord {
        match self {
            Direction::North => Coord::NORTH,
            Direction::East => Coord::EAST,
            Direction::South => Coord::SOUTH,
            Direction::West => Coord::WEST,
        }
    }

    /// Turn right (clockwise)
    pub fn turn_right(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    /// Turn left (counter-clockwise)
    pub fn turn_left(self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    /// Turn around (180 degrees)
    pub fn turn_around(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    /// Parse from common characters (^, >, v, <, N, E, S, W, U, D, L, R)
    pub fn from_char(c: char) -> Option<Direction> {
        match c {
            '^' | 'N' | 'U' => Some(Direction::North),
            '>' | 'E' | 'R' => Some(Direction::East),
            'v' | 'V' | 'S' | 'D' => Some(Direction::South),
            '<' | 'W' | 'L' => Some(Direction::West),
            _ => None,
        }
    }

    /// Convert to character representation
    pub fn to_char(self) -> char {
        match self {
            Direction::North => '^',
            Direction::East => '>',
            Direction::South => 'v',
            Direction::West => '<',
        }
    }
}

impl From<Direction> for Coord {
    fn from(dir: Direction) -> Coord {
        dir.to_coord()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord_arithmetic() {
        let a = Coord::new(3, 4);
        let b = Coord::new(1, 2);

        assert_eq!(a + b, Coord::new(4, 6));
        assert_eq!(a - b, Coord::new(2, 2));
        assert_eq!(a * 2, Coord::new(6, 8));
        assert_eq!(-a, Coord::new(-3, -4));
    }

    #[test]
    fn test_manhattan_distance() {
        let a = Coord::new(0, 0);
        let b = Coord::new(3, 4);
        assert_eq!(a.manhattan_distance(b), 7);
    }

    #[test]
    fn test_rotation() {
        let north = Coord::NORTH;
        assert_eq!(north.rotate_cw(), Coord::EAST);
        assert_eq!(north.rotate_ccw(), Coord::WEST);
        assert_eq!(north.rotate_180(), Coord::SOUTH);
    }

    #[test]
    fn test_direction_turns() {
        assert_eq!(Direction::North.turn_right(), Direction::East);
        assert_eq!(Direction::North.turn_left(), Direction::West);
        assert_eq!(Direction::North.turn_around(), Direction::South);
    }
}

