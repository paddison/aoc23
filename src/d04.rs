use std::collections::HashSet;

#[allow(dead_code)]
static TEST: &str = include_str!("../data/d04t");
#[allow(dead_code)]
static INPUT: &str = include_str!("../data/d04");

fn parse_input(input: &str) -> Vec<(HashSet<u32>, HashSet<u32>)> {
    let mut cards = Vec::new();

    for line in input.lines() {
        let i = line.find(':').unwrap() + 1;
        let card: Vec<_> = line[i..]
            .split('|')
            .map(|card| {
                card.split_whitespace()
                    .filter_map(|n| n.parse().ok())
                    .collect::<HashSet<_>>()
            })
            .collect();
        cards.push((card[0].clone(), card[1].clone()));
    }

    cards
}

pub(crate) fn get_solution_1() -> usize {
    parse_input(INPUT)
        .into_iter()
        .map(|(actual, win)| actual.intersection(&win).count())
        .filter(|n| n > &0)
        .map(|n| 2_usize.pow(n as u32 - 1))
        .sum()
}

pub(crate) fn get_solution_2() -> usize {
    let cards = parse_input(INPUT);
    let mut copies = vec![1; cards.len()];
    cards.into_iter()
        .map(|(act, win)| act.intersection(&win).count())
        .enumerate()
        .map(|(i, n)| {
            for j in i + 1..i + n + 1 { 
                copies[j] += copies[i]; 
            }
            copies[i]
        }).sum()
}
