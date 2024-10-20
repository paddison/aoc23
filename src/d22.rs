use std::collections::HashMap;

#[allow(unused)]
static TEST: &str = include_str!("../data/d22t");
static INPUT: &str = include_str!("../data/d22");

static X: usize = 0;
static Y: usize = 1;
static Z: usize = 2;

#[derive(Debug, PartialEq, Hash, Clone, Copy)]
struct Brick {
    start: [usize; 3], // [x, y, z]
    end: [usize; 3],
}

impl Brick {
    fn drop_down_to(self, z: usize) -> Self {
        let height = self.end[Z] - self.start[Z];
        Self {
            start: [self.start[X], self.start[Y], z],
            end: [self.end[X], self.end[Y], z + height],
        }
    }

    fn supports(&self, other: &Self) -> bool {
        let x_overlap = self.overlaps(other, X);
        let y_overlap = self.overlaps(other, Y);
        let z_touch = self.end[Z] + 1 == other.start[Z];

        x_overlap && y_overlap && z_touch
    }

    fn overlaps(&self, other: &Self, coord: usize) -> bool {
        assert!(coord <= Z);
        Self::overlaps_helper(
            (self.start[coord], self.end[coord]),
            (other.start[coord], other.end[coord]),
        )
    }

    fn overlaps_helper(a: (usize, usize), b: (usize, usize)) -> bool {
        a.0 <= b.0 && a.1 >= b.0 || a.0 >= b.0 && a.0 <= b.1
    }

    // check if any other bricks rest on this self
    fn safe_to_disintegrate(&self, others: &[Self]) -> bool {
        // in order to delete a brick, there needs to be at least one other brick
        // which supports all the brick that the brick to delete also supports

        // figure out which bricks rest on this brick
        let mut supported: Vec<&Brick> = others
            .iter()
            .filter(|b| b != &self && self.supports(b))
            .collect();

        if supported.is_empty() {
            return true;
        }

        // see if there are other brick, which in total support these bricks
        // the bricks need to have the same top Z coordinate as this brick
        // if this brick doesn't support any bricks, it is safe to
        // disintegrate it regardless
        for brick in others
            .iter()
            .filter(|b| b != &self && b.end[Z] == self.end[Z])
        {
            // filter all bricks who aren't supported by 'brick'
            supported = supported
                .into_iter()
                .filter(|s| !brick.supports(s))
                .collect();
        }
        supported.is_empty()
    }

    fn disintegrate(&self, bricks: &[Self]) -> usize {
        let bricks: Vec<Brick> = bricks.iter().filter(|b| &self != b).cloned().collect();
        drop_bricks_counted(bricks, self.end[Z])
    }
}

fn parse_input(inp: &str) -> Vec<Brick> {
    let mut bricks = Vec::new();

    for line in inp.lines() {
        let delim = line.find('~').unwrap();
        let start: Vec<_> = line[0..delim]
            .split(',')
            .filter_map(|num| str::parse::<usize>(num).ok())
            .collect();
        let end: Vec<_> = line[delim + 1..]
            .split(',')
            .filter_map(|num| str::parse::<usize>(num).ok())
            .collect();

        assert_eq!(start.len(), 3);
        assert_eq!(end.len(), 3);

        let brick = Brick {
            start: [start[X], start[Y], start[Z]],
            end: [end[X], end[Y], end[Z]],
        };

        assert!(brick.start[X] <= brick.end[X]);
        assert!(brick.start[Y] <= brick.end[Y]);
        assert!(brick.start[Z] <= brick.end[Z]);

        bricks.push(brick);
    }

    bricks
}

fn drop_bricks(mut bricks: Vec<Brick>) -> Vec<Brick> {
    // sort bricks to according to their z position
    bricks.sort_by(|a, b| a.start[Z].cmp(&b.start[Z]));
    // dropped_bricks and bricks have to be disjoint!
    let mut dropped_bricks = Vec::new();

    for mut brick in bricks {
        //println!("{brick:?}");
        let max_z = dropped_bricks
            .iter()
            .filter(|b| brick.overlaps(b, X) && brick.overlaps(b, Y))
            .map(|b| b.end[Z])
            .max();

        brick = match max_z {
            Some(z) => brick.drop_down_to(z + 1),
            None => brick.drop_down_to(1),
        };
        assert!(!dropped_bricks.contains(&brick));
        dropped_bricks.push(brick);
    }

    dropped_bricks
}

fn drop_bricks_counted(mut bricks: Vec<Brick>, end_z: usize) -> usize {
    bricks.sort_by(|a, b| a.start[Z].cmp(&b.start[Z]));
    let mut dropped_bricks = Vec::new();
    let mut count = 0;

    for mut brick in bricks {
        /* ignore all bricks which are below the disintegrated brick */
        if brick.start[Z] <= end_z {
            dropped_bricks.push(brick);
            continue;
        }

        let max_z = dropped_bricks
            .iter()
            .filter(|b| brick.overlaps(b, X) && brick.overlaps(b, Y))
            .map(|b| b.end[Z])
            .max();

        brick = match max_z {
            Some(z) => {
                if z + 1 == brick.start[Z] {
                    brick // brick doesn't move
                } else {
                    count += 1;
                    brick.drop_down_to(z + 1)
                }
            }
            None => {
                if brick.start[Z] == 1 {
                    brick // brick doesn't move
                } else {
                    count += 1;
                    brick.drop_down_to(1)
                }
            }
        };
        dropped_bricks.push(brick);
    }

    count
}

