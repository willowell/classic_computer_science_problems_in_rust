/// # Build Your Own Graph
///
/// Before working through Kopec's examples for this chapter, we have a problem to solve:
/// how do we implement a graph in Rust?
/// Of course, we can always implement a graph as an adjacency list, even DAGs, but cyclical graphs present a particular challenge
/// - how do we express a cyclical graph using Rust's ownership and borrowing, if we cannot point to one root node
/// and call that the owner?
///
/// Indeed, Kopec's use of Java allows him to circumvent this altogether, but you see this issue pop up in his [SwiftGraph](https://github.com/davecom/SwiftGraph)
/// library for the Swift implementation. Since Swift uses automatic reference counting, we can't rely on a garbage collector
/// to figure out how to manage a cyclical graph.
///
/// So, what do we do? Do we give up and just use [petgraph](https://github.com/petgraph/petgraph)?
///
/// No.
///
/// Do we rewrite everything in a GC'd language - say, Haskell?
///
/// YES!!!!!
///
/// Just kidding. ;-)
///
/// (Although a Haskell port would be very interesting!)
///
/// Well, why don't we take a look at [the C++ port](https://github.com/araya-andres/classic_computer_sci) for ideas?
/// Andres Araya thankfully licensed it under the Unlicense, so we are free to use their code however we see fit.
///
/// Their port uses C++20, and their [graph implementation](https://github.com/araya-andres/classic_computer_sci/blob/master/ch4/graph.cc)
/// looks promising from a potential Rust port perspective:
/// * no classes, just simple structs which may have a generic field
/// * generic associated types via `using` declarations
/// * responsible uses of `const` methods and parameters.
///
///
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct EdgeUV {
    u: i32,
    v: i32,
}

trait Edge {
    fn get_uv(&self) -> &EdgeUV;
    fn reversed(&self) -> Self;
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct UnweightedEdge {
    uv: EdgeUV,
}

#[derive(Clone, Debug)]
struct WeightedEdge<W> {
    uv: EdgeUV,
    weight: W,
}

impl Edge for UnweightedEdge {
    fn get_uv(&self) -> &EdgeUV {
        &self.uv
    }

    fn reversed(&self) -> Self {
        UnweightedEdge {
            uv: EdgeUV {
                u: self.uv.v,
                v: self.uv.u,
            },
        }
    }
}

impl fmt::Display for UnweightedEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{} <-> {}}}", self.uv.u, self.uv.v)
    }
}

struct Graph<V, E>
where
    E: Edge + PartialEq,
{
    vertices: Vec<V>,
    edges: Vec<E>,
}

impl<V, E> Graph<V, E>
where
    V: Clone + PartialEq,
    E: Clone + Edge + PartialEq,
{
    fn new(vertices: Vec<V>) -> Self {
        Self {
            vertices,
            edges: Default::default(),
        }
    }

    fn get_vertex_count(&self) -> usize {
        self.vertices.len()
    }

    fn get_edge_count(&self) -> usize {
        self.edges.len()
    }

    fn insert_vertex(&mut self, v: V) -> usize {
        self.vertices.push(v);

        self.vertices.len() - 1
    }

    fn insert_edge(&mut self, e: E) {
        if !self.edges.contains(&e) {
            self.edges.push(e.clone());
            self.edges.push(e.reversed());
        }
    }

    fn get_index_of(&self, v: &V) -> Option<usize> {
        self.vertices.iter().position(|u| u == v)
    }

    fn get_neighbours_for(&self, v: &V) -> Vec<V> {
        self.get_neighbours_for_index(self.get_index_of(&v).unwrap())
    }

    fn get_neighbours_for_index(&self, index: usize) -> Vec<V> {
        let mut neighbours: Vec<V> = vec![];

        for e in &self.edges {
            if e.get_uv().u == index as i32 {
                neighbours.push(self.vertices[e.get_uv().v as usize].clone());
            }
        }

        neighbours
    }

    fn get_edges_for_index(&self, index: usize) -> Vec<E> {
        let mut edges: Vec<E> = vec![];

        for e in &self.edges {
            if e.get_uv().u == index as i32 {
                edges.push(e.clone());
            }
        }

        edges
    }
}

impl<V, E> fmt::Display for Graph<V, E>
where
    V: Clone + fmt::Display + fmt::Debug + PartialEq,
    E: Clone + Edge + PartialEq,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.vertices
            .iter()
            .map(|v| writeln!(f, "{} -> {:?}", v, self.get_neighbours_for(v)))
            .collect()
    }
}

struct UnweightedGraph<V> {
    graph: Graph<V, UnweightedEdge>,
}

impl<V> UnweightedGraph<V> {
    fn new(vertices: Vec<V>) -> Self {
        Self {
            graph: Graph {
                vertices,
                edges: vec![],
            },
        }
    }
}

impl<V> fmt::Display for UnweightedGraph<V>
where
    V: Clone + fmt::Display + fmt::Debug + PartialEq,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.graph)
    }
}

fn main() {
    let mut ug = UnweightedGraph::new(vec!["Atlanta", "Boston", "Chicago"]);

    println!("{ug}");
}
