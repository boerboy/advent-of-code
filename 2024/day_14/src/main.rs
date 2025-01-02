use std::collections::HashMap;
use std::hash::Hash;
use common::coords::Coords;
use common::reader::read_csv;
use regex::Regex;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct RobotDetails {
    position: Coords,
    velocity: Coords
}

fn parse_coords(re: &Regex, input: &String) -> Coords {
    re.captures_iter(input)
        .map(|capture| Coords {
            x: capture[1].parse().unwrap(),
            y: capture[2].parse().unwrap(),
        })
        .next()
        .unwrap()
}

fn parse_input() -> Vec<RobotDetails> {
    let re = &Regex::new(r".*(-?\d+),(-?\d+)").expect("Successful regex instantiation");
    read_csv::<(String, String)>("./resources/test.csv", b' ')
        .expect("Successful input read")
        .iter()
        .map(|(position_str, velocity_str)| {
            let position = parse_coords(re, position_str);
            let velocity = parse_coords(re, velocity_str);
            RobotDetails {
                position,
                velocity
            }
        })
        .collect()
}

fn walk_robots(
    input: &Vec<RobotDetails>,
    seconds: i64,
    bounds: &Coords
) -> HashMap<Coords, i64> {
    input.iter().fold(HashMap :: <Coords, i64> :: new(), |mut acc, details| {
        let final_position = details
            .position
            .add(details.velocity.multiply_const(seconds))
            .wrap(*bounds);

        *acc.entry(final_position).or_insert(0i64) += 1i64;
        acc
    })
}

fn calc_safety_factor(input: &Vec<RobotDetails>, seconds: i64, bounds: &Coords) -> i64 {
    let walked_positions = walk_robots(input, seconds, bounds);
    let middle_coords = Coords {x :bounds.x  / 2i64, y : bounds.y / 2i64 };
    let (north_west, north_east, south_east, south_west) = walked_positions.iter().fold(
        (0, 0, 0, 0),
        |(north_west, north_east, south_east, south_west), (Coords { x, y }, count)| {
            match (*x < middle_coords.x,  *y < middle_coords.y) {
                _   if *x == middle_coords.x || *y == middle_coords.y => (north_west, north_east, south_east, south_west),
                (true, true)   => (north_west + count, north_east, south_east, south_west),   // North-West
                (false, true)  => (north_west, north_east + count, south_east, south_west),  // North-East
                (true, false)  => (north_west, north_east, south_east + count, south_west),  // South-East
                (false, false) => (north_west, north_east, south_east, south_west + count),  // South-West
            }
        }
    );
    north_west * north_east * south_east * south_west
}

fn main() {
    let input = &parse_input();
    let part_1 = calc_safety_factor(input, 100, &Coords {x: 11, y: 7});
    println!("Part 1: {:?}", part_1);
    let part_2 = 0;
    println!("Part 2: {:?}", part_2)
}