use crate::{faction::FactionResourse, solar_system::SolarSystem};
use bevy::prelude::*;
use rand::Rng;

/// Creates a faction entity for each faction
pub fn create_faction_entities(mut commands: Commands, factions: Res<FactionResourse>) {
    for faction in factions.factions.iter() {
        commands.spawn(faction.clone());
    }
}

/// Assigns a random faction to each system
pub fn assign_systems_to_factions(
    mut query: Query<(Entity, &mut SolarSystem), With<SolarSystem>>,
    factions: Res<FactionResourse>,
) {
    let mut rng = rand::thread_rng();
    for (_, mut solar_system) in query.iter_mut() {
        let faction_id = rng.gen_range(0..factions.factions.len());
        let faction = factions.factions.get(faction_id).unwrap();
        solar_system.attributes.owner = faction.faction_attributes.id;
    }
}
