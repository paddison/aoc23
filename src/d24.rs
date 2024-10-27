use std::fmt::Display;

static TEST: &str = include_str!("../data/d24t");
static INPUT: &str = include_str!("../data/d24");

#[derive(Debug, Clone, Copy)]
struct HailStone2D {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
}

impl From<HailStone3D> for HailStone2D {
    fn from(value: HailStone3D) -> Self {
        Self {
            x: value.x,
            y: value.y,
            dx: value.dx,
            dy: value.dy,
        }
    }
}

impl std::ops::Mul<f64> for HailStone2D {
    type Output = (f64, f64);

    fn mul(self, rhs: f64) -> Self::Output {
        (self.x + rhs * self.dx, self.y + rhs * self.dy)
    }
}

impl Display for HailStone2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {} @ {}, {}", self.x, self.y, self.dx, self.dy,)
    }
}

impl HailStone2D {
    fn normalize_velocity(&mut self) {
        let dx = self.dx.abs();
        self.dx /= dx;
        self.dy /= dx;
    }

    fn intersect(&self, other: &Self) -> Option<(f64, f64)> {
        if self.is_parallel_to(other) {
            return None;
        }

        let HailStone2D {
            x: x1,
            y: y1,
            dx: dx1,
            dy: dy1,
        } = self;

        let HailStone2D {
            x: x2,
            y: y2,
            dx: dx2,
            dy: dy2,
        } = other;

        let t1 = (dx2 * (y1 - y2) + dy2 * (x2 - x1)) / ((dx1 * dy2) - (dy1 * dx2));
        let t2 = (x1 + t1 * dx1 - x2) / dx2;

        Some((t1, t2))
    }

    fn is_parallel_to(&self, other: &Self) -> bool {
        let m = (self.dx / other.dx).abs();
        self.dx == (other.dx * m) && self.dy == (other.dy * m)
    }
}

#[derive(Debug, Clone, Copy)]
struct HailStone3D {
    x: f64,
    y: f64,
    z: f64,
    dx: f64,
    dy: f64,
    dz: f64,
}

impl From<&[f64]> for HailStone3D {
    fn from(value: &[f64]) -> Self {
        assert_eq!(value.len(), 6);
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
            dx: value[3],
            dy: value[4],
            dz: value[5],
        }
    }
}

impl Display for HailStone3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {} @ {}, {}, {}",
            self.x, self.y, self.z, self.dx, self.dy, self.dz
        )
    }
}

fn parse_input(inp: &str) -> Vec<HailStone3D> {
    inp.lines()
        .map(|line| {
            line.split([',', '@'])
                .filter_map(|val| val.trim().parse::<f64>().ok())
                .collect::<Vec<_>>()
        })
        .map(|pos| pos.as_slice().into())
        .collect()
}

fn determine_intersection(hss: Vec<HailStone2D>, lower: f64, upper: f64) -> usize {
    let mut count = 0;
    for (i, hs1) in hss.iter().enumerate() {
        for hs2 in &hss[i + 1..] {
            if let Some((t1, t2)) = hs1.intersect(hs2) {
                if t1 < 0. || t2 < 0. {
                    continue;
                }
                let (x1, y1) = *hs1 * t1;
                if (x1 >= lower) && (x1 <= upper) && (y1 >= lower) && (y1 <= upper) {
                    count += 1;
                }
            }
        }
    }

    count
}

pub(crate) fn get_solution_1() -> usize {
    let hss: Vec<HailStone2D> = parse_input(INPUT).into_iter().map(|hs| hs.into()).collect();
    determine_intersection(hss, 200000000000000., 400000000000000.)
}

#[test]
fn test_parse_input() {
    let hss: Vec<HailStone2D> = parse_input(TEST).into_iter().map(|hs| hs.into()).collect();
    for mut hs in hss {
        hs.normalize_velocity();
        println!("{hs}");
    }
}
