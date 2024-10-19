use std::{
    collections::{HashMap, VecDeque},
    iter::Sum,
    ops::Add,
};

#[allow(dead_code)]
static TEST: &str = include_str!("../data/d20t");

#[allow(dead_code)]
static TEST2: &str = include_str!("../data/d20t2");

static INPUT: &str = include_str!("../data/d20");

type EdgeIndex = usize;
type VertexIndex = usize;

#[derive(Debug, Clone, PartialEq)]
struct Pulses {
    low: usize,
    high: usize,
}

impl Add<Self> for Pulses {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            low: self.low + other.low,
            high: self.high + other.high,
        }
    }
}

impl Sum<Self> for Pulses {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Pulses { low: 0, high: 0 }, |acc, n| acc + n)
    }
}

#[derive(Debug, Clone)]
struct Module {
    name: String,
    m_type: ModuleType,
}

#[derive(Debug, Clone, Copy)]
enum ModuleType {
    FlipFlop(bool),
    Conjunction,
    Broadcast,
    Out,
}

#[derive(Debug)]
struct Wire {
    signal: bool,
}

#[derive(Debug)]
struct EdgeList {
    incoming: EdgeIndex,
    outgoing: VertexIndex,
}

impl EdgeList {
    fn new() -> Self {
        Self {
            incoming: EdgeIndex::MAX,
            outgoing: EdgeIndex::MAX,
        }
    }

    fn new_with_indices(incoming: EdgeIndex, outgoing: EdgeIndex) -> Self {
        Self { incoming, outgoing }
    }
}

#[derive(Debug)]
struct Vertex<V> {
    weight: V,
    next: EdgeList, // edge index
}

#[derive(Debug)]
struct Edge<E> {
    weight: E,
    next: EdgeList, // edge index
    head: VertexIndex,
    tail: VertexIndex,
}

#[derive(Debug)]
struct DiGraph<V, E> {
    vertices: Vec<Vertex<V>>,
    edges: Vec<Edge<E>>,
}

