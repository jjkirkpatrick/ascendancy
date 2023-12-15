//! Everything needed to run the main game logic

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::prelude::*;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use big_brain::prelude::*;
/// Set the game state to align systems with their respective runtimes
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Menu,
    //Playing,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("Ascendancy"),
                    present_mode: PresentMode::AutoVsync,
                    ..default()
                }),
                ..default()
            }), //.set(LogPlugin {
                //    // Use `RUST_LOG=big_brain=trace,thirst=trace cargo run --example thirst --features=trace` to see extra tracing output.
                //    filter: "big_brain=debug,sequence=debug".to_string(),
                //    ..default()
                //}),
        )
        .add_plugins((
            DefaultPickingPlugins,
            //WorldInspectorPlugin::new(),
            ScreenDiagnosticsPlugin::default(),
            ScreenFrameDiagnosticsPlugin,
            ascendancy_lib::player_interactions::InteractionPlugin,
            ascendancy_lib::graphics::GraphicsPlugin,
            ascendancy_lib::units::UnitPlugin,
            ascendancy_lib::world_gen::WorldGenPlugin,
            ascendancy_lib::solar_system::SolarSystemPlugin,
            ascendancy_lib::faction::FactionPlugin,
            BigBrainPlugin::new(PreUpdate),
        ))
        .run();
}
