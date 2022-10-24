use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt,
    hash::Hash,
};

use ordered_float::OrderedFloat;


pub fn linear_contains<T>(xs: &[T], key: T) -> bool
where
    T: Ord,
{
    for x in xs {
        if *x == key {
            return true;
        }
    }
    false
}

fn is_sorted<T>(xs: &[T]) -> bool
where
    T: Clone + Ord,
{
    let mut sorted = Vec::from(xs);

    sorted.sort();

    sorted == xs
}

pub fn binary_contains<T>(xs: &[T], key: T) -> Result<bool, &'static str>
where
    T: Clone + Ord,
{
    if !is_sorted(xs) {
        return Err("container must be sorted first");
    }

    let mut low = 0;
    let mut high = xs.len() - 1;

    while low <= high {
        let middle = (low + high) / 2;

        if let Some(x) = xs.get(middle) {
            use std::cmp::Ordering::*;

            match x.cmp(&key) {
                Less => {
                    low = middle + 1;
                }
                Greater => {
                    high = middle - 1;
                }
                Equal => {
                    return Ok(true);
                }
            }
        }
    }
    return Ok(false);
}

#[derive(Clone, Debug)]
pub struct Node<T>
where
    T: Clone + fmt::Debug + Default + PartialEq,
{
    state: T,
    parent: Option<Box<Node<T>>>,
    cost: OrderedFloat<f64>,
    heuristic: OrderedFloat<f64>,
}

impl<T> Default for Node<T>
where
    T: Clone + fmt::Debug + Default + PartialEq,
{
    fn default() -> Self {
        Self {
            state: Default::default(),
            parent: None,
            cost: Default::default(),
            heuristic: Default::default(),
        }
    }
}

/*
Rust's equivalent of Java's `Comparable` interface is more or less the combination of Rust's `PartialEq`, `Eq`,
`PartialOrd`, and possibly `Ord` traits.

Note that we are comparing ONLY the cost and heuristic / estimated cost of the nodes.
If you `#[derive]` these traits, you will see strange results because the comparison will use ALL of the node's fields.
*/

impl<T> PartialEq for Node<T>
where
    T: Clone + fmt::Debug + Default + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        let my_cost = self.cost + self.heuristic;
        let others_cost = other.cost + other.heuristic;

        my_cost == others_cost
    }
}

impl<T> Eq for Node<T> where T: Clone + fmt::Debug + Default + PartialEq {}

impl<T> PartialOrd for Node<T>
where
    T: Clone + fmt::Debug + Default + PartialEq,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let my_cost = self.cost + self.heuristic;
        let others_cost = other.cost + other.heuristic;

        Some(my_cost.cmp(&others_cost))
    }
}

impl<T> Ord for Node<T>
where
    T: Clone + fmt::Debug + Default + PartialEq + Eq + PartialOrd,
{
    fn cmp(&self, other: &Self) -> Ordering {
        let my_cost = self.cost + self.heuristic;
        let others_cost = other.cost + other.heuristic;

        my_cost.cmp(&others_cost)
    }
}

impl<T> Iterator for Node<T> where T: Clone + fmt::Debug + Default + PartialEq {
    type Item = Box<Node<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parent.clone()
    }
}

impl<T> Node<T>
where
    T: Clone + fmt::Debug + Default + PartialEq,
{
    pub fn new(state: T, parent: Option<Box<Node<T>>>) -> Self {
        Self {
            state,
            parent,
            ..Default::default()
        }
    }

    pub fn new_with_cost_and_heuristic(
        state: T,
        parent: Option<Box<Node<T>>>,
        cost: OrderedFloat<f64>,
        heuristic: OrderedFloat<f64>,
    ) -> Self {
        Self {
            state,
            parent,
            cost,
            heuristic,
        }
    }

    /// Traverse this node's parents, transforming them into a list of nodes
    /// that you can then iterate.
    /// (Deriving `Iterator` would also work)
    pub fn to_node_path(&self) -> VecDeque<Node<T>> {
        let mut path = VecDeque::<Node<T>>::new();

        path.push_front(self.clone());

        let mut current_node = self.clone();

        while let Some(node) = current_node.parent {
            current_node = *node.clone();
            path.push_front(*node);
        }

        path
    }

    pub fn to_path(&self) -> VecDeque<T> {
        let node_path = self.to_node_path();

        node_path.iter().map(|node| node.clone().state).collect()
    }

    pub fn get_total_cost_of_path(&self) -> OrderedFloat<f64> {
        let node_path = self.to_node_path();

        node_path.iter().fold(OrderedFloat(0.0), |acc, node| acc + node.cost)
    }
}

/// # Depth First Search
///
/// Use `goal_test_fn` to search for a value in a given structure,
/// given an `initial` state and `get_successors_fn` that describes how to reach a value's neighbours.
///
/// Returns `Some` if the goal is found, and `None` otherwise. You can traverse the returned `Node`'s `parent`s to get
/// the path the algorithm took.
///
pub fn dfs<T, PredicateFn, SuccessorsFn>(
    initial: T,
    goal_test_fn: PredicateFn,
    get_successors_fn: SuccessorsFn,
) -> Option<Node<T>>
where
    T: Clone + fmt::Debug + Default + Eq + Hash + PartialEq,
    PredicateFn: Fn(T) -> bool,
    SuccessorsFn: Fn(T) -> Vec<T>,
{
    let mut frontier = Vec::<Node<T>>::new();

    frontier.push(Node::<T>::new(initial.clone(), None));

    let mut explored = HashSet::<T>::new();

    explored.insert(initial);

    while !frontier.is_empty() {
        let current_node = frontier.pop();

        //println!("[DFS] Current node: {:?}", (current_node.clone()?).state);

        if let Some(current_node) = current_node {
            let current_state = &current_node.state;

            if goal_test_fn(current_state.clone()) {
                return Some(current_node);
            }

            for succ in get_successors_fn(current_state.clone()) {
                if !explored.insert(succ.clone()) {
                    //println!("[DFS] I already explored {:?}", succ);
                    continue;
                }

                //println!("[DFS] Pushing {:?}", succ);

                frontier.push(Node::<T>::new(succ, Some(Box::new(current_node.clone()))));
            }
        }
    }

    None
}

