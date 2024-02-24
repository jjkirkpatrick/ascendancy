use crate::player_interactions::selection::Selection;
use crate::solar_system::attributes::SystemAttributes;
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
    graph: Graph<SystemAttributes, Stargate>,
    /// A mapping from a system id to a node index
    system_to_node: HashMap<u16, NodeIndex>, // mapping from SystemAttributes id to NodeIndex
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
            graph: Graph::<SystemAttributes, Stargate>::new(),
            system_to_node: HashMap::new(),
        }
    }

    /// Add a node to the graph, i.e a solar system
    pub fn add_node(&mut self, system: SystemAttributes) -> NodeIndex {
        let index = self.graph.add_node(system.clone());
        self.system_to_node.insert(system.id, index); // save the mapping

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
        system_a: &SystemAttributes,
        system_b: &SystemAttributes,
    ) -> Result<Vec<Stargate>, GraphError> {
        let start_index = self
            .system_to_node
            .get(&system_a.id)
            .ok_or(GraphError::SystemNotFound)?;
        let end_index = self
            .system_to_node
            .get(&system_b.id)
            .ok_or(GraphError::SystemNotFound)?;

        self.get_path(*start_index, *end_index)
    }

    /// get the path between a known starting system and a random system
    /// retry up to a maximum of 10 times if a path is not found
    pub fn get_pathfinding_to_random_system(
        &self,
        system_a: &SystemAttributes,
    ) -> Result<Vec<Stargate>, GraphError> {
        let start_index = self
            .system_to_node
            .get(&system_a.id)
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
    pub fn remove_node(&mut self, system: &SystemAttributes) {
        if let Some(index) = self.system_to_node.remove(&system.id) {
            self.graph.remove_node(index);
        }
    }

    /// Remove an edge from the graph, i.e a connection between two systems
    pub fn remove_edge(&mut self, edge_index: EdgeIndex) {
        self.graph.remove_edge(edge_index);
    }

    /// Check if the graph contains a system
    pub fn contains_system(&self, system: &SystemAttributes) -> bool {
        self.system_to_node.contains_key(&system.id)
    }

    /// Check if the graph contains a gate
    pub fn contains_gate(&self, edge_index: EdgeIndex) -> bool {
        self.graph.edge_weight(edge_index).is_some()
    }

    /// Get a system by its id
    pub fn system_by_id(&self, id: &u16) -> Option<&SystemAttributes> {
        self.system_to_node
            .get(id)
            .and_then(|index| self.graph.node_weight(*index))
    }

    /// Get the `NodeIndex` of a system by its id
    pub fn node_by_id(&self, id: &u16) -> Option<&NodeIndex> {
        self.system_to_node.get(id)
    }

    /// Get a system by its `NodeIndex`
    pub fn system_by_node(&self, node: &NodeIndex) -> Option<&SystemAttributes> {
        self.graph.node_weight(*node)
    }
}

