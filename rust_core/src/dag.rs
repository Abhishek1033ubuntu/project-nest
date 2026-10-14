use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
pub struct DAG {
    pub nodes: HashSet<String>,
    pub adjacency: HashMap<String, Vec<String>>,
    pub in_degree: HashMap<String, usize>,
}

impl DAG {
    pub fn new() -> Self {
        DAG {
            nodes: HashSet::new(),
            adjacency: HashMap::new(),
            in_degree: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: &str) {
        if self.nodes.insert(node.to_string()) {
            self.adjacency.entry(node.to_string()).or_default();
            self.in_degree.entry(node.to_string()).or_insert(0);
        }
    }

    pub fn add_edge(&mut self, from: &str, to: &str) -> Result<(), String> {
        self.add_node(from);
        self.add_node(to);

        self.adjacency.get_mut(from).unwrap().push(to.to_string());
        *self.in_degree.get_mut(to).unwrap() += 1;

        if !self.is_acyclic() {
            self.adjacency.get_mut(from).unwrap().pop();
            *self.in_degree.get_mut(to).unwrap() -= 1;
            return Err(format!("Cycle detected when adding edge {} -> {}", from, to));
        }
        Ok(())
    }

    pub fn topological_sort(&self) -> Vec<String> {
        let mut in_degrees = self.in_degree.clone();
        let mut queue = VecDeque::new();
        let mut sorted = Vec::new();

        for (node, &deg) in &in_degrees {
            if deg == 0 {
                queue.push_back(node.clone());
            }
        }

        while let Some(node) = queue.pop_front() {
            sorted.push(node.clone());
            if let Some(neighbors) = self.adjacency.get(&node) {
                for neighbor in neighbors {
                    let deg = in_degrees.get_mut(neighbor).unwrap();
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }
        sorted
    }

    pub fn is_acyclic(&self) -> bool {
        self.topological_sort().len() == self.nodes.len()
    }
}
