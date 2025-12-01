# Advent of Code 2025

My solutions for [Advent of Code 2025](https://adventofcode.com/2025) in Rust.

## Project Structure

```
2025/
├── Cargo.toml          # Workspace root
├── common/             # Shared utilities library
│   └── src/
│       ├── lib.rs      # Library exports & prelude
│       ├── coords.rs   # 2D coordinate utilities
│       ├── grid.rs     # 2D grid operations
│       ├── input.rs    # Input parsing utilities
│       └── math.rs     # Mathematical helpers
└── day_XX/             # Individual day solutions
    ├── Cargo.toml
    ├── src/
    │   └── main.rs
    └── resources/
        ├── input.txt   # Puzzle input
        └── test.txt    # Example input
```

## Usage

### Running a specific day

```bash
# From workspace root (2025/)
cargo run -p day_01

# Or from the day's directory
cd day_01
cargo run
```

### Running with release optimizations

```bash
cargo run -p day_01 --release
```

### Running tests

```bash
# All tests
cargo test

# Specific day
cargo test -p day_01

# With output
cargo test -p day_01 -- --nocapture
```

### Building all days

```bash
cargo build --release
```

## Creating a New Day

1. Copy the `day_01` directory to `day_XX` (e.g., `day_02`)
2. Update `name` in `day_XX/Cargo.toml` to `"day_XX"`
3. Update the `[[bin]]` name to `"day_XX"`
4. Update the header comment in `src/main.rs`
5. Paste your puzzle input into `resources/input.txt`
6. Paste the example input into `resources/test.txt`
7. Implement your solution in `src/main.rs`

Or use this script from the workspace root:
```bash
# Create day 02
DAY=02
cp -r day_01 day_$DAY
sed -i '' "s/day_01/day_$DAY/g" day_$DAY/Cargo.toml
sed -i '' "s/Day 01/Day $DAY/g" day_$DAY/src/main.rs
sed -i '' "s|day/1|day/${DAY#0}|g" day_$DAY/src/main.rs
> day_$DAY/resources/input.txt
```

## Common Library

The `common` crate provides utilities useful for AoC puzzles:

### Quick Import

```rust
use common::prelude::*;
```

This imports:
- `Coord`, `Direction` - 2D coordinate types
- `Grid` - 2D grid container
- `read_lines`, `read_grid`, `read_csv`, `read_string` - Input parsers
- `gcd` - Greatest common divisor
- `anyhow::{Result, Context, bail, anyhow}` - Error handling
- `itertools::Itertools` - Iterator extensions
- `HashMap`, `HashSet`, `VecDeque` - Collections

### Coordinates (`coords.rs`)

```rust
use common::Coord;

let pos = Coord::new(3, 4);
let moved = pos + Coord::NORTH;  // Move up
let neighbors = pos.neighbors(); // 4-connected neighbors
let dist = pos.manhattan_distance(Coord::ORIGIN);
```

### Grid (`grid.rs`)

```rust
use common::Grid;

let grid = Grid::from_str("ABC\nDEF\nGHI");
let value = grid.get(Coord::new(0, 0)); // Some(&'A')
let start = grid.find(&'S'); // Find coordinates of 'S'

for (coord, value) in grid.iter() {
    println!("{}: {}", coord, value);
}
```

### Input Parsing (`input.rs`)

```rust
use common::prelude::*;

// Read lines
let lines = read_lines("input.txt")?;

// Read as character grid
let grid = read_grid("input.txt")?;

// Read numbers
let nums: Vec<i64> = read_parsed("input.txt")?;

// Extract integers from string
let coords: Vec<i64> = extract_integers("pos=<-1,2,3>");
```

### Math Utilities (`math.rs`)

```rust
use common::math::*;

let g = gcd(48, 18);        // 6
let l = lcm(4, 6);          // 12
let l_all = lcm_all(&[2, 3, 4]); // 12

// Modular arithmetic
let inv = mod_inverse(3, 7); // Some(5)
let pow = mod_pow(2, 10, 1000); // 24
```

## License

MIT

