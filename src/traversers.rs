use std::collections::HashSet;

use egui::Pos2;
use instant::Instant;

use crate::graph::{Graph, NodeIndex};

pub struct TraversalManager {
    pub last_traversal: Instant,
    pub speed: u32,
    pub auto: bool,
    pub currently_traversing: bool,
    pub alg: GraphTraversers,
    pub traversal: Option<TraversalData>,
}

impl TraversalManager {
    pub fn new(alg: GraphTraversers) -> Self {
        TraversalManager {
            alg,
            ..Default::default()
        }
    }

    pub fn new_traversal(&mut self, start_node: NodeIndex, end_node: NodeIndex) {
        self.currently_traversing = true;
        self.traversal = Some(TraversalData::new(start_node, end_node))
    }

    pub fn update(&mut self, graph: &mut Graph) {
        if self.currently_traversing {
            if self.auto {
                let now = Instant::now();
                if let Some(dur) = now.checked_duration_since(self.last_traversal) {
                    if dur.as_millis() as u32 >= self.speed {
                        if self.step(graph) {
                            self.currently_traversing = false;
                        }
                        self.last_traversal = now;
                    }
                }
            } else if self.step(graph) {
                self.currently_traversing = false;
            }
        }
    }

    pub fn stop_traversal(&mut self) {
        self.traversal = None;
    }

    fn step(&mut self, graph: &mut Graph) -> bool {
        if let Some(traversal) = &mut self.traversal {
            match self.alg {
                GraphTraversers::BreadthFirst => traversal.breadth_first_step(graph),
                GraphTraversers::DepthFirst => traversal.depth_first_step(graph),
                GraphTraversers::Dijkstras => traversal.dijkstras_step(graph),
                GraphTraversers::AStar => traversal.astar_step(graph),
                GraphTraversers::SimpleBreadth => traversal.simple_breadth(graph),
            }
        } else {
            true
        }
    }
}

impl Default for TraversalManager {
    fn default() -> Self {
        TraversalManager {
            last_traversal: Instant::now(),
            speed: 30,
            auto: true,
            currently_traversing: false,
            alg: GraphTraversers::DepthFirst,
            traversal: None,
        }
    }
}

pub struct TraversalData {
    pub end_node: NodeIndex,
    pub start_node: NodeIndex,
    pub to_traverse: Vec<(f32, NodeIndex)>,
    pub visited: HashSet<NodeIndex>,
}

impl TraversalData {
    pub fn breadth_first_step(&mut self, graph: &mut Graph) -> bool {
        let (_curr_length, idx) = self.to_traverse.remove(0);
        let node = graph.get_node_mut(idx);

        if idx == self.start_node {
            node.start();
        } else {
            node.visit();
        }

        self.visited.insert(idx);

        let id = node.get_id();
        let path_len = node.get_curr_path();

        if id == self.end_node {
            node.end();
            return self.to_traverse.is_empty();
        }

        let neighbors = node
            .get_neighbors()
            .iter()
            .filter_map(|n| {
                if self.visited.contains(n) || self.to_traverse.iter().any(|(_, n1)| n == n1) {
                    None
                } else {
                    let node_to_visit = graph.get_node_mut(*n);
                    node_to_visit.view();
                    node_to_visit.set_last_node(id, path_len + 1.0);

                    Some((0.0, *n))
                }
            })
            .collect::<Vec<_>>();

        self.to_traverse.extend(neighbors.iter());

        self.to_traverse.is_empty()
    }

    pub fn depth_first_step(&mut self, graph: &mut Graph) -> bool {
        let (_priority, idx) = self.to_traverse.remove(0);
        let node = graph.get_node_mut(idx);

        if idx == self.start_node {
            node.start();
        } else {
            node.visit();
        }

        self.visited.insert(idx);
        let path_len = node.get_curr_path();

        let id = node.get_id();

        if id == self.end_node {
            node.end();
            return self.to_traverse.is_empty();
        }

        let mut neighbors = node
            .get_neighbors()
            .iter()
            .filter_map(|n| {
                if self.visited.contains(n) || self.to_traverse.iter().any(|(_, n1)| n == n1) {
                    None
                } else {
                    let node_to_visit = graph.get_node_mut(*n);
                    node_to_visit.view();
                    node_to_visit.set_last_node(id, path_len + 1.0);

                    Some((0.0, *n))
                }
            })
            .collect::<Vec<_>>();

        neighbors.extend(&self.to_traverse);

        self.to_traverse = neighbors;

        self.to_traverse.is_empty()
    }