pub(crate) fn get_solution_1() -> usize {
    let dropped_bricks = drop_bricks(parse_input(INPUT));
    dropped_bricks
        .iter()
        .filter(|b| b.safe_to_disintegrate(&dropped_bricks))
        .count()
}

pub(crate) fn get_solution_2() -> usize {
    let dropped_bricks = drop_bricks(parse_input(INPUT));
    dropped_bricks
        .iter()
        .filter(|b| !b.safe_to_disintegrate(&dropped_bricks))
        .map(|b| b.disintegrate(&dropped_bricks))
        .sum()
}

#[test]
fn test_get_solution_1() {
    println!("{}", get_solution_1());
}

#[test]
fn test_get_solution_2() {
    println!("{}", get_solution_2());
}

#[test]
fn test_parse_input() {
    for b in parse_input(TEST) {
        println!("{b:?}");
    }
}

#[test]
fn test_rests_on_other() {
    let names = ['A', 'B', 'C', 'D', 'E', 'F', 'G'];
    let bricks = parse_input(TEST);

    // named_bricks
    let nb: HashMap<char, Brick> = names.into_iter().zip(bricks).collect();
    assert!(nb[&'A'].supports(&nb[&'B']));
    assert!(!nb[&'B'].supports(&nb[&'C']));
    assert!(nb[&'C'].supports(&nb[&'D']));
    assert!(!nb[&'D'].supports(&nb[&'E']));
    assert!(nb[&'E'].supports(&nb[&'F']));

    let g = &nb[&'G'];

    for n in ['A', 'B', 'C', 'D', 'E', 'F'] {
        assert!(!nb[&n].supports(g));
    }
}

#[test]
fn test_drop_bricks() {
    let (A, B, C, D, E, F, G) = (0, 1, 2, 3, 4, 5, 6);

    let bricks = parse_input(TEST);

    assert_eq!(bricks[G].start[Z], 8);
    let dropped_bricks = drop_bricks(bricks);
    assert_eq!(dropped_bricks[A].start[Z], 1);
    assert_eq!(dropped_bricks[B].start[Z], 2);
    assert_eq!(dropped_bricks[C].start[Z], 2);
    assert_eq!(dropped_bricks[D].start[Z], 3);
    assert_eq!(dropped_bricks[E].start[Z], 3);
    assert_eq!(dropped_bricks[F].start[Z], 4);
    assert_eq!(dropped_bricks[G].start[Z], 5);
}

#[test]
fn test_safe_to_disintegrate() {
    let bricks = parse_input(TEST);

    let dropped_bricks = drop_bricks(bricks);

    let num_to_disintegrate = dropped_bricks
        .iter()
        .filter(|b| b.safe_to_disintegrate(&dropped_bricks))
        .count();

    assert_eq!(num_to_disintegrate, 5);
}

#[test]
fn test_overlaps() {
    /* Case 1: A and B are the same */
    let A = (0, 1);
    let B = (0, 1);
    assert!(Brick::overlaps_helper(A, B));

    /* Case 2: A and B have the same start, but A is longer */
    let A = (0, 2);
    let B = (0, 1);
    assert!(Brick::overlaps_helper(A, B));

    /* Case 3: A and B have the same start, but B is longer */
    let A = (0, 1);
    let B = (0, 2);
    assert!(Brick::overlaps_helper(A, B));

    /* Case 4: A is fully contained in B */
    let A = (1, 2);
    let B = (0, 3);
    assert!(Brick::overlaps_helper(A, B));

    /* Case 5: B is fully contained in B */
    let A = (0, 3);
    let B = (1, 2);
    assert!(Brick::overlaps_helper(A, B));

    /* Case 6: A starts later than B and ends later */
    let A = (2, 3);
    let B = (1, 2);
    assert!(Brick::overlaps_helper(A, B));

    /* Case 7: A starts earlier than B and ends earlier */
    let A = (1, 2);
    let B = (2, 3);
    assert!(Brick::overlaps_helper(A, B));

    /* Case 8: They don't overlap */
    let A = (4, 5);
    let B = (2, 3);
    assert!(!Brick::overlaps_helper(A, B));
}

#[test]
fn test_disintegrate() {
    let dropped_bricks = drop_bricks(parse_input(TEST));
    let sum = dropped_bricks
        .iter()
        .filter(|b| !b.safe_to_disintegrate(&dropped_bricks))
        .map(|b| b.disintegrate(&dropped_bricks))
        .sum::<usize>();

    assert_eq!(sum, 7);
}
