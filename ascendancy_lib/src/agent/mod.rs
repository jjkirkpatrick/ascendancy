use bevy::prelude::*;
use big_brain::BigBrainSet;

use crate::agent::pathfinding::SystemGraph;

use self::{
    fly_to_system_action::{fly_to_system, want_to_fly_to_system_scorer_system},
    idle::{idle_action_system, idle_scorer_system},
};

/// agent module
pub mod agent;
/// fly to system action
pub mod fly_to_system_action;
/// idleing Action
pub mod idle;
/// pathfinding module
pub mod pathfinding;
/// The plugin for the unit module.
pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SystemGraph::default())
            .add_systems(
                FixedUpdate,
                (
                    (idle_action_system, fly_to_system).in_set(BigBrainSet::Actions),
                    (
                        idle_scorer_system,
                        want_to_fly_to_system_scorer_system, //fly_to_system,
                                                             //want_to_fly_to_system_scorer_system,
                    )
                        .in_set(BigBrainSet::Scorers),
                ),
            )
            .register_type::<agent::Agent>();
    }
}
