use std::collections::HashSet;
use std::ops::Index;

#[allow(dead_code)]
static TEST: &str = include_str!("../data/d10t");
#[allow(dead_code)]
static TEST2: &str = include_str!("../data/d10t2");
#[allow(dead_code)]
static TEST3: &str = include_str!("../data/d10t3");
#[allow(dead_code)]
static TEST4: &str = include_str!("../data/d10t4");
static INPUT: &str = include_str!("../data/d10");

type Position = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
    Start,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            '.' => Self::Ground,
            _ => Self::Start,
        }
    }
}

#[derive(Clone, Copy)]
enum Dir {
    North,
    East,
    South,
    West,
}

struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Index<Position> for Map {
    type Output = Tile;

    fn index(&self, index: Position) -> &Self::Output {
        if let Some(Some(tile)) = self.tiles.get(index.1).map(|row| row.get(index.0)) {
            tile
        } else {
            &Tile::Ground
        }
    }
}

impl Map {
    fn new(start: Position, tiles: Vec<Vec<Tile>>) -> Self {
        let mut map = Self { tiles };
        map.set_start_tile(start, map.determine_start_tile(start));
        map
    }

    fn can_move(&self, pos: Position, dir: Dir) -> bool {
        if !self.is_valid_direction(pos, dir) {
            return false;
        }
        match dir {
            Dir::North => matches!(
                self[(pos.0, pos.1.overflowing_sub(1).0)],
                Tile::SouthWest | Tile::SouthEast | Tile::Vertical
            ),
            Dir::East => matches!(
                self[(pos.0 + 1, pos.1)],
                Tile::SouthWest | Tile::NorthWest | Tile::Horizontal
            ),
            Dir::South => matches!(
                self[(pos.0, pos.1 + 1)],
                Tile::NorthEast | Tile::NorthWest | Tile::Vertical
            ),
            Dir::West => matches!(
                self[(pos.0.overflowing_sub(1).0, pos.1)],
                Tile::NorthEast | Tile::SouthEast | Tile::Horizontal
            ),
        }
    }

    fn is_valid_direction(&self, pos: Position, dir: Dir) -> bool {
        let tile = self[pos];

        match dir {
            Dir::North => matches!(tile, Tile::NorthEast | Tile::NorthWest | Tile::Vertical),
            Dir::East => matches!(tile, Tile::NorthEast | Tile::SouthEast | Tile::Horizontal),
            Dir::South => matches!(tile, Tile::SouthEast | Tile::SouthWest | Tile::Vertical),
            Dir::West => matches!(tile, Tile::NorthWest | Tile::SouthWest | Tile::Horizontal),
        }
    }

    fn determine_start_tile(&self, start: Position) -> Tile {
        // check if we can go south, east left or right
        if self.can_move(start, Dir::North) && self.can_move(start, Dir::East) {
            // all norths
            Tile::NorthEast
        } else if self.can_move(start, Dir::North) && self.can_move(start, Dir::South) {
            Tile::Vertical
        } else if self.can_move(start, Dir::North) && self.can_move(start, Dir::West) {
            Tile::NorthWest
        } else if self.can_move(start, Dir::East) && self.can_move(start, Dir::South) {
            // all easts
            Tile::SouthEast
        } else if self.can_move(start, Dir::East) && self.can_move(start, Dir::West) {
            Tile::Horizontal
        } else {
            Tile::SouthWest
        }
    }

    fn set_start_tile(&mut self, start: Position, tile: Tile) {
        if let Some(Some(cur_tile)) = self.tiles.get_mut(start.1).map(|row| row.get_mut(start.0)) {
            *cur_tile = tile;
        }
    }
}

fn parse_input(input: &str) -> (Position, Map) {
    let tiles: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.into()).collect())
        .collect();
    let position = tiles
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, tile)| match *tile == Tile::Start {
                    true => Some((x, y)),
                    false => None,
                })
        })
        .unwrap();
    let map = Map::new(position, tiles);
    (position, map)
}

fn move_position(dir: Dir, position: Position) -> Position {
    match dir {
        Dir::North => (position.0, position.1 - 1),
        Dir::East => (position.0 + 1, position.1),
        Dir::South => (position.0, position.1 + 1),
        Dir::West => (position.0 - 1, position.1),
    }
}

fn traverse(start: Position, map: &Map) -> HashSet<Position> {
    let mut cur = start;
    let mut prev = (usize::MAX, usize::MAX);
    let dirs = [Dir::North, Dir::East, Dir::South, Dir::West];
    let mut pipe = HashSet::from([start]);

    loop {
        for dir in dirs {
            if map.can_move(cur, dir) {
                let next = move_position(dir, cur);
                if next != prev {
                    prev = cur;
                    cur = next;
                    pipe.insert(cur);
                    break;
                }
            }
        }
        if cur == start {
            break;
        }
    }
    pipe
}

// check for parity in tiles:
// passing a '|' means if we're outside the loop, then we will be inside afterwards and
// vice versa (it flips)
// similarly the combination 'F'...'J' and 'L' .. '7' cause a flip,
// (think of these as vertical pipes which are stretched in the horizontal axis.
// writing out all the combinations, it suffices to flip on encountering 'J', 'L' or '|'
// tiles (or 'F', '7' and '|')
fn count_inside_tiles(map: &Map, pipe: HashSet<Position>) -> usize {
    // flip on 'J', 'L' and '|'
    let flip_tiles = [Tile::NorthWest, Tile::NorthEast, Tile::Vertical];
    let mut is_outside = true;
    let mut inside_tiles = 0;

    for (y, row) in map.tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if pipe.contains(&(x, y)) && flip_tiles.contains(tile) {
                is_outside = !is_outside;
            } else if !pipe.contains(&(x, y)) && !is_outside {
                inside_tiles += 1;
            }
        }
    }

    inside_tiles
}

pub fn get_solution_1() -> usize {
    let (position, map) = parse_input(INPUT);

    traverse(position, &map).len() / 2
}

pub fn get_solution_2() -> usize {
    let (position, map) = parse_input(INPUT);
    let pipe = traverse(position, &map);

    count_inside_tiles(&map, pipe)
}

#[test]
fn test_parse_input() {
    let (position, map) = parse_input(TEST);
    println!("{position:?}\n{:?}", map.tiles);
}

#[test]
fn test_determine_start_tile() {
    let (position, map) = parse_input(TEST);
    let start_tile = map.determine_start_tile(position);
    assert_eq!(start_tile, Tile::SouthEast);
}

#[test]
fn test_traverse() {
    let (position, mut map) = parse_input(INPUT);
    let start_tile = map.determine_start_tile(position);
    map.set_start_tile(position, start_tile);

    let res = traverse(position, &map).len();
    println!("{:?}", res / 2);
}

#[test]
fn test_count_inside_tiles() {
    let (position, mut map) = parse_input(INPUT);
    let start_tile = map.determine_start_tile(position);
    map.set_start_tile(position, start_tile);

    let pipe = traverse(position, &map);
    println!("{}", count_inside_tiles(&map, pipe));
}
