//! Search algorithms for Advent of Code puzzles
//!
//! Common search patterns extracted for reuse:
//! - BFS for shortest paths and minimum steps
//! - DFS for path counting and exploration

use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

// ============================================================================
// BFS - Breadth-First Search
// ============================================================================

/// Result of a BFS search
#[derive(Debug, Clone)]
pub struct BfsResult<S> {
    /// The target state that was found (if any)
    pub state: Option<S>,
    /// Number of steps to reach the target
    pub steps: u64,
}

/// BFS to find minimum steps to reach a target state.
///
/// # Arguments
/// * `start` - Initial state
/// * `is_target` - Predicate to check if we've reached the goal
/// * `neighbors` - Function that returns all neighboring states from a given state
///
/// # Example
/// ```ignore
/// let result = bfs(0u64, |s| *s == target, |s| {
///     button_masks.iter().map(move |m| s ^ m).collect()
/// });
/// ```
pub fn bfs<S, F, N, I>(start: S, is_target: F, neighbors: N) -> BfsResult<S>
where
    S: Clone + Eq + Hash,
    F: Fn(&S) -> bool,
    N: Fn(&S) -> I,
    I: IntoIterator<Item = S>,
{
    if is_target(&start) {
        return BfsResult { state: Some(start), steps: 0 };
    }

    let mut visited: HashSet<S> = HashSet::new();
    visited.insert(start.clone());

    let mut queue: VecDeque<(S, u64)> = VecDeque::new();
    queue.push_back((start, 0));

    while let Some((current, steps)) = queue.pop_front() {
        for next in neighbors(&current) {
            if visited.contains(&next) {
                continue;
            }

            if is_target(&next) {
                return BfsResult { state: Some(next), steps: steps + 1 };
            }

            visited.insert(next.clone());
            queue.push_back((next, steps + 1));
        }
    }

    BfsResult { state: None, steps: 0 }
}

/// BFS that returns all states at each distance level.
/// Useful when you need to process states level-by-level.
///
/// # Arguments
/// * `start` - Initial state
/// * `neighbors` - Function that returns all neighboring states
///
/// # Returns
/// Iterator yielding (depth, states_at_depth) pairs
pub fn bfs_levels<S, N, I>(start: S, neighbors: N) -> impl Iterator<Item = (u64, HashSet<S>)>
where
    S: Clone + Eq + Hash,
    N: Fn(&S) -> I + Clone,
    I: IntoIterator<Item = S>,
{
    let initial: HashSet<S> = [start].into_iter().collect();
    let mut visited = initial.clone();
    let mut depth = 0u64;
    let mut frontier = initial;

    std::iter::from_fn(move || {
        if frontier.is_empty() {
            return None;
        }

        let current_frontier = frontier.clone();
        let current_depth = depth;

        // Expand to next level
        let next: HashSet<S> = frontier
            .iter()
            .flat_map(|s| neighbors(s))
            .filter(|s| !visited.contains(s))
            .collect();

        for s in &next {
            visited.insert(s.clone());
        }

        frontier = next;
        depth += 1;

        Some((current_depth, current_frontier))
    })
}

/// Find shortest path and reconstruct it.
///
/// # Arguments
/// * `start` - Initial state
/// * `is_target` - Predicate to check if we've reached the goal
/// * `neighbors` - Function that returns all neighboring states
///
/// # Returns
/// Option containing the path from start to target (inclusive)
pub fn bfs_path<S, F, N, I>(start: S, is_target: F, neighbors: N) -> Option<Vec<S>>
where
    S: Clone + Eq + Hash,
    F: Fn(&S) -> bool,
    N: Fn(&S) -> I,
    I: IntoIterator<Item = S>,
{
    if is_target(&start) {
        return Some(vec![start]);
    }

    let mut visited: HashSet<S> = HashSet::new();
    let mut parent: HashMap<S, S> = HashMap::new();
    visited.insert(start.clone());

    let mut queue: VecDeque<S> = VecDeque::new();
    queue.push_back(start.clone());

    while let Some(current) = queue.pop_front() {
        for next in neighbors(&current) {
            if visited.contains(&next) {
                continue;
            }

            parent.insert(next.clone(), current.clone());

            if is_target(&next) {
                // Reconstruct path
                let mut path = vec![next.clone()];
                let mut curr = &next;
                while let Some(p) = parent.get(curr) {
                    path.push(p.clone());
                    curr = p;
                }
                path.reverse();
                return Some(path);
            }

            visited.insert(next.clone());
            queue.push_back(next);
        }
    }

    None
}

// ============================================================================
// DFS - Depth-First Search
// ============================================================================

/// Count all paths from start to target in a DAG (Directed Acyclic Graph).
/// Uses memoization for efficiency.
///
/// # Arguments
/// * `start` - Starting node
/// * `target` - Target node to reach
/// * `neighbors` - Function that returns all neighboring nodes
///
/// # Example
/// ```ignore
/// let count = dfs_count_paths("you", "out", |node| {
///     graph.get(node).cloned().unwrap_or_default()
/// });
/// ```
pub fn dfs_count_paths<S, N, I>(start: S, target: S, neighbors: N) -> u64
where
    S: Clone + Eq + Hash,
    N: Fn(&S) -> I,
    I: IntoIterator<Item = S>,
{
    fn count_inner<S, N, I>(
        current: &S,
        target: &S,
        neighbors: &N,
        memo: &mut HashMap<S, u64>,
    ) -> u64
    where
        S: Clone + Eq + Hash,
        N: Fn(&S) -> I,
        I: IntoIterator<Item = S>,
    {
        if current == target {
            return 1;
        }

        if let Some(&count) = memo.get(current) {
            return count;
        }

        let count: u64 = neighbors(current)
            .into_iter()
            .map(|next| count_inner(&next, target, neighbors, memo))
            .sum();

        memo.insert(current.clone(), count);
        count
    }

    let mut memo = HashMap::new();
    count_inner(&start, &target, &neighbors, &mut memo)
}

