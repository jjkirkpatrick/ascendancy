use bevy::{prelude::*, gizmos};
use rand::seq::SliceRandom;

use crate::{solar_system::{attributes::SystemAttributes, gates::SystemGate}, player_interactions::selection::Selection};

use super::pathfinding::SystemGraph;



/// get a path between two selected Systems
pub fn get_stargate_path_between_systems(
    selected_systems: Res<Selection>,
    mut gizmos: Gizmos,
    system_graph: Res<SystemGraph>,
    star_gates: Query<(&SystemGate, &GlobalTransform)>,
){
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
            let mut previous_gate_transform: Option<GlobalTransform> = None;
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