/// get a path between two selected Systems
pub fn get_Stargate_path_between_systems(
    selected_systems: Res<Selection>,
    gizmos: Gizmos,
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
            let previous_gate_transform: Option<GlobalTransform> = None;
            for gate in gates {
                let mut gate_transform: Option<GlobalTransform> = None;

                for (star_gates, global_transform) in star_gates.iter() {
                    if star_gates.id() == gate.id() {
                        gate_transform = Some(*global_transform);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    println!("End of path");
}

#[cfg(test)]
mod tests {
    use crate::faction::attributes::FactionID;
    use std::time::Instant;

    use super::*;

    fn generate_test_system(id: u16, name: &str) -> SystemAttributes {
        SystemAttributes {
            id: id,
            name: name.to_string(),
            owner: FactionID { id: 0 },
        }
    }

    #[test]
    fn test_add_node() {
        let mut graph = SystemGraph::new();
        let system = generate_test_system(0, "TestSystem");

        graph.add_node(system.clone());

        assert!(graph.contains_system(&system));
    }

    #[test]
    fn test_add_edge() {
        let mut graph = SystemGraph::new();
        let system_a = generate_test_system(0, "SystemA");
        let system_b = generate_test_system(1, "SystemB");

        let node_a = graph.add_node(system_a.clone());
        let node_b = graph.add_node(system_b.clone());

        let gate = Stargate {
            id: 0,
            name: "TestGate".to_string(),
            distance: 5,
            destination_gate_id: 1, // Use the dummy function
            destination_system_id: 0,
            origin_system_id: 0,
            is_active: true,
        };

        let edge = graph.add_edge(node_a, node_b, gate);

        assert!(graph.contains_gate(edge));
    }

    #[test]
    fn test_get_pathfinding_between() {
        let mut graph = SystemGraph::new();
        let system_a = generate_test_system(0, "SystemA");
        let system_b = generate_test_system(1, "SystemB");

        let node_a = graph.add_node(system_a.clone());
        let node_b = graph.add_node(system_b.clone());

        let gate1 = Stargate {
            id: 0,
            name: "TestGate".to_string(),
            distance: 5,
            destination_gate_id: 1,
            origin_system_id: 0,
            destination_system_id: 1,
            is_active: true,
        };

        let gate2 = Stargate {
            id: 1,
            name: "TestGate".to_string(),
            distance: 5,
            destination_gate_id: 0,
            origin_system_id: 1,
            destination_system_id: 0,
            is_active: true,
        };

        graph.add_edge(node_a, node_b, gate1.clone());
        graph.add_edge(node_b, node_a, gate2.clone());

        let path_result = graph.get_pathfinding_between(&system_a, &system_b);

        assert!(path_result.is_ok());
        assert_eq!(path_result.unwrap(), vec![gate1]); // Only gate1 is needed to travel from SystemA to SystemB
    }

    #[test]
    fn test_large_system_network() {
        // Setup
        let mut graph = SystemGraph::new();

        // Create 20 solar systems and add them to the graph
        let mut systems = Vec::new();
        for i in 0..20 {
            let system = SystemAttributes {
                id: i,
                name: format!("System {}", i),
                owner: FactionID { id: 0 },
            };
            systems.push(system.clone());
            graph.add_node(system);
        }

        // Connect each system to its previous and next one
        for i in 1..20 {
            let gate_a_to_b = Stargate {
                id: 1,
                name: "TestGate".to_string(),
                distance: 1,
                destination_gate_id: 0,   // these ID's arent correct
                origin_system_id: 0,      // these ID's arent correct
                destination_system_id: 1, // these ID's arent correct
                is_active: true,
            };
            let gate_b_to_a = Stargate {
                id: 1,
                name: "TestGate".to_string(),
                distance: 1,
                destination_gate_id: 0, // Use the dummy function// These IDs aren't correct
                origin_system_id: 1,    // These IDs aren't correct
                destination_system_id: 1, // These IDs aren't correct
                is_active: true,
            };

            // Add the system gates to the graph
            let system_a_index = *graph.system_to_node.get(&systems[i - 1].id).unwrap();
            let system_b_index = *graph.system_to_node.get(&systems[i].id).unwrap();
            graph.add_edge(system_a_index, system_b_index, gate_a_to_b);
            graph.add_edge(system_b_index, system_a_index, gate_b_to_a);
        }

        // Assert
        assert_eq!(graph.graph.node_count(), 20);
        assert_eq!(graph.graph.edge_count(), 38); // 19 * 2 = 38, two edges between each adjacent pair

        // Additional test: pathfinding between first and last system
        let path = graph.get_pathfinding_between(&systems[0], &systems[19]);

        assert!(path.is_ok());
        assert_eq!(path.unwrap().len(), 19); // 19 hops to reach the last system from the first one
    }

    #[test]
    fn test_remove_node() {
        let mut graph = SystemGraph::new();
        let system = generate_test_system(0, "TestSystem");

        graph.add_node(system.clone());
        graph.remove_node(&system);

        assert!(!graph.contains_system(&system));
    }

    #[test]
    fn test_remove_edge() {
        let mut graph = SystemGraph::new();
        let system_a = generate_test_system(0, "SystemA");
        let system_b = generate_test_system(1, "SystemB");

        let node_a = graph.add_node(system_a);
        let node_b = graph.add_node(system_b);

        let gate = Stargate {
            id: 0,
            name: "TestGate".to_string(),
            distance: 5,
            destination_gate_id: 1, // Use the dummy function
            origin_system_id: 0,
            destination_system_id: 0,
            is_active: true,
        };

        let edge = graph.add_edge(node_a, node_b, gate);
        graph.remove_edge(edge);

        assert!(!graph.contains_gate(edge));
    }

    #[test]
    fn test_system_by_id() {
        let mut graph = SystemGraph::new();
        let system = generate_test_system(0, "TestSystem");

        graph.add_node(system.clone());

        let fetched_system = graph.system_by_id(&system.id);

        assert_eq!(fetched_system, Some(&system));
    }

    #[test]
    fn test_multiple_edges_between_nodes() {
        let mut graph = SystemGraph::new();
        let system_a = generate_test_system(0, "SystemA");
        let system_b = generate_test_system(1, "SystemB");

        let node_a = graph.add_node(system_a);
        let node_b = graph.add_node(system_b);

        let gate1 = Stargate {
            id: 0,
            name: "TestGate".to_string(),
            distance: 5,
            destination_gate_id: 1,
            origin_system_id: 0,
            destination_system_id: 1,
            is_active: true,
        };
        let gate2 = Stargate {
            id: 1,
            name: "TestGate".to_string(),
            distance: 10,
            destination_gate_id: 0,
            origin_system_id: 1,
            destination_system_id: 0,
            is_active: true,
        };

        graph.add_edge(node_a, node_b, gate1);
        graph.add_edge(node_a, node_b, gate2);

        assert_eq!(graph.graph.edge_count(), 2);
    }

    #[test]
    fn test_cycles_in_graph() {
        let mut graph = SystemGraph::new();
        let system_a = generate_test_system(0, "SystemA");
        let system_b = generate_test_system(1, "SystemB");

        let node_a = graph.add_node(system_a.clone());
        let node_b = graph.add_node(system_b.clone());

        let gate1 = Stargate {
            id: 0,
            name: "TestGate".to_string(),
            distance: 5,
            destination_gate_id: 1,
            origin_system_id: 0,
            destination_system_id: 1,
            is_active: true,
        };
        let gate2 = Stargate {
            id: 1,
            name: "TestGate".to_string(),
            distance: 5,
            destination_gate_id: 0,
            origin_system_id: 1,
            destination_system_id: 0,
            is_active: true,
        };

        graph.add_edge(node_a, node_b, gate1);
        graph.add_edge(node_b, node_a, gate2);

        // Testing that a cycle doesn't affect pathfinding.
        let path_result = graph.get_pathfinding_between(&system_a, &system_b);
        assert!(path_result.is_ok());
    }

    #[test]
    fn test_disconnected_nodes() {
        let mut graph = SystemGraph::new();
        let system_a = generate_test_system(0, "SystemA");
        let system_b = generate_test_system(1, "SystemB");

        graph.add_node(system_a.clone());
        graph.add_node(system_b.clone());

        let path_result = graph.get_pathfinding_between(&system_a, &system_b);
        assert_eq!(path_result, Err(GraphError::NoPath));
    }

    #[test]
    fn test_single_node_pathfinding() {
        let mut graph = SystemGraph::new();
        let system_a = generate_test_system(0, "SystemA");

        graph.add_node(system_a.clone());

        let path_result = graph.get_pathfinding_between(&system_a, &system_a);
        assert!(path_result.is_ok());
        assert_eq!(path_result.unwrap(), vec![]); // No gates are needed to travel within the same system
    }

    #[test]
    fn test_remove_nonexistent_system() {
        let mut graph = SystemGraph::new();
        let system = generate_test_system(0, "TestSystem");

        // Try to remove a system that hasn't been added.
        graph.remove_node(&system);

        assert!(!graph.contains_system(&system));
        assert_eq!(graph.graph.node_count(), 0);
    }

    #[test]
    fn test_remove_nonexistent_gate() {
        let mut graph = SystemGraph::new();
        let edge = EdgeIndex::new(0);

        // Try to remove a gate that hasn't been added.
        graph.remove_edge(edge);

        assert!(!graph.contains_gate(edge));
        assert_eq!(graph.graph.edge_count(), 0);
    }

    #[test]
    fn test_add_duplicate_nodes() {
        let mut graph = SystemGraph::new();
        let system = generate_test_system(0, "TestSystem");

        let node_1 = graph.add_node(system.clone());
        let node_2 = graph.add_node(system.clone());

        // The two nodes should be distinct.
        assert_ne!(node_1, node_2);
        assert_eq!(graph.graph.node_count(), 2);
    }

    #[test]
    fn test_add_duplicate_edges() {
        let mut graph = SystemGraph::new();
        let system_a = generate_test_system(0, "SystemA");
        let system_b = generate_test_system(1, "SystemB");

        let node_a = graph.add_node(system_a.clone());
        let node_b = graph.add_node(system_b.clone());

        let gate = Stargate {
            id: 0,
            name: "TestGate".to_string(),
            distance: 5,
            destination_gate_id: 1,
            origin_system_id: 0,
            destination_system_id: 1,
            is_active: true,
        };

        let edge_1 = graph.add_edge(node_a, node_b, gate.clone());
        let edge_2 = graph.add_edge(node_a, node_b, gate.clone());

        // The two edges should be distinct.
        assert_ne!(edge_1, edge_2);
        assert_eq!(graph.graph.edge_count(), 2);
    }

    #[test]
    fn test_add_edge_to_self() {
        let mut graph = SystemGraph::new();
        let system_a = generate_test_system(0, "SystemA");

        let node_a = graph.add_node(system_a.clone());

        let gate = Stargate {
            id: 0,
            name: "TestGate".to_string(),
            distance: 5,
            destination_gate_id: 1,
            origin_system_id: 0,
            destination_system_id: 1,
            is_active: true,
        };

        let edge = graph.add_edge(node_a, node_a, gate.clone());

        // Assert edge was added and connects to itself.
        assert!(graph.contains_gate(edge));
        assert_eq!(graph.graph.edge_count(), 1);
    }

    #[test]
    fn test_adding_large_number_of_nodes() {
        let mut graph = SystemGraph::new();

        const NUM_NODES: usize = 100_000;

        let start_time = Instant::now();

        for i in 0..NUM_NODES {
            let system = generate_test_system(0, &format!("System{}", i));
            graph.add_node(system);
        }

        let duration = start_time.elapsed();

        println!("Time taken to add {} nodes: {:?}", NUM_NODES, duration);

        // Here, you might want to assert that duration is less than some threshold
        // if you have a performance requirement.
    }

    #[test]
    fn test_pathfinding_in_large_graph() {
        let mut graph = SystemGraph::new();

        const NUM_NODES: usize = 10000; // Reduced to 5

        let mut prev_system = generate_test_system(0, "System0");
        let first = prev_system.clone();
        graph.add_node(prev_system.clone());

        let mut gate_id = 0;

        for i in 1..NUM_NODES {
            let system = generate_test_system(i.try_into().unwrap(), &format!("System{}", i));
            let node = graph.add_node(system.clone());

            let gate1 = Stargate {
                id: gate_id,
                name: "TestGate".to_string(),
                distance: 1,
                destination_gate_id: 1,
                origin_system_id: 0,      // These IDS aren't correct
                destination_system_id: 1, // These IDS aren't correct
                is_active: true,
            };
            gate_id += 1;
            let gate2 = Stargate {
                id: gate_id,
                name: "TestGate".to_string(),
                distance: 1,
                destination_gate_id: 1,
                origin_system_id: 1,      // These ID's aren't correct
                destination_system_id: 0, // These ID's aren't correct
                is_active: true,
            };
            gate_id += 1;

            graph.add_edge(
                graph.system_to_node.get(&prev_system.id).unwrap().clone(),
                node,
                gate1,
            );

            graph.add_edge(
                node,
                graph.system_to_node.get(&prev_system.id).unwrap().clone(),
                gate2,
            );

            prev_system = system;
        }

        let start_time = Instant::now();

        // Try to find a path from the first to the last system
        let result = graph.get_pathfinding_between(&prev_system, &first);
        let duration = start_time.elapsed();

        if let Err(err) = &result {
            panic!("Pathfinding failed with error: {:?}", err);
        }

        println!(
            "Time taken for pathfinding in a graph with {} nodes: {:?}",
            NUM_NODES, duration
        );

        assert!(result.is_ok());
        // As mentioned before, if you have performance requirements, you might want to assert that duration is under a specific threshold.
    }
}
