use std::{cmp::Ordering, fmt::Display};

#[allow(dead_code)]
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
    bids.sort_by(|(lhs, _), (rhs, _)| lhs.compare(rhs));

    bids.into_iter()
        .enumerate()
        .map(|(i, (_, amount))| amount * (i + 1))
        .sum()
}

pub(crate) fn get_solution_1() -> usize {
    let bids = parse_input(INPUT);
    determine_total_winnings(bids)
}

pub(crate) fn get_solution_2() -> usize {
    let mut bids = parse_input(INPUT);
    for (hand, _) in &mut bids {
        hand.p2 = true;
    }
    determine_total_winnings(bids)
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
enum Card {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    T = 10,
    J = 1,
    Q = 12,
    K = 13,
    A = 14,
}

impl Card {
    fn value(&self, is_joker: bool) -> usize {
        if self == &Card::J {
            if is_joker {
                1
            } else {
                11
            }
        } else {
            *self as usize
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c: char = char::from(*self);
        write!(f, "{c}")
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::Two
    }
}

impl From<char> for Card {
    fn from(input: char) -> Self {
        match input {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
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
            Card::Two => '2',
            Card::Three => '3',
            Card::Four => '4',
            Card::Five => '5',
            Card::Six => '6',
            Card::Seven => '7',
            Card::Eight => '8',
            Card::Nine => '9',
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
    p2: bool,
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

impl From<&str> for Hand {
    fn from(input: &str) -> Self {
        let mut cards = [Card::default(); 5];
        input
            .chars()
            .enumerate()
            .for_each(|(i, c)| cards[i] = c.into());
        Self { cards, p2: false }
    }
}

impl Hand {
    fn compare(&self, other: &Self) -> Ordering {
        let lhs = if self.p2 {
            self.score_joker()
        } else {
            self.score()
        };

        let rhs = if other.p2 {
            other.score_joker()
        } else {
            other.score()
        };

        match lhs.cmp(&rhs) {
            Ordering::Equal => self.compare_strongest_card(other),
            ordering => ordering,
        }
    }

    fn compare_strongest_card(&self, other: &Self) -> Ordering {
        let p2 = self.p2;
        for (lhs, rhs) in self.cards.iter().zip(other.cards) {
            match lhs.value(p2).cmp(&rhs.value(p2)) {
                Ordering::Equal => continue,
                ordering => return ordering,
            }
        }
        Ordering::Equal
    }

    fn is_five(&self) -> bool {
        self.is_n(5, 0).is_some()
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

    fn is_n(&self, n: usize, start: usize) -> Option<Card> {
        // ignore jokers
        for i in start..(6 - n) {
            if self.cards[i] == Card::J && self.p2 {
                continue;
            }
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

    fn upgrade(&self, score: usize) -> usize {
        match score {
            6 => unreachable!(), // five of a kind can't be upgraded
            5 => 6,              // four gets upgraded to five
            4 => unreachable!(), // full house can't be upgraded
            3 => 5,              // three gets upgraded to four
            2 => 4,              // double pair gets upgraded to full house
            1 => 3,              // pair gets upgraded to three
            0 => 1,              // high card gets upgraded to pair
            _ => unreachable!(),
        }
    }

    // joker will upgrade to the next best:
    // four -> five
    // three -> four
    // full house stays
    // two pair -> full house
    // pair -> three
    // high card -> two
    //
    // will also depend on joker count
    // solution: make upgrade function, analyzes each card and then upgrades the rank
    fn score_joker(&self) -> usize {
        if self.cards.iter().all(|c| *c == Card::J) {
            return 6;
        }
        let mut score = self.score();
        for c in self.cards {
            if c == Card::J {
                score = self.upgrade(score);
            }
        }
        score
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
    assert!(greater.compare(&lesser) == Ordering::Greater);
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
    hands.sort_by(|a, b| a.compare(&b));
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

#[test]
fn test_score_joker_four() {
    let hand: Hand = "T55J5".into();
    assert_eq!(hand.score_joker(), 5);
}

#[test]
fn test_score_joker_full_house() {
    let hand: Hand = "2233J".into();
    assert_eq!(hand.score_joker(), 4);
}

#[test]
fn test_solution_p2() {
    println!("{}", get_solution_1());
}
