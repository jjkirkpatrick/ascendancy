use crate::player_interactions::selection::UpdateSelectedItemEvent;
use crate::ui::selected_panel::selected_item_panel;
use crate::GameState;
use bevy::app::{App, Plugin, Update};
use bevy::ecs::schedule::common_conditions::on_event;
use bevy::ecs::schedule::IntoSystemConfigs;
use bevy::prelude::OnEnter;

use self::selected_panel::update_ui_system;

///Selected panel mod
pub mod selected_panel;

///Asset loading plugin
pub struct UiPlugin;

/// A plugin for loading Ui systems
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), selected_item_panel)
            .add_systems(
                Update,
                update_ui_system.run_if(on_event::<UpdateSelectedItemEvent>()),
            );
    }
}
