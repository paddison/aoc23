use std::{
    collections::{hash_map::Entry, HashMap, HashSet, VecDeque},
    fmt::Display,
};

#[allow(unused)]
static TEST: &str = include_str!("../data/d23t");
static INPUT: &str = include_str!("../data/d23");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    U,
    D,
    L,
    R,
}

impl Dir {
    fn is_opposite(&self, other: Dir) -> bool {
        match (self, other) {
            (Self::U, Self::D) => true,
            (Self::D, Self::U) => true,
            (Self::L, Self::R) => true,
            (Self::R, Self::L) => true,
            _ => false,
        }
    }
}

impl From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::U,
            'v' => Self::D,
            '<' => Self::L,
            '>' => Self::R,
            _ => panic!("Invalid char for tile"),
        }
    }
}

impl From<Dir> for char {
    fn from(value: Dir) -> Self {
        match value {
            Dir::U => '^',
            Dir::D => 'v',
            Dir::L => '<',
            Dir::R => '>',
        }
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Path,
    Forest,
    Slope(Dir),
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Path,
            '#' => Self::Forest,
            slope => Self::Slope(slope.into()),
        }
    }
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Path => '.',
            Tile::Forest => '#',
            Tile::Slope(dir) => dir.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn update_position(&self, dir: Dir) -> Self {
        match dir {
            Dir::U => Self {
                x: self.x,
                y: self.y.wrapping_sub(1),
            },
            Dir::D => Self {
                x: self.x,
                y: self.y + 1,
            },
            Dir::L => Self {
                x: self.x.wrapping_sub(1),
                y: self.y,
            },
            Dir::R => Self {
                x: self.x + 1,
                y: self.y,
            },
        }
    }

    fn can_move(&self, dir: Dir, map: &Map) -> Option<Self> {
        let next = self.update_position(dir);
        match map.get_tile(next.x, next.y) {
            Some(tile) => match tile {
                Tile::Path => Some(next),
                Tile::Forest => None,
                Tile::Slope(slope_dir) => {
                    if dir == slope_dir {
                        Some(next)
                    } else {
                        None
                    }
                }
            },
            None => None,
        }
    }
}

/*
* Coordinates behave as following:
*  x0123...
* y
* 0 #######
* 1 #######
* 2 #######
* 3 #######
* 4 #######
*
*/
struct Map {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Map {
    fn get_tile(&self, x: usize, y: usize) -> Option<Tile> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let idx = x + self.width * y;

        assert!(idx < self.tiles.len());
        Some(self.tiles[idx])
    }

    fn get_end(&self) -> Point {
        Point {
            x: self.width - 2,
            y: self.height - 1,
        }
    }

    fn remove_slopes(mut self) -> Self {
        for tile in &mut self.tiles {
            if let Tile::Slope(_) = tile {
                *tile = Tile::Path;
            }
        }

        self
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tiles = String::new();

        for y in 0..self.height {
            let line = self.tiles[y * self.width..(y + 1) * self.width]
                .iter()
                .copied()
                .map(|t| char::from(t))
                .collect::<String>();
            tiles.push_str(&line);
            tiles.push('\n');
        }

        write!(f, "{}", tiles)
    }
}

fn parse_input(inp: &str) -> Map {
    let lines = inp.lines().map(|l| l.trim()).collect::<Vec<&str>>();
    let height = lines.len();
    assert!(height > 0);
    let width = lines[0].len();
    assert!(lines.iter().all(|l| l.len() == width));

    let tiles = lines
        .into_iter()
        .flat_map(|l| l.chars())
        .map(|t| t.into())
        .collect();

    Map {
        tiles,
        width,
        height,
    }
}

