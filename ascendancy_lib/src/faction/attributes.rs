use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// The factions bank balance
#[derive(
    Component,
    Serialize,
    Default,
    Deserialize,
    Reflect,
    Clone,
    Copy,
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Hash,
)]
#[reflect(Component)]
pub struct FactionID {
    /// The ID of the faction
    pub id: u8,
}

/// The factions bank balance
#[derive(Component, Serialize, Deserialize, Reflect, Clone, Debug, PartialEq)]
#[reflect(Component)]
pub struct Attributes {
    /// The ID of the faction
    pub id: FactionID,
    /// The Name of the faction
    pub name: String,
    /// the factions colors
    pub colors: Color,
}

impl Default for Attributes {
    fn default() -> Self {
        Self {
            id: FactionID::default(),
            name: "Placeholder".to_string(),
            colors: Color::rgb(0.0, 0.0, 0.0),
        }
    }
}
