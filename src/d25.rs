use std::{
    collections::{HashMap, HashSet},
    time::{SystemTime, UNIX_EPOCH},
};

#[allow(unused)]
static TEST: &str = include_str!("../data/d25t");
static INPUT: &str = include_str!("../data/d25");

struct Rng {
    state: usize,
}

impl Rng {
    fn seed(seed: usize) -> Self {
        Self { state: seed }
    }

    fn seed_from_epoch() -> Self {
        let state = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as usize;

        Self { state }
    }

    fn next(&mut self) -> usize {
        let mut tmp = self.state;
        tmp ^= tmp << 13;
        tmp ^= tmp >> 17;
        tmp ^= tmp << 5;
        self.state = tmp;

        tmp
    }
}

struct Graph {
    n_edges: usize,
    n_vertices: usize,
    edges: Vec<(usize, usize)>,
}

#[derive(Clone, Copy)]
struct UnionRank {
    parent: usize,
    rank: usize,
}

/**
* Represents a tree data structure, which can be used to
* partition arrays into subsets.
*
*/
trait UnionFind {
    type Item;

    fn find(&mut self, id: usize) -> Self::Item;
    fn union(&mut self, a: Self::Item, b: Self::Item);
}

impl UnionFind for Vec<UnionRank> {
    type Item = usize;

    /**
     * Finds the parent of 'id' in the tree.
     */
    fn find(&mut self, id: usize) -> Self::Item {
        let mut item = self[id];

        if item.parent != id {
            item.parent = self.find(item.parent);
        }

        item.parent
    }

    /**
     * Merges the two sets which contain a and b.
     */
    fn union(&mut self, a: Self::Item, b: Self::Item) {
        let mut root_a = self.find(a);
        let mut root_b = self.find(b);

        /* both a and b are in the same set */
        if root_a == root_b {
            return;
        }

        /* swap variables, since root_a is always assumed to be the root */
        if self[root_a].rank < self[root_b].rank {
            (root_a, root_b) = (root_b, root_a);
        }

        self[root_b].parent = root_a;

        /* Increment the rank of a if both subtrees have equal rank */
        if self[root_a].rank == self[root_b].rank {
            self[root_a].rank += 1;
        }
    }
}

fn karger(graph: &Graph, seed_opt: Option<usize>) -> Vec<UnionRank> {
    let mut rng = match seed_opt {
        Some(seed) => Rng::seed(seed),
        None => Rng::seed_from_epoch(),
    };

    /* store each vertice in a distinct subset */
    let mut subsets: Vec<_> = (0..graph.n_vertices)
        .map(|v| UnionRank { parent: v, rank: 0 })
        .collect();

    let mut vertices_left = graph.n_vertices;

    while vertices_left > 2 {
        /* select a random edge */
        let edge_index = rng.next() % graph.n_edges;
        let edge = graph.edges[edge_index];

        /* check if this edge was compacted already */
        let root_u = subsets.find(edge.0);
        let root_v = subsets.find(edge.1);

        /* if not compact the edge, by uniting the subsets */
        if root_u != root_v {
            subsets.union(root_u, root_v);
            vertices_left -= 1;
        }
    }

    subsets
}

fn parse_input(inp: &'static str) -> Graph {
    /* collect a hashset of all nodes*/
    let mut vertices = HashSet::new();
    let mut edges = Vec::new();

    /* collect a vec of all the edges */
    for line in inp.lines() {
        let parsed_line = line.split(':').collect::<Vec<_>>();
        let parent = parsed_line[0].trim();
        let children = parsed_line[1]
            .split_whitespace()
            .map(|s| s.trim())
            .collect::<HashSet<&str>>();

        vertices.insert(parent);
        for child in children {
            vertices.insert(child);
            edges.push((parent, child));
        }
    }
    /* lookup table for vertices to their indices */
    let index_table: HashMap<&str, usize> = vertices
        .iter()
        .enumerate()
        .map(|(i, name)| (*name, i))
        .collect();

    let edges_as_indices: Vec<_> = edges
        .into_iter()
        .map(|(u, v)| (*index_table.get(u).unwrap(), *index_table.get(v).unwrap()))
        .collect();

    let graph = Graph {
        n_edges: edges_as_indices.len(),
        n_vertices: index_table.len(),
        edges: edges_as_indices,
    };

    graph
}

fn find_minimal_cut(graph: &Graph) -> usize {
    loop {
        let mut subsets = karger(&graph, None);
        /* determine the cut edges */
        let edges: Vec<_> = graph
            .edges
            .iter()
            .filter(|(u, v)| subsets.find(*u) != subsets.find(*v))
            .collect();

        /* storing the size of each subset */
        let mut sizes = HashMap::new();

        for v in 0..graph.n_vertices {
            let root_v = subsets.find(v);
            sizes.entry(root_v).and_modify(|n| *n += 1).or_insert(1);
        }

        if edges.len() == 3 {
            break sizes.values().product::<usize>();
        }
    }
}

pub(crate) fn get_solution_1() -> usize {
    let graph = parse_input(INPUT);
    find_minimal_cut(&graph)
}