impl<V, E> DiGraph<V, E> {
    fn new() -> Self {
        Self {
            vertices: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn get_vertex(&self, id: VertexIndex) -> Option<&V> {
        self.vertices.get(id).map(|v| &v.weight)
    }

    fn get_vertex_mut(&mut self, id: VertexIndex) -> Option<&mut V> {
        self.vertices.get_mut(id).map(|v| &mut v.weight)
    }

    fn get_edge(&self, id: EdgeIndex) -> Option<&E> {
        self.edges.get(id).map(|e| &e.weight)
    }

    fn get_edge_mut(&mut self, id: EdgeIndex) -> Option<&mut E> {
        self.edges.get_mut(id).map(|e| &mut e.weight)
    }

    fn edge_endpoints(&self, id: EdgeIndex) -> Option<(VertexIndex, VertexIndex)> {
        self.edges.get(id).map(|e| (e.tail, e.head))
    }

    fn add_vertex(&mut self, vertex: V) -> usize {
        let v = Vertex {
            weight: vertex,
            next: EdgeList::new(),
        };
        self.vertices.push(v);
        self.vertices.len() - 1
    }

    fn add_edge(&mut self, edge: E, tail: usize, head: usize) -> usize {
        // don't support edges that point to the same vertices
        if head == tail {
            panic!("Edges to the same vertex are not supported.");
        }
        if head >= self.vertices.len() || tail >= self.vertices.len() {
            panic!("Vertex not contained in graph");
        }

        // for tail, this edge is outgoing
        let tail_vertex = self.vertices.get_mut(tail).unwrap();
        let outgoing = tail_vertex.next.outgoing;
        tail_vertex.next.outgoing = self.edges.len();

        // for head, incoming
        let head_vertex = self.vertices.get_mut(head).unwrap();
        let incoming = head_vertex.next.incoming;
        head_vertex.next.incoming = self.edges.len();

        let next = EdgeList::new_with_indices(incoming, outgoing);
        let edge = Edge {
            weight: edge,
            next,
            tail,
            head,
        };

        self.edges.push(edge);

        self.edges.len() - 1
    }

    fn incoming_edges(&self, vertex: VertexIndex) -> Vec<EdgeIndex> {
        if vertex >= self.vertices.len() {
            panic!("Vertex not contained in graph");
        }
        let mut edges = Vec::new();
        let mut cur = &self.vertices[vertex].next;

        while cur.incoming != VertexIndex::MAX {
            edges.push(cur.incoming);
            cur = &self.edges[cur.incoming].next;
        }

        edges
    }

    fn outgoing_edges(&self, vertex: VertexIndex) -> Vec<EdgeIndex> {
        if vertex >= self.vertices.len() {
            panic!("Vertex not contained in graph");
        }
        let mut edges = Vec::new();
        let mut cur = &self.vertices[vertex].next;

        while cur.outgoing != EdgeIndex::MAX {
            edges.push(cur.outgoing);
            cur = &self.edges[cur.outgoing].next;
        }

        edges
    }
}

fn parse_input(inp: &str) -> DiGraph<Module, Wire> {
    let mut graph = DiGraph::new();
    let mut ids = HashMap::new();
    let input: Vec<(&str, Vec<&str>)> = inp
        .lines()
        .map(|line| {
            line.split("->")
                .map(|word| word.trim())
                .collect::<Vec<&str>>()
        })
        .map(|words| {
            (
                words[0],
                words[1].split(',').map(|word| word.trim()).collect(),
            )
        })
        .collect();

    // first add modules to graph
    for (from, _) in &input {
        let (id, name) = if from.starts_with('%') {
            // flip flop
            let name = &from[1..];
            (
                graph.add_vertex(Module {
                    name: name.to_string(),
                    m_type: ModuleType::FlipFlop(false),
                }),
                name,
            )
        } else if from.starts_with('&') {
            // conjunction
            let name = &from[1..];
            (
                graph.add_vertex(Module {
                    name: name.to_string(),
                    m_type: ModuleType::Conjunction,
                }),
                name,
            )
        } else {
            assert_eq!(from, &"broadcaster");
            (
                graph.add_vertex(Module {
                    name: from.to_string(),
                    m_type: ModuleType::Broadcast,
                }),
                *from,
            )
        };
        ids.insert(name, id);
    }

    println!("{ids:?}");
    // add edges
    for (from, dests) in &input {
        let from_name = match from == &"broadcaster" {
            true => from,
            false => &from[1..],
        };
        let from_id = ids.get(from_name).expect("Module not found in graph");
        for dest in dests {
            let id = match ids.get(dest) {
                Some(id) => *id,
                None => graph.add_vertex(Module {
                    name: dest.to_string(),
                    m_type: ModuleType::Out,
                }),
            };
            graph.add_edge(Wire { signal: false }, *from_id, id);
        }
    }

    graph
}

trait Circuit {
    fn push_button(&mut self, conjunctions: &mut HashMap<String, bool>) -> Pulses;
    fn push_button_repeatedly(&mut self, repeats: usize) -> usize;
    fn all_off(&self) -> bool;
}

impl Circuit for DiGraph<Module, Wire> {
    fn push_button(&mut self, conjunctions: &mut HashMap<String, bool>) -> Pulses {
        let broadcaster = self
            .vertices
            .iter()
            .enumerate()
            .find_map(|(i, m)| match m.weight.name == "broadcaster" {
                true => Some(i),
                false => None,
            })
            .expect("No broadcaster in circuit");

        let mut queue = VecDeque::from([(broadcaster, false)]);
        let mut pulses_send = Pulses { low: 0, high: 0 };

        while let Some((id, pulse)) = queue.pop_front() {
            let m_name = self
                .get_vertex_mut(id)
                .expect("Module not found")
                .name
                .clone();
            let m_type = &mut self.get_vertex_mut(id).expect("Module not found").m_type;

            match pulse {
                true => pulses_send.high += 1,
                false => pulses_send.low += 1,
            }

            let send_pulse = match m_type {
                ModuleType::FlipFlop(is_on) => match pulse {
                    false => {
                        *is_on = !*is_on;
                        *is_on
                    }
                    true => continue,
                },
                ModuleType::Conjunction => {
                    let send_pulse = !self
                        .incoming_edges(id)
                        .into_iter()
                        .all(|e| self.get_edge(e).unwrap().signal);
                    let e = conjunctions.get_mut(&m_name).unwrap();
                    *e = send_pulse;
                    //conjunctions.entry(m_name).and_modify(|p| *p = send_pulse);
                    send_pulse
                }
                ModuleType::Broadcast => pulse,
                ModuleType::Out => {
                    if !pulse {
                        return Pulses {
                            high: usize::MAX,
                            low: usize::MAX,
                        };
                    } else {
                        continue;
                    }
                }
            };

            for e in self.outgoing_edges(id).iter().rev().copied() {
                self.get_edge_mut(e).unwrap().signal = send_pulse;
                queue.push_back((self.edge_endpoints(e).unwrap().1, send_pulse));
            }
        }

        pulses_send
    }

    fn push_button_repeatedly(&mut self, repeats: usize) -> usize {
        let pulses = (0..repeats)
            .map(|_| self.push_button(&mut HashMap::new()))
            .sum::<Pulses>();
        pulses.low * pulses.high
    }

    fn all_off(&self) -> bool {
        let n_flip_flops = self
            .vertices
            .iter()
            .filter(|v| match v.weight.m_type {
                ModuleType::FlipFlop(_) => true,
                _ => false,
            })
            .count();

        let n_flip_flops_off = self
            .vertices
            .iter()
            .filter(|v| match v.weight.m_type {
                ModuleType::FlipFlop(s) => !s,
                _ => false,
            })
            .count();
        //println!("{n_flip_flops}: {n_flip_flops_off}");
        n_flip_flops == n_flip_flops_off
    }
}

pub fn get_solution_1() -> usize {
    let mut circuit = parse_input(INPUT);
    circuit.push_button_repeatedly(1000)
}

pub fn get_solution_2() -> usize {
    let mut circuit = parse_input(INPUT);
    let last = Pulses {
        high: usize::MAX,
        low: usize::MAX,
    };
    // get all conjunction modules
    let mut conjunctions = circuit
        .vertices
        .iter()
        .filter(|v| match v.weight.m_type {
            ModuleType::Conjunction => true,
            _ => false,
        })
        .map(|v| (v.weight.name.clone(), false))
        .collect::<HashMap<String, bool>>();

    for i in 0.. {
        let pulses = circuit.push_button(&mut conjunctions);
        print!("{i}:\t");
        for (name, pulse) in &conjunctions {
            print!("{name}: {pulse}\t");
        }
        println!("");
        if pulses == last {
            return i;
        }
    }
    0
}

#[test]
fn test_get_solution_1() {
    println!("{}", get_solution_2());
}

#[test]
fn di_graph_add_edge() {
    let mut graph = DiGraph::<usize, usize>::new();

    graph.add_vertex(0);
    graph.add_vertex(1);

    graph.add_edge(0, 0, 1);
}

#[test]
fn di_graph_outgoing_edges() {
    let mut graph = DiGraph::<usize, usize>::new();

    graph.add_vertex(0);
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_vertex(3);

    graph.add_edge(0, 0, 1);
    graph.add_edge(1, 1, 2);
    graph.add_edge(2, 1, 3);

    let edges = graph.outgoing_edges(1);
    println!("{:?}", graph);
    assert_eq!(edges.len(), 2);

    for edge in edges {
        assert_eq!(graph.edges[edge].tail, 1);
    }
}

#[test]
fn di_graph_incoming_edges() {
    let mut graph = DiGraph::<usize, usize>::new();

    graph.add_vertex(0);
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_vertex(3);
    graph.add_vertex(4);

    graph.add_edge(0, 0, 1);
    graph.add_edge(1, 1, 2);
    graph.add_edge(2, 3, 2);
    graph.add_edge(3, 2, 4);
    graph.add_edge(4, 3, 4);
    graph.add_edge(5, 1, 4);

    let edges = graph.incoming_edges(2);
    assert_eq!(edges.len(), 2);

    for edge in edges {
        assert_eq!(graph.edges[edge].head, 2);
    }

    let edges = graph.incoming_edges(4);
    assert_eq!(edges.len(), 3);

    for edge in edges {
        assert_eq!(graph.edges[edge].head, 4);
    }
}
