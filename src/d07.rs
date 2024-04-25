use std::{cmp::Ordering, fmt::Display};

static TEST: &str = include_str!("../data/d07t");
static INPUT: &str = include_str!("../data/d07");

fn parse_input(input: &str) -> Vec<(Hand, usize)> {
    let mut bets = Vec::new();
    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        bets.push((parts[0].into(), parts[1].parse::<usize>().unwrap()));
    }

    bets
}

fn determine_total_winnings(mut bids: Vec<(Hand, usize)>) -> usize {
    bids.sort_by(|(lhs, _), (rhs, _)| lhs.partial_cmp(rhs).unwrap());

    for (card, _) in &bids {
        //println!("{card}");
    }

    bids.into_iter()
        .enumerate()
        .map(|(i, (_, amount))| amount * (i + 1))
        .sum()
}

pub(crate) fn get_solution_1() -> usize {
    let bids = parse_input(INPUT);
    determine_total_winnings(bids)
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
enum Card {
    TWO = 0,
    THREE = 1,
    FOUR = 2,
    FIVE = 3,
    SIX = 4,
    SEVEN = 5,
    EIGHT = 6,
    NINE = 7,
    T = 8,
    J = 9,
    Q = 10,
    K = 11,
    A = 12,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c: char = char::from(*self);
        write!(f, "{c}")
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::TWO
    }
}

impl From<char> for Card {
    fn from(input: char) -> Self {
        match input {
            '2' => Self::TWO,
            '3' => Self::THREE,
            '4' => Self::FOUR,
            '5' => Self::FIVE,
            '6' => Self::SIX,
            '7' => Self::SEVEN,
            '8' => Self::EIGHT,
            '9' => Self::NINE,
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            _ => unreachable!(),
        }
    }
}

impl From<Card> for char {
    fn from(value: Card) -> Self {
        match value {
            Card::TWO => '2',
            Card::THREE => '3',
            Card::FOUR => '4',
            Card::FIVE => '5',
            Card::SIX => '6',
            Card::SEVEN => '7',
            Card::EIGHT => '8',
            Card::NINE => '9',
            Card::T => 'T',
            Card::J => 'J',
            Card::Q => 'Q',
            Card::K => 'K',
            Card::A => 'A',
        }
    }
}

#[derive(Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            self.cards[0], self.cards[1], self.cards[2], self.cards[3], self.cards[4],
        )
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let lhs = self.score();
        let rhs = other.score();

        if lhs > rhs {
            Some(Ordering::Greater)
        } else if lhs < rhs {
            Some(Ordering::Less)
        } else {
            Some(self.compare(other))
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl From<&str> for Hand {
    fn from(input: &str) -> Self {
        let mut cards = [Card::default(); 5];
        input
            .chars()
            .enumerate()
            .for_each(|(i, c)| cards[i] = c.into());
        Self { cards }
    }
}

impl Hand {
    fn is_five(&self) -> bool {
        return self.is_n(5, 0).is_some();
    }

    fn is_four(&self) -> bool {
        self.is_n(4, 0).is_some()
    }

    fn is_three(&self) -> bool {
        self.is_n(3, 0).is_some()
    }

    fn is_pair(&self) -> bool {
        self.is_n(2, 0).is_some()
    }

    fn is_full_house(&self) -> bool {
        if let Some(three) = self.is_n(3, 0) {
            return (0..5).any(|i| self.is_n(2, i).filter(|two| three != *two).is_some());
        }
        false
    }

    fn is_double_pair(&self) -> bool {
        if let Some(first) = self.is_n(2, 0) {
            return (1..5).any(|i| self.is_n(2, i).filter(|second| first != *second).is_some());
        }
        false
    }

    fn compare(&self, other: &Self) -> Ordering {
        for (lhs, rhs) in self.cards.iter().zip(other.cards) {
            if lhs > &rhs {
                return Ordering::Greater;
            } else if lhs < &rhs {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }

    fn score(&self) -> usize {
        if self.is_five() {
            6
        } else if self.is_four() {
            5
        } else if self.is_full_house() {
            4
        } else if self.is_three() {
            3
        } else if self.is_double_pair() {
            2
        } else if self.is_pair() {
            1
        } else {
            0
        }
    }

    fn is_n(&self, n: usize, start: usize) -> Option<Card> {
        for i in start..(6 - n) {
            let count = self
                .cards
                .iter()
                .skip(i + 1)
                .filter(|c| **c == self.cards[i])
                .count();
            //println!("{count}");
            if count == n - 1 {
                // subtract one since the card is not compared to itself
                return Some(self.cards[i]);
            }
        }
        None
    }
}

#[test]
fn test_is_n_1() {
    let hand: Hand = "A2345".into();
    assert!(hand.is_n(1, 0).is_some());
}

#[test]
fn test_is_n_2() {
    let hand: Hand = "A234A".into();
    assert!(hand.is_n(2, 0).is_some());
}

#[test]
fn test_is_n_3() {
    let hand: Hand = "A2A4A".into();
    assert!(hand.is_n(3, 0).is_some());
}

#[test]
fn test_is_n_4() {
    let hand: Hand = "AA3AA".into();
    assert!(hand.is_n(4, 0).is_some());
}

#[test]
fn test_is_five() {
    let hand: Hand = "AAAAA".into();
    assert!(hand.is_five());
}

#[test]
fn test_is_four() {
    let hand: Hand = "AAAA4".into();
    assert!(hand.is_four());
}

#[test]
fn test_is_full_house() {
    let hand: Hand = "AATTT".into();
    assert!(hand.is_full_house());
}

#[test]
fn test_is_three() {
    let hand: Hand = "AA2AT".into();
    assert!(hand.is_three());
}

#[test]
fn test_is_double_pair() {
    let hand: Hand = "AAT2T".into();
    assert!(hand.is_double_pair());
}

#[test]
fn test_four_is_not_double_pair() {
    assert!(!Hand::from("AATAA").is_double_pair());
}

#[test]
fn test_is_pair() {
    let hand: Hand = "A234A".into();
    println!("{}", hand);
    assert!(hand.is_pair());
}

#[test]
fn test_comparison_same_type() {
    let greater: Hand = "KK677".into();
    let lesser: Hand = "KTJJT".into();
    assert!(greater.is_pair() && lesser.is_pair());
    assert!(greater > lesser);
}

#[test]
fn test_full_house_2() {
    let first: Hand = "77J7T".into();
    let second: Hand = "77JJJ".into();

    assert!(!first.is_full_house());
    assert!(second.is_full_house());
}

#[test]
fn test_correct_order() {
    let mut hands: Vec<Hand> = parse_input(TEST)
        .into_iter()
        .map(|(hand, _)| hand)
        .collect();
    hands.sort();
    for h in &hands {
        println!("{}", h);
    }
}

#[test]
fn test_total_winnings() {
    let hands = parse_input(INPUT);
    let result = determine_total_winnings(hands);
    println!("{result}");
    //assert_eq!(result ,6440);
}