/// DFS to find all paths from start to target.
/// Warning: Can be expensive if there are many paths!
///
/// # Arguments
/// * `start` - Starting node
/// * `target` - Target node to reach
/// * `neighbors` - Function that returns all neighboring nodes
pub fn dfs_all_paths<S, N, I>(start: S, target: S, neighbors: N) -> Vec<Vec<S>>
where
    S: Clone + Eq + Hash,
    N: Fn(&S) -> I,
    I: IntoIterator<Item = S>,
{
    fn find_paths<S, N, I>(
        current: S,
        target: &S,
        neighbors: &N,
        path: &mut Vec<S>,
        all_paths: &mut Vec<Vec<S>>,
    )
    where
        S: Clone + Eq + Hash,
        N: Fn(&S) -> I,
        I: IntoIterator<Item = S>,
    {
        path.push(current.clone());

        if &current == target {
            all_paths.push(path.clone());
        } else {
            for next in neighbors(&current) {
                find_paths(next, target, neighbors, path, all_paths);
            }
        }

        path.pop();
    }

    let mut all_paths = Vec::new();
    let mut path = Vec::new();
    find_paths(start, &target, &neighbors, &mut path, &mut all_paths);
    all_paths
}

/// DFS with cycle detection for graphs that may have cycles.
/// Tracks visited nodes in the current path to avoid infinite loops.
///
/// # Arguments
/// * `start` - Starting node
/// * `is_target` - Predicate to check if we've reached the goal
/// * `neighbors` - Function that returns all neighboring nodes
///
/// # Returns
/// Option containing a path from start to target if one exists
pub fn dfs_find_path<S, F, N, I>(start: S, is_target: F, neighbors: N) -> Option<Vec<S>>
where
    S: Clone + Eq + Hash,
    F: Fn(&S) -> bool,
    N: Fn(&S) -> I,
    I: IntoIterator<Item = S>,
{
    fn find_inner<S, F, N, I>(
        current: S,
        is_target: &F,
        neighbors: &N,
        visited: &mut HashSet<S>,
        path: &mut Vec<S>,
    ) -> bool
    where
        S: Clone + Eq + Hash,
        F: Fn(&S) -> bool,
        N: Fn(&S) -> I,
        I: IntoIterator<Item = S>,
    {
        path.push(current.clone());

        if is_target(&current) {
            return true;
        }

        visited.insert(current.clone());

        for next in neighbors(&current) {
            if !visited.contains(&next) {
                if find_inner(next, is_target, neighbors, visited, path) {
                    return true;
                }
            }
        }

        path.pop();
        false
    }

    let mut visited = HashSet::new();
    let mut path = Vec::new();

    if find_inner(start, &is_target, &neighbors, &mut visited, &mut path) {
        Some(path)
    } else {
        None
    }
}

// ============================================================================
// Flood Fill
// ============================================================================

/// Flood fill from a starting point, returning all connected states.
///
/// # Arguments
/// * `start` - Starting state
/// * `neighbors` - Function that returns all valid neighboring states
///
/// # Returns
/// Set of all reachable states (including start)
pub fn flood_fill<S, N, I>(start: S, neighbors: N) -> HashSet<S>
where
    S: Clone + Eq + Hash,
    N: Fn(&S) -> I,
    I: IntoIterator<Item = S>,
{
    let mut visited: HashSet<S> = HashSet::new();
    let mut stack: Vec<S> = vec![start];

    while let Some(current) = stack.pop() {
        if visited.contains(&current) {
            continue;
        }

        visited.insert(current.clone());

        for next in neighbors(&current) {
            if !visited.contains(&next) {
                stack.push(next);
            }
        }
    }

    visited
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_simple() {
        // Simple graph: 0 -> 1 -> 2 -> 3
        let result = bfs(0, |&s| s == 3, |&s| {
            if s < 3 { vec![s + 1] } else { vec![] }
        });
        assert_eq!(result.steps, 3);
        assert_eq!(result.state, Some(3));
    }

    #[test]
    fn test_bfs_path() {
        let path = bfs_path(0, |&s| s == 3, |&s| {
            if s < 3 { vec![s + 1] } else { vec![] }
        });
        assert_eq!(path, Some(vec![0, 1, 2, 3]));
    }

    #[test]
    fn test_dfs_count_paths() {
        // Diamond graph: A -> B -> D, A -> C -> D
        let graph: HashMap<&str, Vec<&str>> = [
            ("A", vec!["B", "C"]),
            ("B", vec!["D"]),
            ("C", vec!["D"]),
            ("D", vec![]),
        ].into_iter().collect();

        let count = dfs_count_paths("A", "D", |node| {
            graph.get(node).cloned().unwrap_or_default()
        });
        assert_eq!(count, 2);
    }

    #[test]
    fn test_flood_fill() {
        // Grid-like: fill from (0,0) to all points where x+y <= 2
        let filled = flood_fill((0i32, 0i32), |(x, y)| {
            [(x + 1, *y), (x - 1, *y), (*x, y + 1), (*x, y - 1)]
                .into_iter()
                .filter(|(nx, ny)| *nx >= 0 && *ny >= 0 && nx + ny <= 2)
                .collect::<Vec<_>>()
        });
        assert_eq!(filled.len(), 6); // (0,0), (1,0), (0,1), (2,0), (1,1), (0,2)
    }
}

