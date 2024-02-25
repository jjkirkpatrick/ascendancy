use bevy::prelude::*;
use big_brain::BigBrainSet;

use crate::{agent::pathfinding::SystemGraph, GameState};

use self::{
    fly_to_system_action::{fly_to_system, want_to_fly_to_system_scorer_system},
    idle::{idle_action_system, idle_scorer_system},
    pathfinding::get_stargate_path_between_systems,
    random_path::{get_random_path_between_two_systems, PathTimer},
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
pub mod random_path;
/// utils
pub mod utils;

/// A plugin for the unit module
pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SystemGraph::default())
            .insert_resource(PathTimer(Timer::from_seconds(5.0, TimerMode::Repeating)))
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
            //.add_systems(FixedUpdate, get_random_path_between_two_systems.run_if(in_state(GameState::Playing)))
            .register_type::<agent::Agent>();
    }
}
