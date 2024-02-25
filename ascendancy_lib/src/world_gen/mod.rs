use bevy::prelude::*;

use crate::world_gen::faction_generation::{assign_systems_to_factions, create_faction_entities};
use crate::world_gen::generate_system_path::create_system_graph;
use crate::world_gen::npc_generation::spawn_agent;
use crate::world_gen::solar_system_generation::create_galaxy_solar_systems;
use crate::GameState;

use self::solar_system_generation::{spawn_space_station, GalaxyConfig};
use self::stargate_generation::spawn_stargates;
/// Set the game state to align systems with their respective runtimes
pub struct WorldGenPlugin;

/// The plugin that handles Factions generation.
pub(crate) mod faction_generation;
/// The plugin that handles `SystemPaths` generation.
pub(crate) mod generate_system_path;
/// The plugin that handles NPC generation.
pub(crate) mod npc_generation;
/// The plugin that handles solar system generation.
pub(crate) mod solar_system_generation;
/// The plugin that handles stargate generation.
pub(crate) mod stargate_generation;

impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .insert_resource(GalaxyConfig::default())
            .add_systems(
                OnEnter(GameState::WorldGenerating),
                (
                    create_galaxy_solar_systems,
                    apply_deferred,
                    spawn_stargates,
                    create_system_graph,
                    create_faction_entities,
                    assign_systems_to_factions,
                    apply_deferred,
                    spawn_agent,
                    spawn_space_station,
                )
                    .chain(),
            );
        // The manage_state method has been removed, so we can't use it here anymore.
        // We need to replace it with the appropriate logic or method.
    }
}
