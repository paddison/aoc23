use std::num::ParseIntError;

#[allow(dead_code)]
static TEST: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
static INPUT: &str = include_str!("../data/d15");

const CAP: usize = 256;
const EMPTY: Vec<&str> = Vec::new();

struct AocHashMap<T: Lens, const C: usize = CAP> {
    buckets: [Vec<T>; C],
}

impl<T: Lens, const C: usize> AocHashMap<T, C> 
where for <'a> <T as Lens>::Item<'a>: PartialEq
{
    fn remove(&mut self, lens: T) -> Option<T> {
        let bucket_index = lens.hash();
        match self.buckets[bucket_index]
            .iter()
            .position(|item| item.label() == lens.label())
        {
            Some(pos) => Some(self.buckets[bucket_index].remove(pos)),
            _ => None,
        }
    }

    fn insert(&mut self, lens: T) -> Option<T> {
        let bucket_index = lens.hash();
        match self.buckets[bucket_index]
            .iter()
            .position(|item| item.label() == lens.label())
        {
            Some(pos) => Some(std::mem::replace(
                &mut self.buckets[bucket_index][pos],
                lens,
            )),
            _ => {
                self.buckets[bucket_index].push(lens);
                None
            }
        }
    }

    fn focusing_power(&self) -> usize {
        self.buckets
            .iter()
            .enumerate()
            .flat_map(|(i, bucket)| {
                bucket
                    .iter()
                    .enumerate()
                    .filter_map(move |(j, lens)| {
                        lens.focusing_power().map(|fp| fp * (i + 1) * (j + 1)).ok()
                    })
            })
            .sum::<usize>()
    }
}

trait Lens {
    type Item<'a> where Self: 'a;

    fn label(&self) -> Self::Item<'_>;
    fn op(&self) -> Self::Item<'_>;
    fn comp(&self, other: &Self) -> bool;
    fn hash(&self) -> usize;
    fn hash_p1(&self) -> usize;
    fn focusing_power(&self) -> Result<usize, ParseIntError>;
}

impl Lens for &str {
    type Item<'a> = &'a str where Self: 'a;

    fn label(&self) -> Self::Item<'_> {
        &self[..self.find(&['-', '=']).unwrap()]
    }

    fn op(&self) -> Self::Item<'_> {
        let idx = self.find(&['-', '=']).unwrap();
        &(*self)[idx..idx + 1]
    }

    fn comp(&self, other: &Self) -> bool {
        self.label() == other.label()
    }

    fn hash(&self) -> usize {
        self.label().hash_p1()
    }
    
    fn hash_p1(&self) -> usize {
        self.bytes()
            .fold(0, |acc, b| ((acc + b as usize) * 17) % 256)
    }

    fn focusing_power(&self) -> Result<usize, ParseIntError> {
        let idx = self.find(&['-', '=']).unwrap();
        str::parse::<usize>(&self[idx + 1..])
    }
}

fn parse_input(input: &str) -> Vec<&str> {
    input.trim_end().split(',').collect()
}

pub fn get_solution_1() -> usize {
    parse_input(INPUT)
        .into_iter()
        .map(|s| s.hash())
        .sum::<usize>()
}

pub fn get_solution_2() -> usize {
    let mut map = AocHashMap::<&str, CAP> {
        buckets: [EMPTY; CAP]
    };
    for lens in parse_input(INPUT) {
        match lens.op() {
            "=" => map.insert(lens),
            "-" => map.remove(lens),
            _ => panic!("found invalid operation"),
        };
    }
    map.focusing_power()
}

#[test]
fn test_hash() {
    assert_eq!(
        1320,
        parse_input(TEST)
            .into_iter()
            .map(|s| s.hash_p1())
            .sum::<usize>()
    );
}
