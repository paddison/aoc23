use std::collections::HashSet;
use std::fmt::Display;
use std::ops::Index;

static TEST: &str = include_str!("../data/d21t");
static INPUT: &str = include_str!("../data/d21");

#[derive(Debug, Clone, Copy)]
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

    fn get_next_positions_wrapping(&self, (x, y): (isize, isize)) -> HashSet<(isize, isize)> {
        [
            (x + 1, y),
            (x - 1, y),
            (x, y + 1),
            (x, y - 1),
        ]
        .into_iter()
        .filter_map(|pos| match self.get_tile_wrapping(pos) {
            Tile::Garden => Some(pos),
            Tile::Rock => None,
        }) 
        .collect()
    }

    fn get_tile_wrapping(&self, (x, y): (isize, isize)) -> &Tile {
        let wrapped_x = ((x % self.width as isize) + self.width as isize) % self.width as isize;
        let wrapped_y = ((y % self.height as isize) + self.height as isize) % self.height as isize;
        self.tiles.get(wrapped_x as usize + (wrapped_y as usize * self.width)).unwrap()
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
    println!("{width}, {height}");

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
    let mut reachable = HashSet::from([start]);

    for _ in 0..n_steps {
        reachable = reachable.into_iter().fold(HashSet::new(), |acc, pos| {
            map.get_next_positions(pos).union(&acc).copied().collect()
        });
    }

    reachable.len()
}

pub fn get_solution_1() -> usize {
    let (start, map) = parse_input(INPUT);
    calculate_reachable_number_of_tiles(start, map, 64)
}

#[test]
fn test_calculate_reachable_number_of_tiles() {
    let (start, map) = parse_input(INPUT);
    let actual = calculate_reachable_number_of_tiles(start, map, 64);
    println!("{actual}");
}

#[test]
fn test_calculate_reachable_number_of_tiles_wrapping() {
    let (start, map) = parse_input(TEST);
    
    let mut reachable = HashSet::from([(start.0 as isize, start.1 as isize)]);
    let mut prev = 0;

    for _ in 0..1000 {
        reachable = reachable.into_iter().fold(HashSet::new(), |acc, pos| {
            map.get_next_positions_wrapping((pos.0 as isize, pos.1 as isize)).union(&acc).copied().collect()
        });
        println!("len: {}", reachable.len() - prev);
        prev = reachable.len();
    }
    println!("-1 % 5: {}", (((-1) % 5) + 5) % 5);
}
