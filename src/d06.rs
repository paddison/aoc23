#[allow(dead_code)]
static TEST: [(usize, usize); 3] = [(7, 9), (15, 40), (30, 200)];

#[allow(dead_code)]
static INPUT: [(usize, usize); 4] = [(41, 249), (77, 1362), (70, 1127), (96, 1011)];

#[allow(dead_code)]
static INPUT2: (usize, usize) = (41777096, 249136211271011);

fn determine_ways_to_beat((time, record): (usize, usize)) -> Vec<usize> {
    (0..=time)
        .into_iter()
        .filter(|pushed| (time - pushed) * pushed > record)
        .collect()
}

pub(crate) fn get_solution_1() -> usize {
    INPUT
        .into_iter()
        .map(|race| determine_ways_to_beat(race).len())
        .product()
}

pub(crate) fn get_solution_2() -> usize {
    determine_ways_to_beat(INPUT2).len()
}
