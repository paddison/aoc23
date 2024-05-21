use std::{collections::HashMap, fmt::Display, num::ParseIntError};

#[allow(dead_code)]
static TEST: &str = include_str!("../data/d19t");
static INPUT: &str = include_str!("../data/d19");

#[derive(Debug)]
enum InputError {
    Category(String),
    Comparison(String),
    Params(String),
    Numerical(ParseIntError),
    _Rule(String),
    Workflow(String),
    Part(String),
}

impl std::error::Error for InputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Numerical(err) => Some(err),
            _ => None,
        }
    }
}

impl From<ParseIntError> for InputError {
    fn from(err: ParseIntError) -> Self {
        Self::Numerical(err)
    }
}

impl Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            InputError::Category(msg) => format!("Category - {}", msg),
            InputError::Comparison(msg) => format!("Comparison - {}", msg),
            InputError::Params(msg) => format!("Params - {}", msg),
            InputError::Numerical(err) => format!("Numerical - {}", err.to_string()),
            InputError::_Rule(msg) => format!("Rule - {}", msg),
            InputError::Workflow(msg) => format!("Workflow - {}", msg),
            InputError::Part(msg) => format!("Part - {}", msg),
        };
        write!(f, "{}", msg)
    }
}

#[derive(Debug, Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

impl TryFrom<&str> for Category {
    type Error = InputError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "x" => Ok(Self::X),
            "m" => Ok(Self::M),
            "a" => Ok(Self::A),
            "s" => Ok(Self::S),
            s => Err(InputError::Category(format!("Invalid input: {}", s))),
        }
    }
}

#[derive(Debug)]
enum Comparison {
    Greater,
    Less,
}

impl TryFrom<&str> for Comparison {
    type Error = InputError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "<" => Ok(Self::Less),
            ">" => Ok(Self::Greater),
            s => Err(InputError::Comparison(format!("Invalid input: {}", s))),
        }
    }
}

#[derive(Debug)]
struct Workflow(String, Vec<Rule>);

impl TryFrom<&str> for Workflow {
    type Error = InputError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (rule_start, rule_end) = match (value.find('{'), value.find('}')) {
            (Some(start), Some(end)) => (start, end),
            _ => return Err(InputError::Workflow("No rules found".to_string())),
        };

        let name = value[..rule_start].to_string();
        let rules: Vec<Rule> = value[rule_start + 1..rule_end]
            .split(',')
            .map(|rule| Rule::try_from(rule))
            .collect::<Result<Vec<Rule>, Self::Error>>()?;

        Ok(Self(name, rules))
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Destination {
    Accepted,
    Rejected,
    Other(String),
}

impl From<&str> for Destination {
    fn from(value: &str) -> Self {
        match value {
            "A" => Self::Accepted,
            "R" => Self::Rejected,
            s => Self::Other(s.to_string()),
        }
    }
}

#[derive(Debug)]
enum Rule {
    Dest(Destination),
    Eval(Params),
}

impl TryFrom<&str> for Rule {
    type Error = InputError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.find(':') {
            Some(_) => Ok(Self::Eval(value.try_into()?)),
            None => Ok(Self::Dest(value.into())),
        }
    }
}

#[derive(Debug)]
struct Params {
    category: Category,
    cmp: Comparison,
    val: usize,
    dest: Destination,
}

impl TryFrom<&str> for Params {
    type Error = InputError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let colon = value
            .find(':')
            .ok_or(InputError::Params("No colon found".to_string()))?;
        if colon == value.len() - 1 {
            return Err(InputError::Params("No destination found".to_string()));
        }
        let category = value[0..1].try_into()?;
        let cmp = value[1..2].try_into()?;
        let val = usize::from_str_radix(&value[2..colon], 10)?;
        let dest = value[colon + 1..].into();

        Ok(Self {
            category,
            cmp,
            val,
            dest,
        })
    }
}

impl PartialEq<Part> for Params {
    fn eq(&self, other: &Part) -> bool {
        self.val == other.extract_value(self.category)
    }
}

impl PartialOrd<Part> for Params {
    fn partial_cmp(&self, other: &Part) -> Option<std::cmp::Ordering> {
        self.val.partial_cmp(&other.extract_value(self.category))
    }
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl TryFrom<&str> for Part {
    type Error = InputError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !(value.starts_with('{') && value.ends_with('}')) {
            return Err(InputError::Part("Not contained in {{}}".to_string()));
        }

        let values = value[1..value.len() - 1]
            .split(',')
            .map(|s| usize::from_str_radix(&s[2..], 10))
            .collect::<Result<Vec<_>, _>>()?;

        if let [x, m, a, s] = values[..] {
            Ok(Part { x, m, a, s })
        } else {
            Err(InputError::Part("Missing categories".to_string()))
        }
    }
}

impl Part {
    fn extract_value(&self, category: Category) -> usize {
        match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }

    fn apply_rules<'a>(&self, rules: &'a [Rule]) -> &'a Destination {
        for rule in rules {
            match rule {
                Rule::Dest(dest) => return dest,
                Rule::Eval(params) => {
                    if match &params.cmp {
                        Comparison::Greater => params < self,
                        Comparison::Less => params > self,
                    } {
                        return &params.dest;
                    }
                }
            }
        }
        panic!("couldn't match rule");
    }

    fn sort<'a>(&self, workflows: &'a HashMap<String, Vec<Rule>>) -> &'a Destination {
        let mut wf = &"in".to_string();
        loop {
            if let Some(rules) = workflows.get(wf) {
                wf = match self.apply_rules(rules) {
                    Destination::Other(next) => &next,
                    other => break other,
                };
            }
        }
    }

    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

fn parse_input(inp: &str) -> Result<(HashMap<String, Vec<Rule>>, Vec<Part>), InputError> {
    let mut iter = inp.lines();

    let workflows = iter
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| Workflow::try_from(l).map(|Workflow(name, rules)| (name, rules)))
        .collect::<Result<HashMap<String, Vec<Rule>>, InputError>>()?;

    let parts = iter
        .map(|l| Part::try_from(l))
        .collect::<Result<Vec<_>, _>>()?;

    Ok((workflows, parts))
}

pub fn get_solution_1() -> usize {
    match parse_input(INPUT) {
        Ok((workflows, parts)) => parts
            .into_iter()
            .filter(|p| p.sort(&workflows) == &Destination::Accepted)
            .map(|p| p.sum())
            .sum(),
        Err(e) => {
            println!("{e}");
            0
        }
    }
}
