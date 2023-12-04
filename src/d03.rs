use std::collections::HashMap;

#[allow(dead_code)]
static TEST: &str = include_str!("../data/d03t");

#[allow(dead_code)]
static INP: &str = include_str!("../data/d03");

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    map.iter_mut().for_each(|r| {
        r.insert(0, '.');
        r.push('.');
    });
    map.insert(0, vec!['.'; map[0].len()]);
    map.push(vec!['.'; map[0].len()]);
    map
}

fn add_gear_num(row: usize, col: usize, num: u32, gears: &mut HashMap<(usize, usize), Vec<u32>>) {
    match gears.get_mut(&(row, col)) {
        None => {
            gears.insert((row, col), vec![num]);
        }
        Some(nums) => {
            nums.push(num);
        }
    }
}

fn get_num_width(row: usize, col: usize, map: &[Vec<char>]) -> usize {
    let mut width = 0;
    let mut cc = col;
    while cc < map[0].len() && map[row][cc].is_ascii_digit() {
        width += 1;
        cc += 1;
    }
    width
}

fn get_adjacent(row: usize, col: usize, width: usize) -> Vec<(usize, usize)> {
    let mut adjacent = Vec::new();
    adjacent.append(&mut (col - 1..col + width + 1).map(|c| (row - 1, c)).collect());

    adjacent.push((row, col - 1));
    adjacent.push((row, col + width));

    adjacent.append(&mut (col - 1..col + width + 1).map(|c| (row + 1, c)).collect());

    adjacent
}

fn has_adjacent(row: usize, col: usize, width: usize, map: &[Vec<char>]) -> bool {
    get_adjacent(row, col, width)
        .into_iter()
        .any(|(r, c)| map[r][c] != '.')
}

fn has_gear(adjacent: &[(usize, usize)], map: &[Vec<char>]) -> Option<(usize, usize)> {
    adjacent.iter().find(|(r, c)| map[*r][*c] == '*').copied()
}

fn parse_num(row: &[char], start: usize, width: usize) -> u32 {
    row[start..start + width]
        .iter()
        .filter_map(|c| c.to_digit(10))
        .reduce(|mut acc, n| {
            acc *= 10;
            acc + n
        })
        .unwrap()
}

pub(crate) fn get_solution_1() -> u32 {
    let map = parse_input(INP);
    let mut non_adj_nums = Vec::new();

    for (i, row) in map.iter().enumerate() {
        let mut j = 0;

        while j < row.len() {
            if !row[j].is_ascii_digit() {
                j += 1;
                continue;
            }

            let width = get_num_width(i, j, &map);

            if has_adjacent(i, j, width, &map) {
                non_adj_nums.push(parse_num(row, j, width));
            }

            j += width;
        }
    }
    non_adj_nums.into_iter().sum()
}

pub(crate) fn get_solution_2() -> u32 {
    let map = parse_input(INP);
    let mut gears = HashMap::new();
    for (i, row) in map.iter().enumerate() {
        let mut j = 0;

        while j < row.len() {
            if !row[j].is_ascii_digit() {
                j += 1;
                continue;
            }
            let width = get_num_width(i, j, &map);
            let adjacent = get_adjacent(i, j, width);

            if let Some((r, c)) = has_gear(&adjacent, &map) {
                add_gear_num(r, c, parse_num(row, j, width), &mut gears);
            }
            j += width;
        }
    }

    gears
        .into_iter()
        .filter(|(_, nums)| nums.len() == 2)
        .map(|(_, nums)| nums.into_iter().product::<u32>())
        .sum()
}
