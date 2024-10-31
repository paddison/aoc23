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

impl From<HailStoneVec> for HailStone2D {
    fn from(value: HailStoneVec) -> Self {
        Self {
            x: value.p.x1,
            y: value.p.x2,
            dx: value.v.x1,
            dy: value.v.x2,
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
struct Vec3D {
    x1: f64,
    x2: f64,
    x3: f64,
}

impl Vec3D {
    fn new(x1: f64, x2: f64, x3: f64) -> Self {
        Self { x1, x2, x3 }
    }
    fn cross_product(&self, other: &Self) -> Self {
        let x1 = (self.x2 * other.x3) - (self.x3 * other.x2);
        let x2 = (self.x3 * other.x1) - (self.x1 * other.x3);
        let x3 = (self.x1 * other.x2) - (self.x2 * other.x1);

        Self { x1, x2, x3 }
    }

    fn dot_product(&self, other: &Self) -> f64 {
        (self.x1 * other.x1) + (self.x2 * other.x2) + (self.x3 * other.x3)
    }
}

impl std::ops::Sub for Vec3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x1: self.x1 - rhs.x1,
            x2: self.x2 - rhs.x2,
            x3: self.x3 - rhs.x3,
        }
    }
}

impl std::ops::Add for Vec3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x1: self.x1 + rhs.x1,
            x2: self.x2 + rhs.x2,
            x3: self.x3 + rhs.x3,
        }
    }
}

impl std::ops::Mul<f64> for Vec3D {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x1: self.x1 * rhs,
            x2: self.x2 * rhs,
            x3: self.x3 * rhs,
        }
    }
}

impl std::ops::Div<f64> for Vec3D {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x1: self.x1 / rhs,
            x2: self.x2 / rhs,
            x3: self.x3 / rhs,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct HailStoneVec {
    p: Vec3D,
    v: Vec3D,
}

impl From<&[f64]> for HailStoneVec {
    fn from(value: &[f64]) -> Self {
        assert_eq!(value.len(), 6);
        Self {
            p: Vec3D::new(value[0], value[1], value[2]),
            v: Vec3D::new(value[3], value[4], value[5]),
        }
    }
}

fn solve(s0: HailStoneVec, s1: HailStoneVec, s2: HailStoneVec) -> HailStoneVec {
    /* this solution is taken from reddit https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/kxqjg33/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button,
     * i'm not solving 9 linear equations by hand...*/

    /* get position and velocity relative to stone 0 */
    let p1 = s1.p - s0.p;
    let v1 = s1.v - s0.v;
    let p2 = s2.p - s0.p;
    let v2 = s2.v - s0.v;

    /* the times when the collision happens, relative to stone 0 */
    let t1 = (-(p1.cross_product(&p2).dot_product(&v2))) / v1.cross_product(&p2).dot_product(&v2);
    let t2 = (-(p1.cross_product(&p2).dot_product(&v1))) / p1.cross_product(&v2).dot_product(&v1);

    /* caluclate the actual collision points */
    let c1 = s1.p + (s1.v * t1);
    let c2 = s2.p + (s2.v * t2);

    /* calculate the stone from this */
    let v = (c2 - c1) / (t2 - t1);
    let p = c1 - (v * t1);

    HailStoneVec { p, v }
}

fn parse_input(inp: &str) -> Vec<HailStoneVec> {
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

pub(crate) fn get_solution_2() -> usize {
    let hss: Vec<HailStoneVec> = parse_input(INPUT).into_iter().map(|hs| hs.into()).collect();
    let stone = solve(hss[0], hss[1], hss[2]);
    (stone.p.x1 + stone.p.x2 + stone.p.x3) as usize
}
