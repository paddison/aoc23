use core::ops::Range;
use std::collections::HashMap;

#[allow(dead_code)]
static TEST: &str = include_str!("../data/d05t");

#[allow(dead_code)]
static INPUT: &str = include_str!("../data/d05");

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Type {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl Type {
    fn to_next(self) -> Self {
        type T = Type;
        match self {
            T::Seed => T::Soil,
            T::Soil => T::Fertilizer,
            T::Fertilizer => T::Water,
            T::Water => T::Light,
            T::Light => T::Temperature,
            T::Temperature => T::Humidity,
            T::Humidity => T::Location,
            T::Location => panic!("Cannot convert Location to something"),
        }
    }
}

impl From<&str> for Type {
    fn from(input: &str) -> Self {
        type T = Type;
        match input {
            "seed" => T::Seed,
            "soil" => T::Soil,
            "fertilizer" => T::Fertilizer,
            "water" => T::Water,
            "light" => T::Light,
            "temperature" => T::Temperature,
            "humidity" => T::Humidity,
            "location" => T::Location,
            _ => panic!("got invalid input str"),
        }
    }
}

#[derive(Debug)]
struct Map {
    dest: usize,
    source: usize,
    range: usize,
}

impl Map {
    fn in_range(&self, item: usize) -> bool {
        self.source <= item && self.source + self.range > item
    }

    fn try_convert(&self, item: usize) -> Option<isize> {
        if self.in_range(item) {
            Some(item as isize + (self.dest as isize - self.source as isize))
        } else {
            None
        }
    }

    // returns start and end range of mapping
    fn determine_split(&self, start: usize, end: usize) -> (usize, usize) {
        (
            start.max(self.source),
            end.min(self.source + self.range),
        )
    }

    // returns the newly mapped seed ranges
    // stored in the form of (start, end] (inclusive - noninclusive]
    fn do_split(&self, seeds: Vec<(usize, usize)>, mapped: &mut Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut todo = Vec::new();
        
        for (start, end) in seeds {
            // mstart and mend are guaranteed to be in the mapping range
            let (mstart, mend) = self.determine_split(start, end);

            // contained -> map
            // need to determine which seed to start on
            if mstart < mend {
                mapped.push((mstart.max(start) + self.dest - self.source, mend.min(end) + self.dest - self.source));
            }
            // overlapping to the left -> no mapping
            if start < mstart {
                todo.push((start, mstart.min(end)));
            }
            // overlapping to the right -> no mapping
            if end > mend {
                todo.push((mend.max(start), end));
            }
        }

        todo
    }
}

fn map_to_location_range(mut seeds: Vec<(usize, usize)>, maps: &HashMap<Type, Vec<Map>>) -> Vec<(usize, usize)> {
    let mut t = Type::Seed;

    while t != Type::Location {
        let mut mapped_seeds = Vec::new();
        let map = maps.get(&t).unwrap();
        for m in map {
            seeds = m.do_split(seeds, &mut mapped_seeds);
        }
        seeds.append(&mut mapped_seeds);
        t = t.to_next();
    }
    seeds
}


impl From<&[usize]> for Map {
    fn from(input: &[usize]) -> Self {
        match input {
            [dest, source, range, ..] => Self {
                dest: *dest,
                source: *source,
                range: *range,
            },
            _ => panic!("input vec doesn't have enough members"),
        }
    }
}

fn parse_input(input: &str) -> (Vec<usize>, HashMap<Type, Vec<Map>>) {
    let mut maps = HashMap::new();
    let mut lines = input.lines().collect::<Vec<_>>();
    let seed_lines = lines.remove(0);
    let seeds = seed_lines[seed_lines.find(':').unwrap() + 1..]
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<_>>();
    lines.remove(0); // empty line

    while !lines.is_empty() {
        let map: Type = lines.remove(0).split('-').next().unwrap().into();
        maps.insert(map, Vec::new());

        while !lines.is_empty() {
            let line = lines.remove(0);
            if line.is_empty() {
                break;
            } else {
                let range = line
                    .split_whitespace()
                    .filter_map(|s| s.parse::<usize>().ok())
                    .collect::<Vec<_>>();
                maps.get_mut(&map).unwrap().push(range.as_slice().into());
            }
        }
    }

    (seeds, maps)
}


// assumes we always start with Type::Seed
fn map_to_location(mut seed: usize, maps: &HashMap<Type, Vec<Map>>) -> usize {
    let mut t = Type::Seed;

    while t != Type::Location {
        let mut next = seed;
        let map = maps.get(&t).unwrap();
        for entry in map {
            if let Some(n) = entry.try_convert(seed) {
                next = n as usize;
                break;
            }
        }
        t = t.to_next();
        seed = next;
    }
    seed
}

pub(crate) fn get_solution_1() -> usize {
    let (seeds, maps) = parse_input(INPUT);

    seeds
        .into_iter()
        .map(|s| map_to_location(s, &maps))
        .min()
        .unwrap_or(usize::MAX)
}

pub(crate) fn get_solution_2() -> usize {
    let (seeds, maps) = parse_input(INPUT);
    let mut seeds: Vec<(usize, usize)> = seeds
        .windows(2)
        .step_by(2)
        .map(|s| (s[0], s[0] + s[1]))
        .collect();

    seeds = map_to_location_range(seeds, &maps); 
    seeds.into_iter().min_by(|(a, _), (b, _)| a.cmp(&b)).unwrap().0
}

#[test]
fn test_solution_1() {
    println!("{}", get_solution_1());
}

#[test]
fn test_solution_2() {
    println!("{}", get_solution_2());
}

#[test]
fn test_overlapping_right() {
    let map = Map { dest: 20, source: 34, range: 5 };
    let seed = vec![(39, 42)];
    let mut mapped_seeds = Vec::new();
    let actual = map.do_split(seed, &mut mapped_seeds);
    println!("{:?}", actual);
    assert_eq!(actual, vec![(39, 42)]);
    assert!(mapped_seeds.is_empty());
}

#[test] 
fn test_overlapping_left() {
    let map = Map { dest: 20, source: 34, range: 5 };
    let seed = vec![(30, 33)];
    let mut mapped_seeds = Vec::new();
    let actual = map.do_split(seed, &mut mapped_seeds);
    println!("{:?}", actual);
    assert_eq!(actual, vec![(30, 33)]);
}

#[test]
fn test_overlapping_center_right() {
    let map = Map { dest: 20, source: 34, range: 5 };
    let seed = vec![(38, 42)];
    let mut mapped_seeds = Vec::new();
    let actual = map.do_split(seed, &mut mapped_seeds);
    println!("{:?}", actual);
    assert_eq!(actual, vec![(39, 42)]);
    assert_eq!(mapped_seeds, vec![(24, 25)]);
}

#[test]
fn test_verlapping_center_left() {
    let map = Map { dest: 20, source: 34, range: 5 };
    let seed = vec![(30, 34)];
    let mut mapped_seeds = Vec::new();
    let actual = map.do_split(seed, &mut mapped_seeds);
    println!("{:?}", actual);
    assert_eq!(actual, vec![(30, 33)]);
    assert_eq!(mapped_seeds, vec![(20, 21)]);

}

#[test]
fn test_overlapping_right_center_left() {
    let map = Map { dest: 20, source: 34, range: 5 };
    let seed = vec![(30, 42)];
    let mut mapped_seeds = Vec::new();
    let actual = map.do_split(seed, &mut mapped_seeds);
    println!("{:?}", actual);
    assert_eq!(actual, vec![(30, 34), (39, 42)]);
    assert_eq!(mapped_seeds, vec![(20, 25)]);
}

#[test]
fn test_overlapping_center() {
    let map = Map { dest: 20, source: 34, range: 5 };
    let seed = vec![(35, 39)];
    let mut mapped_seeds = Vec::new();
    let actual = map.do_split(seed, &mut mapped_seeds);
    println!("{:?}", actual);
    assert!(actual.is_empty());
    assert_eq!(mapped_seeds, vec![(21, 25)]);
}
