use std::collections::HashMap;

use crate::util::Rotate;

#[allow(dead_code)]
static TEST: &str = include_str!("../data/d14t");
static INPUT: &str = include_str!("../data/d14");

const N_CYCLES: usize = 1000000000;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn tilt_up(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid = vec![grid.remove(0)];

    for line in grid.into_iter() {
        // push an empty grid with no rocks
        new_grid.push(
            line.iter()
                .map(|c| if *c == 'O' { '.' } else { *c })
                .collect(),
        );
        for (j, c) in line.iter().copied().enumerate() {
            if c != 'O' {
                continue;
            }
            for k in (0..new_grid.len() - 1).rev() {
                if new_grid[k][j] != '.' {
                    new_grid[k + 1][j] = 'O';
                    break;
                } else if k == 0 {
                    new_grid[k][j] = 'O';
                }
            }
        }
    }

    new_grid
}

fn do_cycle(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // always tilt up and just rotate the grid
    for _ in 0..4 {
        grid = tilt_up(grid);
        grid = grid.rotate();
    }

    grid
}

fn get_total_load(grid: &[Vec<char>]) -> usize {
    grid.iter()
        .rev()
        .enumerate()
        .map(|(i, l)| l.iter().filter(|c| **c == 'O').count() * (i + 1))
        .sum()
}

fn find_repetition(mut grid: Vec<Vec<char>>) -> usize {
    let mut seen = HashMap::new();

    seen.insert(grid.clone(), 0);

    for i in 1.. {
        grid = do_cycle(grid);
        // try to find a cycle in the patterns
        if let Some(start) = seen.get(&grid) {
            // cycle was found, see how many more grid cycles are needed to end up
            // at the same value we would have at N_CYCLES
            let j = (N_CYCLES - i) % (i - start);
            for _ in 0..j {
                grid = do_cycle(grid);
            }
            return get_total_load(&grid);
        }
        seen.insert(grid.clone(), i);
    }

    unreachable!();
}

pub fn get_solution_1() -> usize {
    get_total_load(&tilt_up(parse_input(INPUT)))
}

pub fn get_solution_2() -> usize {
    find_repetition(parse_input(INPUT))
}

#[test]
fn test_tilt_up() {
    let grid = parse_input(TEST);
    let new_grid = tilt_up(grid);
    for line in new_grid {
        println!("{:?}", line)
    }
}

#[test]
fn test_get_total_load() {
    let grid = parse_input(TEST);
    let new_grid = tilt_up(grid);
    assert_eq!(136, get_total_load(&new_grid));
}

#[test]
fn test_do_cycle() {
    let grid = parse_input(TEST);
    let new_grid = do_cycle(grid);

    for line in new_grid {
        println!("{}", line.into_iter().collect::<String>());
    }
}

#[test]
fn test_find_repetition() {
    let grid = parse_input(TEST);
    assert_eq!(64, find_repetition(grid));
}
