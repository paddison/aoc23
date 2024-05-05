#[allow(dead_code)]
static TEST: &str = include_str!("../data/d11t");
static INPUT: &str = include_str!("../data/d11");

type Position = [usize; 2];

const COL: usize = 0;
const ROW: usize = 1;

#[derive(Debug)]
struct Universe {
    galaxies: Vec<Position>,
    width: usize,
    height: usize,
}

impl Universe {
    fn expand(self, rate: usize) -> ExpandedUniverse {
        let Self { galaxies, width, height } = self;
        let mut expanded = galaxies.clone();

        for (axis, len) in [(COL, width), (ROW, height)] {
            for row in (0..len).filter(|p| galaxies.iter().all(|g| g[axis] != *p)) {
                for (exp_galaxy, old_galaxy) in expanded.iter_mut().zip(&galaxies) {
                    if old_galaxy[axis] > row {
                        exp_galaxy[axis] += rate;
                    }
                }
            }
        }

        ExpandedUniverse { galaxies: expanded }
    }

}

struct ExpandedUniverse {
    galaxies: Vec<Position>,
}

impl ExpandedUniverse {
    fn shortest_dists(self) -> Vec<usize> {
        self.galaxies
            .iter()
            .enumerate()
            .flat_map(|(i, from)| {
                self.galaxies
                    .iter()
                    .skip(i + 1)
                    .map(|to| from[COL].abs_diff(to[COL]) + from[ROW].abs_diff(to[ROW]))
            })
            .collect()
    }
}

fn parse_input(input: &str) -> Universe {
    let galaxies: Vec<_> = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter_map(|(x, c)| if c == '#' { Some([x, y]) } else { None })
                .collect::<Vec<_>>()
        })
        .collect();

    Universe {
        galaxies,
        width: input.find('\n').unwrap(),
        height: input.lines().count(),
    }
}

pub fn get_solution_1() -> usize {
    parse_input(INPUT).expand(1).shortest_dists().iter().sum()
}

pub fn get_solution_2() -> usize {
    parse_input(INPUT).expand(999999).shortest_dists().iter().sum()
}
