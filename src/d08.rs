use std::collections::HashMap;

#[allow(dead_code)]
static TEST: &str = include_str!("../data/d08t");
#[allow(dead_code)]
static TEST2: &str = include_str!("../data/d08t2");
static INPUT: &str = include_str!("../data/d08");

type Network = HashMap<&'static str, (&'static str, &'static str)>;

#[derive(Debug)]
enum Turn {
    Left,
    Right,
}

impl From<char> for Turn {
    fn from(c: char) -> Self {
        match c {
            'L' => Self::Left,
            _ => Self::Right, // 'R'
        }
    }
}

fn parse_input(input: &'static str) -> (Vec<Turn>, Network) {
    let mut line_iter = input.lines();
    // parse turns
    let turns = line_iter
        .next()
        .unwrap()
        .chars()
        .map(|c| c.into())
        .collect();
    // parse the network, skipping the first line
    let network = line_iter
        .skip(1)
        .map(|s| (&s[..3], (&s[7..10], &s[12..15])))
        .collect();
    (turns, network)
}

pub fn get_solution_1() -> usize {
    let (turns, network) = parse_input(INPUT);
    traverse(turns, network)
}

pub fn get_solution_2() -> usize {
    let (turns, network) = parse_input(INPUT);
    traverse_ghost(turns, network)
}

fn traverse(turns: Vec<Turn>, network: Network) -> usize {
    let mut current = "AAA";
    let goal = "ZZZ";
    assert!(!turns.is_empty());
    for (step, turn) in turns.iter().cycle().enumerate() {
        current = match turn {
            Turn::Left => network.get(current).unwrap().0,
            Turn::Right => network.get(current).unwrap().1,
        };
        if current == goal {
            // since we start at 0, add 1 in the end
            return step + 1;
        }
    }
    unreachable!();
}

fn traverse_ghost(turns: Vec<Turn>, network: Network) -> usize {
    // get all nodes that end in 'A'
    let mut currents: Vec<&str> = network
        .keys()
        .filter(|k| k.ends_with('A'))
        .copied()
        .collect();

    let mut steps = Vec::new();

    for (step, turn) in turns.iter().cycle().enumerate() {
        currents = currents
            .into_iter()
            .map(|n| match turn {
                Turn::Left => network.get(n).unwrap().0,
                Turn::Right => network.get(n).unwrap().1,
            })
            .filter(|n| {
                if n.ends_with('Z') {
                    steps.push(step + 1);
                    false
                } else {
                    true
                }
            })
            .collect();
        if currents.is_empty() {
            break;
        }
    }

    // find the least common multiple (lcm) of all the steps taken to reach each goal
    lcm(&steps)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a < b {
        std::mem::swap(&mut a, &mut b);
    }

    match b == 0 {
        true => a,
        false => gcd(b, a % b),
    }
}

fn lcm(numbers: &[usize]) -> usize {
    numbers
        .iter()
        .copied()
        .reduce(|acc, n| acc * n / gcd(acc, n))
        .unwrap_or(0)
}
