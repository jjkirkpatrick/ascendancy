use bevy::prelude::*;
use rand::seq::SliceRandom;

use crate::solar_system::{attributes::SystemAttributes, gates::SystemGate};

use super::pathfinding::SystemGraph;

/// A timer for the pathfinding
#[derive(Resource)]
pub struct PathTimer(pub Timer);

/// Get a random path between two systems
pub fn get_random_path_between_two_systems(
    time: Res<Time>,
    mut timer: ResMut<PathTimer>,
    mut gizmos: Gizmos,
    system_graph: Res<SystemGraph>,
    solar_systems: Query<(&SystemAttributes)>,
    star_gates: Query<(&SystemGate, &GlobalTransform)>,
    mut config: ResMut<GizmoConfig>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        config.line_width = 30.0;

        // pick two random systems
        let mut rng = rand::thread_rng();

        let solar_systems_vec: Vec<_> = solar_systems.iter().collect();

        let system_a = solar_systems_vec.choose(&mut rng).unwrap();
        let system_b = solar_systems_vec.choose(&mut rng).unwrap();

        // get the path between the two systems
        let path = system_graph.get_pathfinding_between(system_a, system_b);

        println!("Path between {} and {}:", system_a.id, system_b.id);

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
                    match gate_transform {
                        Some(transform) => {
                            println!(
                                "Travelling through gate: {}, Global Transform: {:?}",
                                gate.id(),
                                transform.translation()
                            );
                            if let Some(prev_transform) = previous_gate_transform {
                                gizmos.line_2d(
                                    Vec2::new(
                                        prev_transform.translation().x,
                                        prev_transform.translation().y,
                                    ),
                                    Vec2::new(transform.translation().x, transform.translation().y),
                                    Color::RED,
                                );
                            }
                            previous_gate_transform = Some(transform);
                        }
                        None => println!("No gate found for id: {}", gate.id()),
                    }
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
                timer.0.tick(time.delta());
            }
        }

        println!("End of path");
    }
}
