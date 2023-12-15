use bevy::prelude::*;
use big_brain::BigBrainSet;

use crate::units::pathfinding::SystemGraph;

use self::{
    fly_to_system_action::{
        //desire_to_travel_scorer_system,
        fly_to_system, //jump_stargate_system,
        //move_to_stargate_system,
        want_to_fly_to_system_scorer_system,
    },
    wandering_action::{
        wander_decision_system, wander_movement_system, wander_rotation_system,
        want_to_wander_scorer_system,
    },
};

/// agent module
pub mod agent;
/// fly to system action
pub mod fly_to_system_action;
/// pathfinding module
pub mod pathfinding;
/// Wandering Action
pub mod wandering_action;
/// The plugin for the unit module.
pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SystemGraph::default())
            .add_systems(
                PreUpdate,
                (
                    wander_decision_system.in_set(BigBrainSet::Actions),
                    wander_movement_system.in_set(BigBrainSet::Actions),
                    wander_rotation_system.in_set(BigBrainSet::Actions),
                    want_to_wander_scorer_system.in_set(BigBrainSet::Scorers),
                    fly_to_system.in_set(BigBrainSet::Actions),
                    want_to_fly_to_system_scorer_system.in_set(BigBrainSet::Scorers),
                ),
            )
            .register_type::<agent::Agent>();
    }
}