/// # Breadth First Search
///
/// Use `goal_test_fn` to search for a value in a given structure,
/// given an `initial` state and `get_successors_fn` that describes how to reach a value's neighbours.
///
/// Returns `Some` if the goal is found, and `None` otherwise. You can traverse the returned `Node`'s `parent`s to get
/// the path the algorithm took.
///
pub fn bfs<T, PredicateFn, SuccessorsFn>(
    initial: T,
    goal_test_fn: PredicateFn,
    get_successors_fn: SuccessorsFn,
) -> Option<Node<T>>
where
    T: Clone + fmt::Debug + Default + Eq + Hash + PartialEq,
    PredicateFn: Fn(T) -> bool,
    SuccessorsFn: Fn(T) -> Vec<T>,
{
    let mut frontier = VecDeque::<Node<T>>::new();

    frontier.push_back(Node::<T>::new(initial.clone(), None));

    let mut explored = HashSet::<T>::new();

    explored.insert(initial);

    while !frontier.is_empty() {
        let current_node = frontier.pop_front();

        if let Some(current_node) = current_node {
            let current_state = current_node.clone().state;

            if goal_test_fn(current_state.clone()) {
                return Some(current_node);
            }

            for succ in get_successors_fn(current_state) {
                if explored.contains(&succ) {
                    continue;
                }

                explored.insert(succ.clone());

                frontier.push_back(Node::<T>::new(succ, Some(Box::new(current_node.clone()))));
            }
        }
    }

    None
}

/// # A-Star Search
///
/// Use `goal_test_fn` to search for a value in a given structure,
/// given an `initial` state and movement cost and `get_successors_fn` that describes how to reach a value's neighbours,
/// and a `heuristic_fn` that describes the estimated cost of moving from one node to the goal node.
///
/// Returns `Some` if the goal is found, and `None` otherwise. You can traverse the returned `Node`'s `parent`s to get
/// the path the algorithm took.
///
pub fn astar<T, PredicateFn, SuccessorsFn, HeuristicFn>(
    initial: T,
    goal_test_fn: PredicateFn,
    get_successors_fn: SuccessorsFn,
    heuristic_fn: HeuristicFn,
) -> Option<Node<T>>
where
    T: Clone + fmt::Debug + Default + Hash + Ord,
    PredicateFn: Fn(T) -> bool,
    SuccessorsFn: Fn(T) -> Vec<T>,
    HeuristicFn: Fn(T) -> OrderedFloat<f64>,
{
    // Note that this uses `std::cmp::Reverse` to create a min-heap,
    // so that it sorts nodes by cost in ascending order, as described by `Node`'s `PartialOrd` and `Ord` traits.
    // The default is descending order, so the algorithm may appear to choose the *worst* possible path,
    // which is obviously not what we want in a proper A* implementation!
    let mut frontier = BinaryHeap::<Reverse<Node<T>>>::new();

    frontier.push(Reverse(Node::<T>::new_with_cost_and_heuristic(
        initial.clone(),
        None,
        OrderedFloat(0.0),
        heuristic_fn(initial.clone()),
    )));

    let mut explored = HashMap::<T, OrderedFloat<f64>>::new();

    explored.insert(initial, OrderedFloat(0.0));

    while !frontier.is_empty() {
        let current_node = frontier.pop();

        //println!("Current node: {:?}", current_node);

        if let Some(current_node) = current_node {
            let current_state = current_node.clone().0.state;

            //println!("I am at {:?}", current_state);

            if goal_test_fn(current_state.clone()) {
                return Some(current_node.0);
            }

            for succ in get_successors_fn(current_state.clone()) {
                // Since we are using a regular grid with no cost to move from one tile to another,
                // an added cost of 1 is fine.
                // If there are obstructions between nodes, a `cost_fn` would make more sense here.
                // For instance, if there was a patch of rough terrain between two nodes, that move
                // would be more expensive than if the path was clear.
                // Or, for another example, if we were making a map where our nodes represent places on that map,
                // we would want to consider traffic and delays on the route here, which can make an otherwise optimal
                // route more expensive.
                let new_cost = current_node.0.cost + 1.0;

                // println!(
                //     "[A*] Cost for current node: {:?}, {:?}",
                //     current_node.0.cost, current_node.0.state
                // );
                // println!("[A*] Cost for next node: {:?}, {:?}", new_cost, succ);

                let old_cost = *explored.get(&succ).unwrap_or(&OrderedFloat(0.0));

                //println!("[A*] Old cost: {:?}", old_cost);

                // If we have not explored this location yet,
                // OR if we have AND it has a lower cost, push the node
                if !explored.contains_key(&succ) || (old_cost != 0.0 && old_cost > new_cost) {
                    explored.insert(succ.clone(), new_cost);

                    // println!(
                    //     "[A*] I just explored {:?}, cost {:?}. old cost {:?}",
                    //     succ, new_cost, old_cost
                    // );

                    frontier.push(Reverse(Node::<T>::new_with_cost_and_heuristic(
                        succ.clone(),
                        Some(Box::new(current_node.0.clone())),
                        new_cost,
                        heuristic_fn(succ.clone()),
                    )));
                }

                //println!("======================");
            }
        }
    }

    None
}