    pub fn dijkstras_step(&mut self, graph: &mut Graph) -> bool {
        let (priority, idx) = self.to_traverse.remove(0);
        let node = graph.get_node_mut(idx);

        if self.visited.contains(&idx) {
            return self.to_traverse.is_empty();
        }

        if idx == self.start_node {
            node.start();
        } else {
            node.visit();
        }

        self.visited.insert(idx);

        let path_len = node.get_curr_path();

        if idx == self.end_node {
            node.end();
            return self.to_traverse.is_empty();
        }

        let neighbors = node
            .get_edges()
            .iter()
            .filter_map(|e| {
                let (weight, a, b) = e.get_weighted_nodes();
                if a != idx {
                    if self.visited.contains(&a) {
                        None
                    } else {
                        let node_to_visit = graph.get_node_mut(a);

                        node_to_visit.view();
                        if node_to_visit.get_curr_path() > path_len + weight
                            || node_to_visit.get_last_node().is_none()
                        {
                            node_to_visit.set_last_node(idx, path_len + weight);
                        }

                        Some((priority + weight, a))
                    }
                } else if self.visited.contains(&b) {
                    None
                } else {
                    let node_to_visit = graph.get_node_mut(b);

                    node_to_visit.view();
                    if node_to_visit.get_curr_path() > path_len + weight
                        || node_to_visit.get_last_node().is_none()
                    {
                        node_to_visit.set_last_node(idx, path_len + weight);
                    }

                    Some((priority + weight, b))
                }
            })
            .collect::<Vec<_>>();

        self.to_traverse.extend(neighbors.iter());

        self.to_traverse
            .sort_by(|(p1, _), (p2, _)| p1.partial_cmp(p2).unwrap());

        self.to_traverse.is_empty()
    }

    pub fn astar_step(&mut self, graph: &mut Graph) -> bool {
        let (priority, idx) = self.to_traverse.remove(0);
        let node = graph.get_node_mut(idx);

        if self.visited.contains(&idx) {
            return self.to_traverse.is_empty();
        }

        if idx == self.start_node {
            node.start();
        } else {
            node.visit();
        }

        self.visited.insert(idx);

        let path_len = node.get_curr_path();

        if idx == self.end_node {
            node.end();
            return self.to_traverse.is_empty();
        }

        let node_pos = Pos2::from(node.get_pos());

        let neighbors = node
            .get_edges()
            .iter()
            .filter_map(|e| {
                let (weight, a, b) = e.get_weighted_nodes();
                if a != idx {
                    if self.visited.contains(&a) {
                        None
                    } else {
                        let node_to_visit = graph.get_node_mut(a);

                        let distance = node_pos.distance(node_to_visit.get_pos().into());

                        node_to_visit.view();
                        if node_to_visit.get_curr_path() > path_len + weight
                            || node_to_visit.get_last_node().is_none()
                        {
                            node_to_visit.set_last_node(idx, path_len + weight);
                        }

                        Some((priority + distance + weight, a))
                    }
                } else if self.visited.contains(&b) {
                    None
                } else {
                    let node_to_visit = graph.get_node_mut(b);

                    let distance = node_pos.distance(node_to_visit.get_pos().into());

                    node_to_visit.view();
                    if node_to_visit.get_curr_path() > path_len + weight
                        || node_to_visit.get_last_node().is_none()
                    {
                        node_to_visit.set_last_node(idx, path_len + weight);
                    }

                    Some((priority + distance + weight, b))
                }
            })
            .collect::<Vec<_>>();

        self.to_traverse.extend(neighbors.iter());

        self.to_traverse
            .sort_by(|(p1, _), (p2, _)| p1.partial_cmp(p2).unwrap());

        self.to_traverse.is_empty()
    }

    pub fn new(start_node: NodeIndex, end_node: NodeIndex) -> TraversalData {
        TraversalData {
            start_node,
            end_node,
            to_traverse: vec![(0.0, start_node)],
            visited: HashSet::new(),
        }
    }

    /// Traverse the graph using breadth first search while treating it as an undirected graph
    ///
    /// Used internally to do connected tests on directed graphs
    fn simple_breadth(&mut self, graph: &mut Graph) -> bool {
        let (_curr_length, idx) = self.to_traverse.remove(0);
        let node = graph.get_node_mut(idx);

        node.visit();
        self.visited.insert(idx);

        let id = node.get_id();
        let path_len = node.get_curr_path();

        if id == self.end_node {
            node.end();
            return self.to_traverse.is_empty();
        }

        // We loop over the connections to the node, not caring if they are in-bound or out-bound
        let neighbors = graph
            .get_connections(idx)
            .iter()
            .filter_map(|n| {
                if self.visited.contains(n) || self.to_traverse.iter().any(|(_, n1)| n == n1) {
                    None
                } else {
                    let node_to_visit = graph.get_node_mut(*n);
                    node_to_visit.view();
                    node_to_visit.set_last_node(id, path_len + 1.0);

                    Some((0.0, *n))
                }
            })
            .collect::<Vec<_>>();

        self.to_traverse.extend(neighbors.iter());

        self.to_traverse.is_empty()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GraphTraversers {
    DepthFirst,
    BreadthFirst,
    Dijkstras,
    AStar,
    SimpleBreadth,
}

impl GraphTraversers {
    pub const fn name(&self) -> &'static str {
        match self {
            GraphTraversers::DepthFirst => "Depth First Search",
            GraphTraversers::BreadthFirst => "Breadth First Search",
            GraphTraversers::Dijkstras => "Dijkstra's Shortest Path",
            GraphTraversers::AStar => "A*",
            GraphTraversers::SimpleBreadth => "Simple Breadth First Search",
        }
    }

    pub const fn values() -> [GraphTraversers; 4] {
        [
            GraphTraversers::DepthFirst,
            GraphTraversers::BreadthFirst,
            GraphTraversers::Dijkstras,
            GraphTraversers::AStar,
        ]
    }
}
