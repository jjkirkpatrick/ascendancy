//! Everything needed to run the main game logic

use bevy::prelude::*;
use bevy::window::PresentMode;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::prelude::*;

use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use big_brain::prelude::*;

fn main() {
    App::new()
        .init_state::<ascendancy_lib::GameState>()
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
            WorldInspectorPlugin::new(),
            //ScreenDiagnosticsPlugin::default(),
            //ScreenFrameDiagnosticsPlugin,
            //ascendancy_lib::debug::DebugPlugin,
            ascendancy_lib::Ui::UiPlugin,
            ascendancy_lib::asset_management::AssetManagementPlugin,
            ascendancy_lib::loading::loading::LoadingPlugin,
            ascendancy_lib::menu::menu::MenuPlugin,
            ascendancy_lib::player_interactions::InteractionPlugin,
            ascendancy_lib::graphics::GraphicsPlugin,
            ascendancy_lib::world_gen::WorldGenPlugin,
            ascendancy_lib::solar_system::SolarSystemPlugin,
            ascendancy_lib::faction::FactionPlugin,
            ascendancy_lib::agent::UnitPlugin,
            ascendancy_lib::structures::StructurePlugin,
            BigBrainPlugin::new(PreUpdate),
        ))
        .run();
}
