use std::cmp::{max, min};
use std::ops::RangeInclusive;
use std::str::FromStr;
use anyhow::anyhow;

/// A range with start and end values (inclusive)
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Range {
    pub start: i64,
    pub end: i64,
}

impl Range {
    pub fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }

    /// Check if a value is within this range (inclusive)
    pub fn contains(&self, value: i64) -> bool {
        value >= self.start && value <= self.end
    }

    /// Check if this range overlaps with another
    pub fn overlaps(&self, other: &Range) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    pub fn combine_ranges(&self, other: &Range) -> Range {
        assert!(self.overlaps(other), "Ranges must overlap");
        let start = min(self.start, other.start);
        let end = max(self.end, other.end);
        Range::new(start, end)
    }

    /// Get the length of the range
    pub fn len(&self) -> i64 {
        self.end - self.start + 1
    }

    pub fn range_iter(&self) -> RangeInclusive<i64> { self.start..=self.end }
}

impl FromStr for Range {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let parts: Vec<&str> = s.trim().split('-').collect();
        if parts.len() != 2 {
            return Err(anyhow!("Invalid range format: '{}', expected 'start-end'", s));
        }

        let start = parts[0]
            .parse::<i64>()
            .map_err(|e| anyhow!("Failed to parse range start '{}': {}", parts[0], e))?;
        let end = parts[1]
            .parse::<i64>()
            .map_err(|e| anyhow!("Failed to parse range end '{}': {}", parts[1], e))?;

        Ok(Range { start, end })
    }
}