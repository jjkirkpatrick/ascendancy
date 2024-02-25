use crate::player_interactions::selection::Selection;
use crate::solar_system::SolarSystem;
use crate::structures::stargate::Stargate;
use bevy::prelude::*;
use petgraph::algo::astar;
use petgraph::graph::{EdgeIndex, Graph, NodeIndex};
use rand::prelude::IteratorRandom;
use std::collections::HashMap;

/// a Graph representing the solar systems and their connections
#[derive(Resource, Default)]
pub struct SystemGraph {
    /// The graph of solar systems and their connections
    graph: Graph<SolarSystem, Stargate>,
    /// A mapping from a system id to a node index
    system_to_node: HashMap<u32, NodeIndex>, // mapping from SolarSystem id to NodeIndex
}

/// Errors that can occur when using the `SystemGraph`
#[derive(Debug, PartialEq)]
pub enum GraphError {
    /// The system was not found in the graph
    SystemNotFound,
    /// No path was found between the two systems
    NoPath,
}

impl SystemGraph {
    /// Create a new `SystemGraph`
    pub fn new() -> Self {
        Self {
            graph: Graph::<SolarSystem, Stargate>::new(),
            system_to_node: HashMap::new(),
        }
    }

    /// Add a node to the graph, i.e a solar system
    pub fn add_node(&mut self, system: SolarSystem) -> NodeIndex {
        let index = self.graph.add_node(system.clone());
        self.system_to_node.insert(system.attributes.id, index); // save the mapping

        index
    }

    /// Add an edge to the graph, i.e a connection between two systems
    pub fn add_edge(
        &mut self,
        system_a: NodeIndex,
        system_b: NodeIndex,
        gate: Stargate,
    ) -> EdgeIndex {
        self.graph.add_edge(system_a, system_b, gate)
    }

    /// Get the path between two systems
    fn get_path(
        &self,
        system_a: NodeIndex,
        system_b: NodeIndex,
    ) -> Result<Vec<Stargate>, GraphError> {
        match astar(
            &self.graph,
            system_a,
            |finish| finish == system_b,
            |e| e.weight().distance,
            |_| 0,
        ) {
            Some((_, path_nodes)) => {
                // Convert path of NodeIndices to a path of Stargates
                let mut path_gates = Vec::new();
                for i in 0..(path_nodes.len() - 1) {
                    if let Some(edge) = self.graph.find_edge(path_nodes[i], path_nodes[i + 1]) {
                        let gate = self.graph.edge_weight(edge).unwrap().clone(); // Clone the Stargate object
                        path_gates.push(gate);
                    }
                }
                Ok(path_gates)
            }
            None => Err(GraphError::NoPath),
        }
    }

    /// Get the path between two systems
    pub fn get_pathfinding_between(
        &self,
        system_a: &SolarSystem,
        system_b: &SolarSystem,
    ) -> Result<Vec<Stargate>, GraphError> {
        let start_index = self
            .system_to_node
            .get(&system_a.attributes.id)
            .ok_or(GraphError::SystemNotFound)?;
        let end_index = self
            .system_to_node
            .get(&system_b.attributes.id)
            .ok_or(GraphError::SystemNotFound)?;

        self.get_path(*start_index, *end_index)
    }

    /// get the path between a known starting system and a random system
    /// retry up to a maximum of 10 times if a path is not found
    pub fn get_pathfinding_to_random_system(
        &self,
        system_a: &SolarSystem,
    ) -> Result<Vec<Stargate>, GraphError> {
        let start_index = self
            .system_to_node
            .get(&system_a.attributes.id)
            .ok_or(GraphError::SystemNotFound)?;

        let mut rng = rand::thread_rng();
        let mut path = Err(GraphError::NoPath);
        for _ in 0..10 {
            let end_index = *self
                .system_to_node
                .values()
                .choose(&mut rng)
                .ok_or(GraphError::SystemNotFound)?;

            path = self.get_path(*start_index, end_index);
            if path.is_ok() {
                break;
            }
        }
        path
    }

    /// Remove a node from the graph, i.e a solar system
    pub fn remove_node(&mut self, system: &SolarSystem) {
        if let Some(index) = self.system_to_node.remove(&system.attributes.id) {
            self.graph.remove_node(index);
        }
    }

    /// Remove an edge from the graph, i.e a connection between two systems
    pub fn remove_edge(&mut self, edge_index: EdgeIndex) {
        self.graph.remove_edge(edge_index);
    }

    /// Check if the graph contains a system
    pub fn contains_system(&self, system: &SolarSystem) -> bool {
        self.system_to_node.contains_key(&system.attributes.id)
    }

    /// Check if the graph contains a gate
    pub fn contains_gate(&self, edge_index: EdgeIndex) -> bool {
        self.graph.edge_weight(edge_index).is_some()
    }

    /// Get a system by its id
    pub fn system_by_id(&self, id: &u32) -> Option<&SolarSystem> {
        self.system_to_node
            .get(id)
            .and_then(|index| self.graph.node_weight(*index))
    }

    /// Get the `NodeIndex` of a system by its id
    pub fn node_by_id(&self, id: &u32) -> Option<&NodeIndex> {
        self.system_to_node.get(id)
    }

    /// Get a system by its `NodeIndex`
    pub fn system_by_node(&self, node: &NodeIndex) -> Option<&SolarSystem> {
        self.graph.node_weight(*node)
    }
}

/// get a path between two selected Systems
pub fn get_stargate_path_between_systems(
    selected_systems: Res<Selection>,
    system_graph: Res<SystemGraph>,
    star_gates: Query<(&Stargate, &GlobalTransform)>,
) {
    if selected_systems.count() != 2 {
        return;
    }

    if selected_systems.count() > 2 {
        return;
    }

    let system_a = selected_systems.get(0).unwrap();
    let system_b = selected_systems.get(1).unwrap();

    let path = system_graph.get_pathfinding_between(&system_a, &system_b);

    match path {
        Ok(gates) => {
            for gate in gates {
                let mut gate_transform: Option<GlobalTransform> = None;

                for (star_gates, global_transform) in star_gates.iter() {
                    if star_gates.id() == gate.id() {
                        gate_transform = Some(*global_transform);
                        break;
                    }
                }

                // Use gate_transform here
                if let Some(transform) = gate_transform {
                    // Do something with transform
                    println!("Gate transform: {:?}", transform);
                }
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    println!("End of path");
}
