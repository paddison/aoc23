use std::collections::HashMap;

#[allow(dead_code)]
static TEST: &str = include_str!("../data/d12t");
static INPUT: &str = include_str!("../data/d12");

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Spring {
    O,
    D,
    U,
}

impl From<char> for Spring {
    fn from(input: char) -> Self {
        match input {
            '.' => Self::O,
            '#' => Self::D,
            _ => Self::U,
        }
    }
}

#[derive(Debug, Clone)]
struct Entry {
    springs: Vec<Spring>,
    damaged: Vec<usize>,
}

impl Entry {
    fn unfold(self) -> Self {
        let Self { springs, damaged } = self;

        Self {
            springs: vec![springs.clone(); 5].join(&Spring::U),
            damaged: vec![damaged.clone(); 5].concat(),
        }
    }

    fn count_arrangements(
        &self,
        i: usize,
        j: usize,
        seen: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        // check if we already looked at this combination
        if let Some(matched) = seen.get(&(i, j)) {
            return *matched;
        }

        // skip all springs which are operational
        if let Some(Spring::O) = self.springs.get(i) {
            return self.count_arrangements(i + 1, j, seen);
        }

        // check if this was a valid arangement
        if j == self.damaged.len()
            && (i >= self.springs.len() || self.springs[i..].iter().all(|s| *s != Spring::D))
        {
            return 1;
        }

        // get the current group
        let n = match self.damaged.get(j) {
            Some(n) => *n,
            None => return 0,
        };

        // check if there is enough remaining space in this set of springs
        let remaining = self.damaged[j..].iter().sum::<usize>() + self.damaged[j..].len() - 1;
        if i + remaining > self.springs.len() {
            return 0;
        }

        let mut matched = 0;

        // if its a valid arrangement, go to the next entry in the list of damaged springs
        if self.is_valid_arrangement(i, n) {
            matched += self.count_arrangements(i + n + 1, j + 1, seen);
        }

        // if the current spring would be damaged, this would be the last possible position
        // for the entry of the list, meaning we cannot look for any more arrangements with
        // this damaged entry.
        if self.springs[i] != Spring::D {
            matched += self.count_arrangements(i + 1, j, seen);
        }

        // store the result of this call in the lookup table
        seen.insert((i, j), matched);

        matched
    }

    fn is_valid_arrangement(&self, i: usize, n: usize) -> bool {
        match self.springs.get(i + n) {
            Some(Spring::D) => false,
            _ => (i..i + n).all(|m| self.springs[m] != Spring::O),
        }
    }
}

impl From<&str> for Entry {
    fn from(input: &str) -> Self {
        let parts = input.split_whitespace().collect::<Vec<_>>();
        Self {
            springs: parts[0].chars().map(|c| c.into()).collect(),
            damaged: parts[1]
                .split(',')
                .filter_map(|n| str::parse::<usize>(n).ok())
                .collect(),
        }
    }
}

fn parse_input(input: &str) -> Vec<Entry> {
    input.lines().map(|line| line.into()).collect()
}

pub fn get_solution_1() -> usize {
    parse_input(INPUT)
        .into_iter()
        .map(|s| s.count_arrangements(0, 0, &mut HashMap::new()))
        .sum()
}

pub fn get_solution_2() -> usize {
    parse_input(INPUT)
        .into_iter()
        .map(|s| s.unfold().count_arrangements(0, 0, &mut HashMap::new()))
        .sum()
}

#[test]
fn test_find_arrangements_rec1() {
    let spring: Entry = "???.### 1,1,3".into();
    assert_eq!(
        1,
        spring
            .unfold()
            .count_arrangements(0, 0, &mut HashMap::new())
    );
}

#[test]
fn test_find_arrangements_rec2() {
    let spring: Entry = ".??..??...?##. 1,1,3".into();
    assert_eq!(
        16384,
        spring
            .unfold()
            .count_arrangements(0, 0, &mut HashMap::new())
    );
}

#[test]
fn test_find_arrangements_rec3() {
    let spring: Entry = "?#?#?#?#?#?#?#? 1,3,1,6".into();
    assert_eq!(
        1,
        spring
            .unfold()
            .count_arrangements(0, 0, &mut HashMap::new())
    );
}

#[test]
fn test_find_arrangements_rec4() {
    let spring: Entry = "????.#...#... 4,1,1".into();
    assert_eq!(
        16,
        spring
            .unfold()
            .count_arrangements(0, 0, &mut HashMap::new())
    );
}

#[test]
fn test_find_arrangements_rec5() {
    let spring: Entry = "????.######..#####. 1,6,5".into();
    assert_eq!(
        2500,
        spring
            .unfold()
            .count_arrangements(0, 0, &mut HashMap::new())
    );
}

#[test]
fn test_find_arrangements_rec6() {
    let spring: Entry = "?###???????? 3,2,1".into();
    assert_eq!(
        506250,
        spring
            .unfold()
            .count_arrangements(0, 0, &mut HashMap::new())
    );
}
