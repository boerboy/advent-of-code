#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Coords {
    pub x: i64,
    pub y: i64
}

impl Coords {
    pub const NORTH: Coords = Coords {x: 0, y: -1};
    pub const EAST: Coords = Coords { x: 1, y: 0 };
    pub const SOUTH: Coords = Coords { x: 0, y: 1 };
    pub const WEST: Coords = Coords { x: -1, y: 0 };
    pub const NORTH_EAST: Coords = Coords { x: 1, y: -1 };
    pub const NORTH_WEST: Coords = Coords { x: -1, y: -1 };
    pub const SOUTH_EAST: Coords = Coords { x: 1, y: 1 };
    pub const SOUTH_WEST: Coords = Coords { x: -1, y: 1 };
    pub const CARDINALS: [Coords; 4] = [
        Coords::NORTH, Coords:: EAST, Coords:: SOUTH, Coords::WEST
    ];


    pub const DIAGONALS: [Coords; 4] = [
        Coords::NORTH_EAST, Coords::NORTH_WEST, Coords::SOUTH_EAST, Coords::SOUTH_WEST
    ];

    pub const DIRECTIONS: [Coords; 8] = [
        Coords::NORTH, Coords:: EAST, Coords:: SOUTH, Coords::WEST,
        Coords::NORTH_EAST, Coords::NORTH_WEST, Coords::SOUTH_EAST, Coords::SOUTH_WEST
    ];

    pub const RIGHT_ANGLES: [(Coords, Coords, Coords); 4] = [
        (Coords::NORTH, Coords::EAST, Coords::NORTH_EAST),
        (Coords::NORTH, Coords::WEST, Coords::NORTH_WEST),
        (Coords::SOUTH, Coords::EAST, Coords::SOUTH_EAST),
        (Coords::SOUTH, Coords::WEST, Coords::SOUTH_WEST)
    ];

    pub fn subtract(&self, other: Coords) -> Coords {
        Coords { x: self.x - other.x, y: self.y - other.y }
    }

    pub fn add(&self, other: Coords) -> Coords {
        Coords { x: self.x + other.x, y: self.y + other.y }
    }

    pub fn multiply(&self, other: Coords) -> Coords {
        Coords { x: self.x * other.x, y: self.y * other.y }
    }

    pub fn wrap(&self, bounds: Coords) -> Coords {
        Coords {
            x: self.x.rem_euclid(bounds.x),
            y: self.y.rem_euclid(bounds.y),
        }
    }

    pub fn add_const(&self, constant: i64) -> Coords {
        Coords { x: self.x + constant, y:  self.y + constant }
    }

    pub fn multiply_const(&self, constant: i64) -> Coords {
        Coords { x: self.x * constant, y:  self.y * constant }
    }
    pub fn is_outside_bounds(&self, max_x: i32, max_y: i32) -> bool {
        self.x < 0 || self.x > max_x as i64 || self.y < 0 || self.y > max_y as i64
    }

    pub fn neighbors(&self) -> Vec<Coords> {
        Coords::CARDINALS.iter().map(|d| self.add(*d)).collect()
    }

    pub fn is_longitudinal(&self) -> bool {
        self == &Coords::NORTH || self == &Coords::SOUTH
    }

}
