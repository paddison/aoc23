use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet},
    hash::Hash,
};

#[allow(dead_code)]
static TEST: &str = include_str!("../data/d17t");
#[allow(dead_code)]
static TEST2: &str = include_str!("../data/d17t2");
static INPUT: &str = include_str!("../data/d17");

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as u8))
                .collect()
        })
        .collect()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn is_opposite(&self, other: &Self) -> bool {
        match self {
            Dir::Up => *other == Dir::Down,
            Dir::Right => *other == Dir::Left,
            Dir::Down => *other == Dir::Up,
            Dir::Left => *other == Dir::Right,
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

#[derive(Debug, Eq, Clone, Copy)]
struct Node {
    cost: usize,
    pos: (usize, usize),
    dir: Dir,
    steps: usize,
}

impl Node {
    fn new(cost: usize, pos: (usize, usize), dir: Dir, steps: usize) -> Self {
        Self {
            cost,
            pos,
            dir,
            steps,
        }
    }

    fn can_move_p1(&self, dir: Dir) -> Option<usize> {
        match (self.dir == dir, self.steps == 3) {
            (true, true) => None,
            (true, false) => Some(self.steps + 1),
            _ => Some(1),
        }
    }

    fn can_move_p2(&self, dir: Dir) -> Option<usize> {
        match (self.dir == dir, self.steps) {
            (true, 10) => None,
            (true, steps) => Some(steps + 1),
            (false, steps) if steps < 4 => None,
            _ => Some(1),
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.dir == other.dir && self.steps == other.steps
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
        self.dir.hash(state);
        self.steps.hash(state);
    }
}

fn dijkstra<F>(map: &[Vec<u8>], can_move: F, min_steps: usize) -> usize
where
    F: Fn(&Node, Dir) -> Option<usize>,
{
    assert!(!map.is_empty());
    // starting positions and goal
    let down = Node::new(0, (0, 0), Dir::Down, 0);
    let right = Node::new(0, (0, 0), Dir::Right, 0);
    let goal = (map[0].len() - 1, map.len() - 1);

    let mut seen = HashSet::new();
    let mut queue = BinaryHeap::from([Reverse(down), Reverse(right)]);

    while let Some(Reverse(node)) = queue.pop() {
        // if check if we've been here before
        if !seen.insert(node) {
            continue;
        }

        // have we reached the goal?
        if node.pos == goal && node.steps >= min_steps {
            return node.cost;
        }

        for dir in [Dir::Up, Dir::Right, Dir::Down, Dir::Left] {
            // cannot move in opposite direction
            if dir.is_opposite(&node.dir) {
                continue;
            }

            // check if a move in that direction is possible
            if let Some(steps) = can_move(&node, dir) {
                let pos = node.pos.move_pos(&dir);

                // try to get the cost of the next tile
                let cost = match map.get(pos.1).map(|r| r.get(pos.0)) {
                    Some(Some(cost)) => *cost as usize,
                    _ => continue,
                };

                queue.push(Reverse(Node::new(cost + node.cost, pos, dir, steps)));
            }
        }
    }
    panic!("couldn't reach the goal");
}

pub fn get_solution_1() -> usize {
    dijkstra(&parse_input(INPUT), Node::can_move_p1, 0)
}

pub fn get_solution_2() -> usize {
    dijkstra(&parse_input(INPUT), Node::can_move_p2, 4)
}