fn compact_edges(
    map: &Map,
    start: Point,
    is_directed: bool,
) -> HashMap<Point, HashSet<(Point, usize)>> {
    let mut graph = HashMap::new();

    let mut visited = HashSet::from([start]);
    let mut queue = vec![(start, Dir::D)];

    while let Some((parent, dir)) = queue.pop() {
        if parent.can_move(dir, map).is_none() {
            continue;
        }
        let next = parent.update_position(dir);
        let (junction, next_points, cost) =
            match follow_path(map, next, dir, HashSet::from([parent])) {
                None => continue,
                Some(tpl) => tpl,
            };
        graph
            .entry(parent)
            .or_insert(HashSet::new())
            .insert((junction, cost));

        if !is_directed {
            graph
                .entry(junction)
                .or_insert(HashSet::new())
                .insert((parent, cost));
        }

        if visited.contains(&junction) || junction == map.get_end() {
            continue;
        }

        visited.insert(junction);

        for (_, dir) in next_points {
            queue.push((junction, dir));
        }
    }

    graph
}

fn follow_path(
    map: &Map,
    point: Point,
    dir: Dir,
    mut visited: HashSet<Point>,
) -> Option<(Point, Vec<(Point, Dir)>, usize)> {
    if point == map.get_end() {
        return Some((point, Vec::new(), visited.len()));
    }

    let next_points = [Dir::U, Dir::D, Dir::L, Dir::R]
        .into_iter()
        .filter(|d| !d.is_opposite(dir))
        .filter_map(|d| point.can_move(d, map).map(|p| (p, d)))
        .collect::<Vec<(Point, Dir)>>();

    assert_ne!(next_points.len(), 0);

    if next_points.len() > 1 {
        return Some((point, next_points, visited.len()));
    }

    assert_eq!(next_points.len(), 1);

    let (next, dir) = next_points[0];
    visited.insert(next);
    follow_path(map, next, dir, visited)
}

fn bfs(start: Point, end: Point, graph: HashMap<Point, HashSet<(Point, usize)>>) -> usize {
    /* Visited points are encoded into a 64 bit integer */
    let mut costs = Vec::new();
    let mut queue = VecDeque::from([(start, 0, 0)]);
    let mut longest_so_far: HashMap<(Point, usize), usize> = HashMap::new();
    /* give each point a unique bitmask */
    let point_to_usize_table: HashMap<_, _> = graph
        .iter()
        .flat_map(|(parent, points)| {
            let mut points = points.iter().copied().map(|(p, _)| p).collect::<Vec<_>>();
            points.push(*parent);
            points
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .enumerate()
        .map(|(i, p)| (p, 1 << i))
        .collect();

    while let Some((point, cost, mut visited)) = queue.pop_front() {
        let point_as_usize = point_to_usize_table.get(&point).unwrap();
        match longest_so_far.entry((point, visited)) {
            Entry::Occupied(length) if length.get() >= &cost => continue,
            Entry::Occupied(mut length) => *length.get_mut() = cost,
            Entry::Vacant(_) => _ = longest_so_far.insert((point, visited), cost),
        }

        if point == end {
            costs.push(cost);
            continue;
        }

        /* Add point to the set */
        visited |= point_as_usize;

        for (neighbour, cur_cost) in graph.get(&point).unwrap() {
            let neighbour_as_usize = point_to_usize_table.get(&neighbour).unwrap();
            /* check if neighbour has been seen */
            if (visited & neighbour_as_usize) == 0 {
                queue.push_back((*neighbour, cost + cur_cost, visited));
            }
        }
    }

    costs.into_iter().max().unwrap_or(0)
}

pub(crate) fn get_solution_1() -> usize {
    let map = parse_input(INPUT);
    let start = Point { x: 1, y: 0 };
    let end = map.get_end();
    let graph = compact_edges(&map, Point { x: 1, y: 0 }, true);
    bfs(start, end, graph)
}

pub(crate) fn get_solution_2() -> usize {
    let map = parse_input(INPUT).remove_slopes();
    let start = Point { x: 1, y: 0 };
    let end = map.get_end();
    let graph = compact_edges(&map, Point { x: 1, y: 0 }, false);
    bfs(start, end, graph)
}

#[test]
fn test_parse_input() {
    let map = parse_input(TEST);
    println!("{map}");
    let map_str = format!("{map}");

    assert_eq!(TEST, map_str.as_str());
}

#[test]
fn test_remove_slopes() {
    let map = parse_input(INPUT).remove_slopes();
    println!("{map}");
}
