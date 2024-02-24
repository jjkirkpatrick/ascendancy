use bevy::prelude::*;

use crate::GameState;

use self::station::run_active_services;
/// Station services
pub mod services;
/// startgate module
pub mod stargate;
/// Station definition
pub mod station;

/// A plugin for the structures module
pub struct StructurePlugin;

impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            run_active_services.run_if(in_state(GameState::Playing)),
        );
    }
}
