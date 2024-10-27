use std::collections::{HashMap, VecDeque};

#[allow(dead_code)]
static TEST: &str = include_str!("../data/d20t");

#[allow(dead_code)]
static TEST2: &str = include_str!("../data/d20t2");

static INPUT: &str = include_str!("../data/d20");

type HMGraph = HashMap<&'static str, Node>;

struct Node {
    typ: ModuleTypeMem,
    last_pulse: bool,
    children: Vec<&'static str>,
}

impl Node {
    fn new(name: &str, children: Vec<&'static str>) -> Self {
        let typ = match name {
            n if n == "broadcaster" => ModuleTypeMem::Broadcast,
            n if n.starts_with("&") => ModuleTypeMem::Conjunction(HashMap::new()),
            n if n.starts_with("%") => ModuleTypeMem::FlipFlop(false),
            n if n == "rx" => ModuleTypeMem::Out,
            _ => panic!("invalid module type"),
        };

        Self {
            typ,
            last_pulse: false,
            children,
        }
    }
}

#[derive(Debug, Clone)]
enum ModuleTypeMem {
    FlipFlop(bool),
    Conjunction(HashMap<&'static str, bool>),
    Broadcast,
    Out,
}

impl ModuleTypeMem {
    fn handle_pulse(&mut self, pulse: bool, parent: &str) -> Option<bool> {
        let pulse_to_send = match self {
            ModuleTypeMem::FlipFlop(cur) => match pulse {
                true => return None,
                false => {
                    *cur = !*cur;
                    *cur
                }
            },
            ModuleTypeMem::Conjunction(mem) => {
                *mem.get_mut(parent).unwrap() = pulse;
                !mem.values().all(|p| *p)
            }
            ModuleTypeMem::Broadcast => false,
            ModuleTypeMem::Out => return None,
        };
        Some(pulse_to_send)
    }
}

fn parse_input(inp: &'static str) -> HMGraph {
    let mut graph = HashMap::new();
    // determine modules types and children
    for line in inp.lines() {
        let parts = line.split("->").collect::<Vec<&str>>();
        let module_def = parts[0].trim();
        let children = parts[1].split(',').map(|m| m.trim()).collect::<Vec<_>>();

        let node = Node::new(module_def, children);
        if module_def.starts_with(['&', '%']) {
            graph.insert(&module_def[1..], node);
        } else {
            graph.insert(module_def, node);
        }
    }
    // insert the 'rx' module
    graph.insert("rx", Node::new("rx", Vec::new()));

    // determine the parents of the modules
    for line in inp.lines() {
        let parts = line.split("->").collect::<Vec<&str>>();
        let parent = parts[0].trim().trim_start_matches(['&', '%']);
        let children = parts[1].split(',').map(|m| m.trim()).collect::<Vec<_>>();

        for child in children {
            let node = graph.get_mut(child).unwrap();
            match &mut node.typ {
                ModuleTypeMem::Conjunction(mem) => _ = mem.insert(parent, false),
                _ => (),
            }
        }
    }

    graph
}

pub fn get_solution_1() -> usize {
    let mut g = parse_input(INPUT);
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _ in 0..1000 {
        let mut queue = VecDeque::from([("broadcaster", false, "dummy")]);

        while let Some((cur, pulse, parent)) = queue.pop_front() {
            match pulse {
                true => high_pulses += 1,
                false => low_pulses += 1,
            }

            let node = g.get_mut(cur).unwrap();

            node.last_pulse = match node.typ.handle_pulse(pulse, parent) {
                None => continue,
                Some(pulse_to_send) => pulse_to_send,
            };

            for child in &node.children {
                queue.push_back((child, node.last_pulse, cur));
            }
        }
    }

    high_pulses * low_pulses
}

pub(crate) fn get_solution_2() -> usize {
    let mut g = parse_input(INPUT);
    /* these could be determined programmaticly by looking at the parents */
    let mut conj = HashMap::from([("br", None), ("lf", None), ("rz", None), ("fk", None)]);

    for i in 1.. {
        let mut queue = VecDeque::from([("broadcaster", false, "dummy")]);

        while let Some((cur, pulse, parent)) = queue.pop_front() {
            let node = g.get_mut(cur).unwrap();

            node.last_pulse = match node.typ.handle_pulse(pulse, parent) {
                None => continue,
                Some(pulse_to_send) => pulse_to_send,
            };

            match conj.iter().find(|(m, _)| **m == cur) {
                Some((_, None)) if node.last_pulse => *conj.get_mut(cur).unwrap() = Some(i),
                _ => (),
            }

            for child in &node.children {
                queue.push_back((child, node.last_pulse, cur));
            }
        }

        /* for each conjunction module feeding into the rx, the cycle has been detected */
        if conj.values().all(|v| v.is_some()) {
            break;
        }
    }
    conj.values().filter_map(|v| *v).product::<usize>()
}
