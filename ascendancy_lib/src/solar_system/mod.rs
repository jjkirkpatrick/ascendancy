use bevy::prelude::*;
use bevy::utils::Uuid;

use self::attributes::SystemAttributes;
use self::events::update_solar_systems_on_entity_movement;
use crate::faction::attributes::FactionID;
use crate::GameState;

/// Solar system attributes
pub mod attributes;
/// Solar system events
pub mod events;

/// Set the game state to align systems with their respective runtimes
pub struct SolarSystemPlugin;

impl Plugin for SolarSystemPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SystemAttributes>()
            .add_event::<events::EntityMovedSystemEvent>()
            .register_type::<Uuid>()
            .register_type::<SolarSystem>()
            .add_systems(
                FixedUpdate,
                update_solar_systems_on_entity_movement.run_if(in_state(GameState::Playing)),
            );
    }
}

/// A list of entities in the solar system
#[derive(Default, Component, Reflect, Clone, Debug, PartialEq, PartialOrd, Eq, Hash)]
pub struct EntityList(pub Vec<Entity>);

/// The solar system
#[derive(Component, Default, Reflect, Clone, Debug, PartialEq, PartialOrd, Eq, Hash)]
pub struct SolarSystem {
    /// The solar systems attributes
    pub attributes: SystemAttributes,

    /// a Vec of jumpgates in the system
    pub entities: EntityList,
}

impl SolarSystem {
    /// Create a new solar system
    pub fn new_placeholder() -> Self {
        Self {
            attributes: SystemAttributes::default(),
            entities: EntityList::default(),
        }
    }

    /// Update the name of the system
    pub fn update_system_name(&mut self, name: String) {
        self.attributes.name = name;
    }

    /// Update the owner of the system
    pub fn update_system_owner(&mut self, owner: FactionID) {
        self.attributes.owner = owner;
    }

    /// Add an entity to the system
    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.0.push(entity);
    }

    /// Remove an entity from the system
    pub fn remove_entity(&mut self, entity: Entity) {
        self.entities.0.retain(|&e| e != entity);
    }
}
