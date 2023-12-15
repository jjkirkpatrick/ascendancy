use bevy::prelude::*;

use crate::{
    solar_system::{attributes::SystemAttributes, gates::SystemGate},
    units::pathfinding::SystemGraph,
};

/// Creates a graph of all solar systems and edge connections (Gates) used for pathfinding.
pub fn create_system_graph(
    mut system_graph: ResMut<SystemGraph>,
    solar_systems: Query<&SystemAttributes>,
    jump_gate: Query<&SystemGate>,
) {
    for system in solar_systems.iter() {
        system_graph.add_node(system.clone());
    }

    // for each system gate, get both the source and destination system from SolarSystem where SystemGate.destination == SolarSystem.id
    // and then add an edge to the graph between the two systems and the system gate

    for gate in jump_gate.iter() {
        let source_system = system_graph.node_by_id(&gate.origin_system_id()).cloned();
        let destination_system = system_graph
            .node_by_id(&gate.destination_system_id)
            .cloned();

        match (source_system, destination_system) {
            (Some(source), Some(destination)) => {
                system_graph.add_edge(source, destination, *gate);
            }
            _ => {
                println!("Error: Could not find source or destination system for gate");
            }
        }
    }
}
