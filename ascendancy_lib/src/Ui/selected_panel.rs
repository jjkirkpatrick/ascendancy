use crate::{
    agent::agent::Agent,
    player_interactions::selection::UpdateSelectedItemEvent,
    structures::{stargate::Stargate, station::Station},
};
use bevy::prelude::*;

///marker for the selected item text
#[derive(Component)]
pub struct SelectedItemText;
/// UI panel for selected Items
pub fn selected_item_panel(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(15.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(200.),
                        border: UiRect::all(Val::Px(2.)),
                        ..default()
                    },
                    background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                ..default()
                            },
                            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // text
                            parent.spawn((
                                TextBundle::from_section(
                                    "Text Example",
                                    TextStyle {
                                        font_size: 30.0,
                                        ..default()
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Px(5.)),
                                    ..default()
                                }),
                                // Because this is a distinct label widget and
                                // not button/list item text, this is necessary
                                // for accessibility to treat the text accordingly.
                                Label,
                                SelectedItemText,
                            ));
                        });
                });
        });
}

/// Updates the UI system
pub fn update_ui_system(
    mut ev_selected_target: EventReader<UpdateSelectedItemEvent>,
    agents: Query<&Agent>,
    stargates: Query<&Stargate>,
    stations: Query<&Station>,
    mut text_query: Query<&mut Text, With<SelectedItemText>>, // Update this line
) {
    for event in ev_selected_target.read() {
        for mut text in text_query.iter_mut() {
            // Reset text or setup for new content
            text.sections.clear();

            // Check if the selected entity is a trader
            if let Ok(agent) = agents.get(event.0) {
                // Add cargo details
                text.sections.push(TextSection {
                    value: format!(
                        "Agent Name: {}\nHealth: {}\nHome System: {}",
                        agent.name, agent.health.current, agent.home_system.home.name
                    ),
                    ..default()
                });
            } else if let Ok(stargate) = stargates.get(event.0) {
                text.sections.push(TextSection {
                    value: format!(
                        "Stargate Name: {}\nDistance: {}\nActive: {}",
                        stargate.name, stargate.distance, stargate.is_active
                    ),
                    ..default()
                });
            } else if let Ok(station) = stations.get(event.0) {
                println!("Station Name: {}", station.name);
                text.sections.push(TextSection {
                    value: format!(
                        "Station Name: {}\nSystem: {}\nServices: {:?}\nResources: Energy - {} / {}",
                        station.name,
                        station.system_id,
                        station.services,
                        station.resource_manager.energy,
                        station.resource_manager.max_energy
                    ),
                    ..default()
                });
            }
            // Check if the selected entity is a station

            // Handle other entity types similarly
        }
    }
}
