use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;
use std::ops::Index;

#[allow(unused)]
static TEST: &str = include_str!("../data/d21t");
static INPUT: &str = include_str!("../data/d21");

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Garden,
    Rock,
}

impl From<Tile> for char {
    fn from(tile: Tile) -> char {
        match tile {
            Tile::Garden => '.',
            Tile::Rock => '#',
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

#[derive(Clone)]
struct Map {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();
        for row in 0..self.height {
            buf.extend(
                self.tiles[self.width * row..self.width * (row + 1)]
                    .iter()
                    .map(|t| char::from(*t)),
            );

            buf.push('\n');
        }
        write!(f, "{buf}")
    }
}

impl Map {
    fn get_next_positions(&self, (x, y): (usize, usize)) -> HashSet<(usize, usize)> {
        [
            (x + 1, y),
            (x.wrapping_sub(1), y),
            (x, y + 1),
            (x, y.wrapping_sub(1)),
        ]
        .into_iter()
        .filter_map(|pos| match self.get_tile(pos) {
            Some(Tile::Garden) => Some(pos),
            _ => None,
        })
        .collect()
    }

    fn get_tile(&self, (x, y): (usize, usize)) -> Option<&Tile> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.tiles.get(x + (y * self.width))
    }
}

impl Index<(usize, usize)> for Map {
    type Output = Tile;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.tiles[x + (y * self.width)]
    }
}

fn parse_input(input: &str) -> ((usize, usize), Map) {
    let mut tiles = Vec::new();
    let width = input.find('\n').unwrap();
    let height = input.lines().count();
    let mut start = (0, 0);
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                '.' => tiles.push(Tile::Garden),
                '#' => tiles.push(Tile::Rock),
                _ => {
                    start = (x, y);
                    tiles.push(Tile::Garden);
                }
            }
        }
    }

    assert_ne!(start, (0, 0));
    assert_eq!(tiles.len(), width * height);

    (
        start,
        Map {
            tiles,
            width,
            height,
        },
    )
}

fn calculate_reachable_number_of_tiles(start: (usize, usize), map: Map, n_steps: usize) -> usize {
    let mut even = HashSet::from([start]);
    let mut odd = HashSet::from([]);

    for i in 0..n_steps {
        let (cur, next) = if i % 2 == 0 {
            (&even, &mut odd)
        } else {
            (&odd, &mut even)
        };
        for pos in cur {
            for pos in map.get_next_positions(*pos) {
                next.insert(pos);
            }
        }
    }

    if n_steps % 2 == 0 {
        even.len()
    } else {
        odd.len()
    }
}

fn get_shortest_paths_per_tile(start: (usize, usize), map: Map) -> HashMap<(usize, usize), usize> {
    let mut queue = VecDeque::from([(start, 0)]);
    let mut seen = HashMap::new();

    while let Some((pos, dist)) = queue.pop_front() {
        if seen.contains_key(&pos) {
            continue;
        }
        seen.insert(pos, dist);

        for next_pos in map.get_next_positions(pos) {
            if !seen.contains_key(&next_pos) {
                queue.push_back((next_pos, dist + 1));
            }
        }
    }

    seen
}

/*
* Taken from https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
* which contains a very good explanation on how this works
*/
fn calculate_visited_tiles(visited_tiles: HashMap<(usize, usize), usize>, dim: usize) -> usize {
    let even_corners = visited_tiles
        .values()
        .filter(|steps| **steps % 2 == 0 && **steps > 65)
        .count();

    let odd_corners = visited_tiles
        .values()
        .filter(|steps| **steps % 2 == 1 && **steps > 65)
        .count();

    let even_full = visited_tiles
        .values()
        .filter(|steps| **steps % 2 == 0)
        .count();

    let odd_full = visited_tiles
        .values()
        .filter(|steps| **steps % 2 == 1)
        .count();

    let n_squares = (26501365 - (dim / 2)) / dim;

    (n_squares + 1).pow(2) * odd_full + n_squares.pow(2) * even_full - (n_squares + 1) * odd_corners
        + n_squares * even_corners
}

pub fn get_solution_1() -> usize {
    let (start, map) = parse_input(INPUT);
    calculate_reachable_number_of_tiles(start, map, 64)
}

pub fn get_solution_2() -> usize {
    let (start, map) = parse_input(INPUT);
    let dim = map.height;
    let shortest_paths_per_tile = get_shortest_paths_per_tile(start, map);
    calculate_visited_tiles(shortest_paths_per_tile, dim)
}
