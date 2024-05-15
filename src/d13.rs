use crate::util::Rotate;

#[allow(dead_code)]
static TEST: &str = include_str!("../data/d13t");
static INPUT: &str = include_str!("../data/d13");

type Pattern = Vec<Vec<char>>;

#[derive(Debug, PartialEq)]
enum Reflection {
    Row(usize),
    Col(usize),
}

fn parse_input(input: &str) -> Vec<Pattern> {
    let mut patterns = Vec::new();
    let mut pattern = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            patterns.push(pattern);
            pattern = Vec::new();
        } else {
            pattern.push(line.chars().collect());
        }
    }

    patterns.push(pattern);

    patterns
}

// i points to the start of the right pattern
fn does_reflect_horizontally(pattern: &Pattern, i: usize) -> usize {
    pattern[0..i]
        .iter()
        .rev()
        .zip(&pattern[i..])
        .flat_map(|(l, r)| l.iter().zip(&r[..]))
        .filter(|(l, r)| l != r)
        .count()
}

fn does_reflect_vertically(pattern: &Pattern, i: usize) -> usize {
    does_reflect_horizontally(&pattern.rotate(), i)
}

fn find_reflection(pattern: &Pattern, diff: usize) -> Reflection {
    let width = pattern.first().map(|row| row.len()).unwrap_or(0);
    let height = pattern.len();

    for i in 1..height {
        if does_reflect_horizontally(pattern, i) == diff {
            return Reflection::Row(i);
        }
    }

    for i in 1..width {
        if does_reflect_vertically(pattern, i) == diff {
            return Reflection::Col(i);
        }
    }
    panic!("found no reflection");
}

fn summarize(patterns: Vec<Pattern>, diff: usize) -> usize {
    patterns
        .into_iter()
        .map(|pattern| match find_reflection(&pattern, diff) {
            Reflection::Row(i) => i * 100,
            Reflection::Col(i) => i,
        })
        .sum()
}

pub fn get_solution_1() -> usize {
    summarize(parse_input(INPUT), 0)
}

pub fn get_solution_2() -> usize {
    summarize(parse_input(INPUT), 1)
}

#[test]
fn test_find_reflection() {
    let input = parse_input(TEST);

    assert_eq!(Reflection::Col(5), find_reflection(&input[0], 0));
    assert_eq!(Reflection::Row(4), find_reflection(&input[1], 0));
}

#[test]
fn test_summarize() {
    assert_eq!(405, summarize(parse_input(TEST), 0));
}

#[test]
fn test_flipped() {
    assert_eq!(400, summarize(parse_input(TEST), 1));
}
