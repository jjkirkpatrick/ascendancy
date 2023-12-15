use bevy::prelude::*;

use crate::solar_system::attributes::SystemAttributes;

use super::attributes::Attributes;

/// Detects when a system has changed owner and updates the color of the material on the entity to the color of the faction
pub fn owner_changed_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    solar_system: Query<(Entity, &SystemAttributes), Changed<SystemAttributes>>,
    factions: Query<(Entity, &Attributes)>,
) {
    for (entity, owner) in solar_system.iter() {
        for (_, faction) in factions.iter() {
            if faction.id == owner.owner {
                let material = materials.add(ColorMaterial::from(faction.colors));
                commands.entity(entity).insert(material);
            }
        }
    }
}
