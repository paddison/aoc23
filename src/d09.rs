#[allow(dead_code)]
static TEST: &str = include_str!("../data/d09t");
static INPUT: &str = include_str!("../data/d09");

pub fn get_solution_1() -> isize {
    parse_input(INPUT).iter_mut().map(|n| derive(n)).sum()
}

pub fn get_solution_2() -> isize {
    parse_input(INPUT)
        .iter_mut()
        .map(|n| {
            n.reverse();
            derive(n)
        })
        .sum()
}

fn parse_input(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|n| str::parse::<isize>(n).ok())
                .collect()
        })
        .collect()
}

fn derive(nums: &[isize]) -> isize {
    if nums.is_empty() || nums.iter().all(|n| *n == 0) {
        return 0;
    }
    let deltas = &nums.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
    derive(deltas) + nums.last().unwrap()
}
