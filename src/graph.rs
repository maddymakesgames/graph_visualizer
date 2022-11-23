use std::hash::Hash;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct NodeIndex(pub usize);

impl NodeIndex {
    pub fn index(&self) -> usize {
        self.0
    }

    pub fn next(&self) -> Self {
        NodeIndex(self.0 + 1)
    }
}

pub struct Graph {
    name: String,
    nodes: Vec<Node>,
    next_index: NodeIndex,
    is_directed: bool,
    is_weighted: bool,
}

impl Graph {
    pub fn new(name: String, directed: bool, weighted: bool) -> Self {
        Graph {
            name,
            nodes: Vec::new(),
            next_index: NodeIndex(0),
            is_directed: directed,
            is_weighted: weighted,
        }
    }

    pub fn add_node(
        &mut self,
        pos: (f32, f32),
        name: String,
        connections: Vec<(NodeIndex, Option<f32>)>,
    ) {
        for (idx, weight) in &connections {
            let n = self.nodes.get_mut(idx.index()).unwrap();
            n.add_edge(self.next_index, *weight)
        }

        self.nodes
            .push(Node::new(self.next_index, pos, name, connections));

        self.next_index = self.next_index.next();
    }

    pub fn add_edge(&mut self, a: NodeIndex, b: NodeIndex, weight: Option<f32>) {
        if a == b {
            return;
        }

        self.nodes[a.index()].add_edge(b, weight);
        if !self.is_directed {
            self.nodes[b.index()].add_edge(a, weight);
        }
    }

    pub fn remove_edge(&mut self, e: Edge) {
        let (a, b) = e.get_nodes();
        self.nodes[a.index()].remove_edge(b);

        if !self.is_directed {
            self.nodes[b.index()].remove_edge(a);
        }
    }

    pub fn get_node(&self, idx: NodeIndex) -> &Node {
        // we assume the idx is always correct
        // the newtype should ensure that the indices shold always come from valid places
        self.nodes.get(idx.index()).unwrap()
    }

    pub fn try_get_node(&self, idx: NodeIndex) -> Option<&Node> {
        self.nodes.get(idx.index())
    }

    pub fn try_get_node_mut(&mut self, idx: NodeIndex) -> Option<&mut Node> {
        self.nodes.get_mut(idx.index())
    }

    pub fn get_nodes(&self) -> &[Node] {
        &self.nodes
    }

    pub fn get_nodes_mut(&mut self) -> &mut [Node] {
        self.nodes.as_mut_slice()
    }

    pub fn get_node_mut(&mut self, idx: NodeIndex) -> &mut Node {
        self.nodes.get_mut(idx.index()).unwrap()
    }

    pub fn reset(&mut self) {
        self.nodes.iter_mut().for_each(|n| n.reset())
    }

    pub fn is_directed(&self) -> bool {
        self.is_directed
    }

    pub fn is_weighted(&self) -> bool {
        self.is_weighted
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_connections(&self, idx: NodeIndex) -> Vec<NodeIndex> {
        if self.is_directed {
            let mut in_bound: Vec<NodeIndex> = self
                .nodes
                .iter()
                .filter(|n| n.get_neighbors().contains(&idx))
                .map(|n| n.id)
                .collect();

            in_bound.extend(self.get_node(idx).get_neighbors());

            in_bound
        } else {
            // If we don't have a directed graph a node's neighbors will be all the connections
            self.get_node(idx).get_neighbors()
        }
    }
}

#[derive(Copy, Clone)]
pub enum NodeState {
    None,
    Start,
    Seen,
    Visited,
    End,
}

pub struct Node {
    id: NodeIndex,
    pos: (f32, f32),
    name: String,
    edges: Vec<Edge>,
    from_node: Option<NodeIndex>,
    curr_path: Option<f32>,
    state: NodeState,
}

impl Node {
    pub fn new(
        id: NodeIndex,
        pos: (f32, f32),
        name: String,
        connections: Vec<(NodeIndex, Option<f32>)>,
    ) -> Self {
        Self {
            pos,
            name,
            state: NodeState::None,
            id,
            edges: connections
                .iter()
                .map(|(idx, w)| Edge::new(id, *idx, *w))
                .collect(),
            from_node: None,
            curr_path: None,
        }
    }

    pub fn get_id(&self) -> NodeIndex {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_neighbors(&self) -> Vec<NodeIndex> {
        self.edges
            .iter()
            .map(|e| {
                let (a, b) = e.get_nodes();
                if a == self.id {
                    b
                } else {
                    a
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn get_edges(&self) -> Vec<Edge> {
        self.edges.clone()
    }

    pub fn view(&mut self) {
        self.state = NodeState::Seen;
    }

    pub fn visit(&mut self) {
        self.state = NodeState::Visited;
    }

    pub fn end(&mut self) {
        self.state = NodeState::End;
    }

    pub fn reset(&mut self) {
        self.state = NodeState::None;
        self.curr_path = None;
        self.from_node = None;
    }

    pub fn get_state(&self) -> NodeState {
        self.state
    }

    pub fn add_edge(&mut self, other: NodeIndex, weight: Option<f32>) {
        self.edges.push(Edge::new(self.id, other, weight))
    }

    pub fn get_curr_path(&self) -> f32 {
        self.curr_path.unwrap_or(f32::NAN)
    }

    pub fn get_pos(&self) -> (f32, f32) {
        self.pos
    }

    pub fn set_last_node(&mut self, index: NodeIndex, len: f32) {
        self.curr_path = Some(len);
        self.from_node = Some(index);
    }

    pub fn get_last_node(&self) -> Option<NodeIndex> {
        self.from_node
    }

    pub fn get_pos_mut(&mut self) -> &mut (f32, f32) {
        &mut self.pos
    }

    pub fn remove_edge(&mut self, other: NodeIndex) {
        self.edges.retain(|e| {
            let (a, b) = e.get_nodes();
            a != other && b != other
        })
    }

    pub fn start(&mut self) {
        self.state = NodeState::Start;
    }
}

#[derive(Clone, Copy)]
pub struct Edge(f32, NodeIndex, NodeIndex);

impl Edge {
    pub fn new(n1: NodeIndex, n2: NodeIndex, weight: Option<f32>) -> Self {
        Self(weight.unwrap_or(1.0), n1, n2)
    }
    pub fn get_nodes(&self) -> (NodeIndex, NodeIndex) {
        (self.1, self.2)
    }

    pub fn get_weighted_nodes(&self) -> (f32, NodeIndex, NodeIndex) {
        (self.0, self.1, self.2)
    }

    pub fn get_weight(&self) -> f32 {
        self.0
    }
}
