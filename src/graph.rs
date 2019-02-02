use std::collections::{HashMap, HashSet};
use std::vec::Vec;

type VertexDescriptor = u32;

pub struct AdjacencyList {
    vertices: HashMap<VertexDescriptor, HashSet<VertexDescriptor>>,
    next_id: VertexDescriptor,
}

impl AdjacencyList {
    pub fn new() -> AdjacencyList {
        AdjacencyList { vertices: HashMap::new(), next_id: 0 }
    }

    pub fn add_vertex(&mut self) -> VertexDescriptor {
        let id = self.next_id;
        println!("adding vertex {}", id);
        self.vertices.insert(id, HashSet::new());
        self.next_id += 1;
        id
    }

    pub fn add_edge(&mut self, from: VertexDescriptor, to: VertexDescriptor) {
        // check if the edge already exists
        let edge_set = self.vertices.get_mut(&from);
        if let Some(e) = edge_set {
            println!("adding edge from {} to {}", from, to);
            e.insert(to);
        }
    }

    pub fn adjacent(&self, from: VertexDescriptor, to: VertexDescriptor) -> bool {
        let edge_set = self.vertices.get(&from).expect(&format!("vertex {} not found", from));
        edge_set.contains(&to)
    }

    pub fn neighbours(&self, from: VertexDescriptor) -> Vec<VertexDescriptor> {
        let mut neighbours: Vec<VertexDescriptor> = Vec::new();
        let edge_set = self.vertices.get(&from).expect(&format!("vertex {} not found", from));
        for &vertex in edge_set {
            neighbours.push(vertex);
        }
        neighbours
    }
}
