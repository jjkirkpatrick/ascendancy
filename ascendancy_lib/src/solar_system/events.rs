use bevy::prelude::*;

use super::SolarSystem;

/// Event for when an entity moves between solar systems
#[derive(Event)]
pub struct EntityMovedSystemEvent {
    entity: Entity,
    from: Option<Entity>, // Use `Option<Entity>` to handle cases where `from` is None (e.g., spawning)
    to: Entity,
}

/// System to update solar systems when an entity moves
pub fn update_solar_systems_on_entity_movement(
    mut events: EventReader<EntityMovedSystemEvent>,
    mut solar_systems: Query<(Entity, &mut SolarSystem)>,
) {
    for event in events.read() {
        if let Some(from_system) = event.from {
            // Query for the `from` solar system and remove the entity
            for (entity, mut from_solar_system) in solar_systems.iter_mut() {
                if entity == from_system {
                    from_solar_system.remove_entity(event.entity);
                    println!("Entity removed from system ");
                }
            }
        }

        // Query for the `to` solar system and add the entity
        for (entity, mut to_solar_system) in solar_systems.iter_mut() {
            if entity == event.to {
                to_solar_system.add_entity(event.entity);
                println!("Entity added to system ");
            }
        }
    }
}
