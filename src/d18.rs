#[allow(dead_code)]
static TEST: &str = include_str!("../data/d18t");
static INPUT: &str = include_str!("../data/d18");

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl TryFrom<&str> for Dir {
    type Error = &'static str;

    fn try_from(inp: &str) -> Result<Self, Self::Error> {
        match inp {
            "U" | "3" => Ok(Dir::Up),
            "R" | "0" => Ok(Dir::Right),
            "D" | "1" => Ok(Dir::Down),
            "L" | "2" => Ok(Dir::Left),
            _ => Err("got invalid direction"),
        }
    }
}

fn parse_hex(hex: &str) -> (isize, Dir) {
    assert!(&hex[0..1] == "#");

    (
        isize::from_str_radix(&hex[1..6], 16).unwrap(),
        Dir::try_from(&hex[6..7]).unwrap(),
    )
}

struct DigEntry {
    dir: Dir,
    steps: usize,
    color: &'static str,
}

impl TryFrom<&'static str> for DigEntry {
    type Error = &'static str;
    fn try_from(inp: &'static str) -> Result<Self, Self::Error> {
        let mut iter = inp.split_whitespace();
        let dir = iter.next().ok_or("empty line")?.try_into()?;
        let steps = iter
            .next()
            .ok_or("steps missing")?
            .parse::<usize>()
            .map_err(|_| "cannot parse steps")?;
        let color = iter
            .next()
            .ok_or("color missing")
            .map(|c| &c[1..c.len() - 1])?;

        Ok(DigEntry { dir, steps, color })
    }
}

fn parse_input(inp: &'static str) -> Result<Vec<DigEntry>, &'static str> {
    inp.lines().map(|l| l.try_into()).collect()
}

fn picks_theorem(map: &[(isize, Dir)]) -> isize {
    let mut area: isize = 0;
    let mut x = 0;
    let mut y = 0;
    let mut y_1 = y;
    let mut x_1 = x;
    let mut perimeter = 0;

    for (d, dir) in map {
        match dir {
            Dir::Up => y_1 -= d,
            Dir::Down => y_1 += d,
            Dir::Right => {
                x_1 += d;
            }
            Dir::Left => {
                x_1 -= d;
            }
        }
        perimeter += d;

        // shoelace formula
        area += (y + y_1) * (x - x_1);
        y = y_1;
        x = x_1;
    }

    area / 2 + perimeter / 2 + 1
}

pub fn get_solution_1() -> usize {
    let inp = parse_input(INPUT)
        .unwrap()
        .into_iter()
        .map(|DigEntry { steps, dir, .. }| (steps as isize, dir))
        .collect::<Vec<(isize, Dir)>>();

    picks_theorem(&inp) as usize
}

pub fn get_solution_2() -> usize {
    let inp = parse_input(INPUT)
        .unwrap()
        .into_iter()
        .map(|DigEntry { color, .. }| parse_hex(color))
        .collect::<Vec<(isize, Dir)>>();

    picks_theorem(&inp) as usize
}
