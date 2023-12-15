use bevy::prelude::*;

use crate::world_gen::faction_generation::{assign_systems_to_factions, create_faction_entities};
use crate::world_gen::generate_system_path::create_system_graph;
use crate::world_gen::npc_generation::spawn_agent;
use crate::world_gen::solar_system_generation::{create_galaxy_solar_systems, spawn_stargates};
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

impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<WorldGenState>()
            .add_systems(
                OnEnter(WorldGenState::Generating),
                (
                    create_galaxy_solar_systems,
                    apply_deferred,
                    spawn_stargates,
                    apply_deferred,
                    create_system_graph,
                    apply_deferred,
                    create_faction_entities,
                    apply_deferred,
                    assign_systems_to_factions,
                    apply_deferred,
                    spawn_agent,
                )
                    .chain(),
            )
            .add_systems(
                PreUpdate,
                (WorldGenState::manage_state).run_if(
                    |world_gen_state: Res<State<WorldGenState>>| {
                        world_gen_state.get() != &WorldGenState::Complete
                    },
                ),
            );
    }
}

/// Tracks world generation progress.
#[derive(Default, States, Clone, Debug, PartialEq, Eq, Hash)]
pub enum WorldGenState {
    /// The world is waiting to be generated.
    #[default]
    Waiting,
    /// The world is being generated.
    Generating,

    /// The world is being simulated to let it stabilize.
    BurningIn,
    /// The world has been generated.
    Complete,
}

impl WorldGenState {
    /// A system that advances the world generation state machine.
    fn manage_state(
        world_gen_state: Res<State<WorldGenState>>,
        mut next_world_gen_state: ResMut<NextState<WorldGenState>>,
    ) {
        match world_gen_state.get() {
            WorldGenState::Waiting => {
                println!("Starting world generation");
                next_world_gen_state.set(WorldGenState::Generating);
            }
            WorldGenState::Generating => {
                println!("World generation complete");
                next_world_gen_state.set(WorldGenState::BurningIn);
            }
            WorldGenState::BurningIn => {
                println!("World burn-in complete");
                next_world_gen_state.set(WorldGenState::Complete);
            }
            WorldGenState::Complete => (),
        }
    }
}
