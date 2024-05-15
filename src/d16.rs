use std::collections::{HashSet, VecDeque};

#[allow(dead_code)]
static TEST: &str = include_str!("../data/d16t");
static INPUT: &str = include_str!("../data/d16");

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Dir {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl From<usize> for Dir {
    fn from(inp: usize) -> Self {
        match inp {
            0 => Self::Up,
            1 => Self::Right,
            2 => Self::Down,
            _ => Self::Left,
        }
    }
}

impl Dir {
    fn turn_clockwise(self) -> Self {
        ((self as usize + 1) % 4).into()
    }

    fn turn_counter_clockwise(self) -> Self {
        ((self as usize + 7) % 4).into()
    }

    fn turn(self, mirror: char) -> Self {
        match (self, mirror) {
            (Self::Up | Self::Down, '/') | (Self::Left | Self::Right, '\\') => {
                self.turn_clockwise()
            }
            _ => self.turn_counter_clockwise(),
        }
    }

    fn split(self, splitter: char) -> (Self, Option<Self>) {
        match (self, splitter) {
            (Self::Up | Self::Down, '-') => (Self::Left, Some(Self::Right)),
            (Self::Left | Self::Right, '|') => (Self::Up, Some(Self::Down)),
            (dir, _) => (dir, None),
        }
    }

    fn next_dir(self, tile: char) -> (Self, Option<Self>) {
        match tile {
            turn if matches!(tile, '/' | '\\') => (self.turn(turn), None),
            split => self.split(split),
        }
    }
}

trait Position {
    fn move_pos(self, dir: &Dir) -> Self;
}

impl Position for (usize, usize) {
    fn move_pos(self, dir: &Dir) -> Self {
        match dir {
            Dir::Up => (self.0, self.1.wrapping_sub(1)),
            Dir::Right => (self.0 + 1, self.1),
            Dir::Down => (self.0, self.1 + 1),
            Dir::Left => (self.0.wrapping_sub(1), self.1),
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect()
}

fn bfs(map: &[Vec<char>], start: (usize, usize), dir: Dir) -> HashSet<(usize, usize)> {
    let mut seen: HashSet<((usize, usize), Dir)> = HashSet::new();
    let mut queue = VecDeque::from([(start, dir)]);

    while let Some((pos, dir)) = queue.pop_front() {
        // check the tile first if its valid, and then update the seen set so
        // there are no invalid positions inserted into the seen set
        // find out where we need to go
        let tile = match map.get(pos.1).map(|row| row.get(pos.0)) {
            Some(Some(tile)) => *tile,
            _ => continue,
        };
        // check if we've been here before
        if !seen.insert((pos, dir)) {
            continue; // we've been here before
        }
        // move the position
        let (first_dir, second_dir_opt) = dir.next_dir(tile);
        let first_pos = pos.move_pos(&first_dir);

        queue.push_back((first_pos, first_dir));

        if let Some(second_dir) = second_dir_opt {
            let second_pos = pos.move_pos(&second_dir);
            queue.push_back((second_pos, second_dir));
        }
    }

    seen.into_iter().map(|(pos, _)| pos).collect()
}

fn bfs_all_positions(map: &[Vec<char>]) -> HashSet<(usize, usize)> {
    let width = map.first().map(|row| row.len()).unwrap_or(0);
    let height = map.len();

    (0..width)
        .map(|x| ((x, 0), Dir::Down))
        .chain((0..width).map(|x| ((x, height - 1), Dir::Up)))
        .chain((0..height).map(|y| ((0, y), Dir::Right)))
        .chain((0..height).map(|y| ((width - 1, y), Dir::Left)))
        .map(|(pos, dir)| bfs(map, pos, dir))
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap_or(HashSet::new())
}

pub fn get_solution_1() -> usize {
    let map = parse_input(INPUT);
    bfs(&map, (0, 0), Dir::Right).len()
}

pub fn get_solution_2() -> usize {
    let map = parse_input(INPUT);
    bfs_all_positions(&map).len()
}
