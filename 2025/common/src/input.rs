//! Input parsing utilities for Advent of Code

use std::error::Error;
use crate::grid::Grid;
use anyhow::{anyhow, Context, Result};
use std::fs;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use serde::de::DeserializeOwned;

/// Read entire file as string
pub fn read_string<P: AsRef<Path>>(path: P) -> Result<String> {
    fs::read_to_string(path.as_ref())
        .with_context(|| format!("Failed to read file: {:?}", path.as_ref()))
}

/// Read file as lines
pub fn read_lines<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    let content = read_string(path)?;
    Ok(content.lines().map(String::from).collect())
}

/// Read file as a grid of characters
pub fn read_grid<P: AsRef<Path>>(path: P) -> Result<Grid<char>> {
    let content = read_string(path)?;
    Ok(Grid::from_str(&content))
}

/// Read file as a grid of digits (0-9)
pub fn read_digit_grid<P: AsRef<Path>>(path: P) -> Result<Grid<u8>> {
    let content = read_string(path)?;
    let cells: Vec<Vec<u8>> = content
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as u8))
                .collect()
        })
        .collect();
    Ok(Grid::new(cells))
}

/// Read file as a grid with custom parsing per character
pub fn read_grid_map<P, T, F>(path: P, map_fn: F) -> Result<Grid<T>>
where
    P: AsRef<Path>,
    F: Fn(char) -> T,
{
    let content = read_string(path)?;
    let cells: Vec<Vec<T>> = content
        .lines()
        .map(|line| line.chars().map(&map_fn).collect())
        .collect();
    Ok(Grid::new(cells))
}

/// Read CSV file with custom delimiter
pub fn read_csv<T>(file: &str, delimiter: u8) -> std::result::Result<Vec<T>, Box<dyn Error>>
where T: DeserializeOwned {
    let file = File::open(file)?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(false)
        .from_reader(file);

    let result = rdr
        .deserialize()
        .flat_map(|x| x)
        .collect();

    Ok(result)
}

/// Read a single line and split by delimiter, parsing each part
pub fn read_delimited<T, P>(path: P, delimiter: char) -> Result<Vec<T>>
where
    T: FromStr,
    T::Err: std::fmt::Debug,
    P: AsRef<Path>,
{
    let content = read_string(path)?;
    Ok(content
        .trim()
        .split(delimiter)
        .filter_map(|s| s.parse().ok())
        .collect())
}

/// Read lines and parse each into a type
pub fn read_parsed<T, P>(path: P) -> Result<Vec<T>>
where
    T: FromStr,
    T::Err: std::fmt::Debug,
    P: AsRef<Path>,
{
    let lines = read_lines(path)?;
    Ok(lines.iter().filter_map(|s| s.parse().ok()).collect())
}

/// Read file and split by blank lines into groups
pub fn read_groups<P: AsRef<Path>>(path: P) -> Result<Vec<Vec<String>>> {
    let content = read_string(path)?;
    Ok(content
        .split("\n\n")
        .map(|group| group.lines().map(String::from).collect())
        .collect())
}

/// Read numbers from a file (one per line)
pub fn read_numbers<T, P>(path: P) -> Result<Vec<T>>
where
    T: FromStr,
    T::Err: std::fmt::Debug,
    P: AsRef<Path>,
{
    read_parsed(path)
}

/// Split input into two sections separated by blank line
pub fn split_sections(input: &str) -> (String, String) {
    let parts: Vec<&str> = input.splitn(2, "\n\n").collect();
    match parts.as_slice() {
        [first, second] => (first.to_string(), second.to_string()),
        [first] => (first.to_string(), String::new()),
        _ => (String::new(), String::new()),
    }
}

/// Extract all integers from a string
pub fn extract_integers<T: FromStr>(s: &str) -> Vec<T> {
    s.split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter(|s| !s.is_empty() && *s != "-")
        .filter_map(|s| s.parse().ok())
        .collect()
}

/// Extract all unsigned integers from a string
pub fn extract_unsigned<T: FromStr>(s: &str) -> Vec<T> {
    s.split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_integers() {
        let nums: Vec<i32> = extract_integers("pos=<-1,2,3>, vel=<4,-5,6>");
        assert_eq!(nums, vec![-1, 2, 3, 4, -5, 6]);
    }

    #[test]
    fn test_extract_unsigned() {
        let nums: Vec<u32> = extract_unsigned("abc123def456");
        assert_eq!(nums, vec![123, 456]);
    }

    #[test]
    fn test_split_sections() {
        let (a, b) = split_sections("first\n\nsecond\nthird");
        assert_eq!(a, "first");
        assert_eq!(b, "second\nthird");
    }
}
