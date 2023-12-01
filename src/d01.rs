#[allow(dead_code)]
static TEST: &'static str = include_str!("../data/d01t");
#[allow(dead_code)]
static TEST2: &'static str = include_str!("../data/d01t2");
#[allow(dead_code)]
static INP: &'static str = include_str!("../data/d01");

fn parse_input(inp: &str) -> Vec<&str> {
    inp.lines().collect()
}

fn parse_number(chars: &[char]) -> Option<u32> {
    if chars[0].is_digit(10) { 
        chars[0].to_digit(10)
    } else {
        match chars {
            &['o', 'n', 'e', ..] => Some(1),
            &['t', 'w', 'o', ..] => Some(2),
            &['t', 'h', 'r', 'e', 'e', ..] => Some(3),
            &['f', 'o', 'u', 'r', ..] => Some(4),
            &['f', 'i', 'v', 'e', ..] => Some(5),
            &['s', 'i', 'x', ..] => Some(6),
            &['s', 'e', 'v', 'e', 'n', ..] => Some(7),
            &['e', 'i', 'g', 'h', 't', ..] => Some(8),
            &['n', 'i', 'n', 'e', ..] => Some(9),
            &['z', 'e', 'r', 'o', ..] => Some(0),
            _ => None,
        }
    }


}

pub(crate) fn get_solution_1() -> u32 {
    let mut sum = 0;
    for line in parse_input(INP) {
        sum += line.chars().find_map(|c| c.to_digit(10)).unwrap() * 10 + 
        line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
    }

    sum
}

pub(crate) fn get_solution_2() -> u32 {
    let mut sum = 0;
    for line in parse_input(INP) {
        let len = line.len();
        let chars = line.chars().collect::<Vec<_>>();
        for i in 0..len {
            if let Some(n) = parse_number(&chars[i..]) {
                println!("{n}");
                sum += n * 10;
                break;
            }
        }

        for i in (0..len).rev() {
            if let Some(n) = parse_number(&chars[i..]) {
                println!("{n}");
                sum += n;
                break;
            }
        }
    }

    sum
}

#[test]
fn test_part2() {
    assert_eq!(281, get_solution_2());
}
