use bevy::prelude::*;

use crate::faction::attributes::FactionID;

/// System Gates
#[derive(Component, Reflect, Clone, Debug, PartialEq, PartialOrd, Eq, Hash)]
#[reflect(Component)]
pub struct SystemAttributes {
    /// The ID of the system
    pub id: u16,
    /// The Name of the system
    pub name: String,
    /// The faction that owns the system
    pub owner: FactionID,
}

impl Default for SystemAttributes {
    fn default() -> Self {
        Self {
            id: 0,
            name: "Placeholder".to_string(),
            owner: FactionID::default(),
        }
    }
}
