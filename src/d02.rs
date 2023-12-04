use std::ops::{Index, IndexMut};

#[allow(dead_code)]
static TEST: &str = include_str!("../data/d02t");
static INP: &str = include_str!("../data/d02");

type Game = Vec<Draw>;
type Draw = [usize; 3];

#[derive(Debug)]
enum Color {
    R,
    G,
    B,
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        match s {
            "red" => Self::R,
            "green" => Self::G,
            "blue" => Self::B,
            _ => unreachable!(),
        }
    }
}

impl Index<Color> for [usize; 3] {
    type Output = usize;

    fn index(&self, color: Color) -> &Self::Output {
        match color {
            Color::R => &self[0],
            Color::G => &self[1],
            Color::B => &self[2],
        }
    }
}

impl IndexMut<Color> for [usize; 3] {
    fn index_mut(&mut self, color: Color) -> &mut Self::Output {
        match color {
            Color::R => &mut self[0],
            Color::G => &mut self[1],
            Color::B => &mut self[2],
        }
    }
}

fn parse_input(inp: &str) -> Vec<Game> {
    let mut games = Vec::new();

    for line in inp.lines() {
        let i = line.find(':').unwrap() + 2;
        let mut game = Vec::new();

        for draw in line[i..].split(';') {
            let mut bag = [0, 0, 0];
            for balls in draw.split(',') {
                match balls.split_whitespace().collect::<Vec<_>>().as_slice() {
                    &[n, color] => bag[Color::from(color)] = n.parse().unwrap(),
                    _ => panic!("draw has wrong form"),
                }
            }
            game.push(bag);
        }
        games.push(game);
    }
    games
}

fn too_many(game: &Game, max: &Draw) -> bool {
    type C = Color;

    game.iter()
        .any(|draw| draw[C::R] > max[C::R] || draw[C::G] > max[C::G] || draw[C::B] > max[C::B])
}

pub(crate) fn get_solution_1() -> usize {
    let max = [12, 13, 14];
    parse_input(INP)
        .into_iter()
        .enumerate()
        .filter(|(_, g)| !too_many(g, &max))
        .map(|(i, _)| i + 1)
        .sum()
}

fn determine_fewest(game: &Game) -> Draw {
    type C = Color;

    let mut max = [0, 0, 0];

    for draw in game {
        if draw[C::R] > max[C::R] {
            max[C::R] = draw[C::R]
        };
        if draw[C::G] > max[C::G] {
            max[C::G] = draw[C::G]
        };
        if draw[C::B] > max[C::B] {
            max[C::B] = draw[C::B]
        };
    }

    max
}

pub(crate) fn get_solution_2() -> usize {
    parse_input(INP).into_iter().fold(0, |s, g| {
        s + determine_fewest(&g).into_iter().product::<usize>()
    })
}
