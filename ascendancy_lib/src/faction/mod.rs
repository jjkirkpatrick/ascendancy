use self::{attributes::Attributes, bank::Bank, claims::owner_changed_system};
use bevy::prelude::*;

/// Set the game state to align systems with their respective runtimes
pub struct FactionPlugin;

impl Plugin for FactionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (create_faction_resourse, apply_deferred).chain())
            .add_systems(Update, owner_changed_system);
    }
}

/// The factions attributes
pub mod attributes;
/// The factions bank balance
pub mod bank;
/// the factions claims
pub mod claims;

/// The factions bundle
#[derive(Bundle, Clone)]
pub struct FactionBundle {
    /// Basic faction attributes
    pub faction_attributes: Attributes,
    /// The factions bank balance
    pub faction_bank: Bank,
}

/// The factions resourse
#[derive(Resource, Clone)]
pub struct FactionResourse {
    /// The factions
    pub factions: Vec<FactionBundle>,
}

/// Creates the faction resourse
fn create_faction_resourse(mut commands: Commands) {
    let factions_vec = vec![
        FactionBundle {
            faction_attributes: Attributes {
                id: attributes::FactionID { id: 0 },
                name: "Galactic Empire".to_string(),
                colors: Color::rgb(0.0, 0.0, 1.0),
            },
            faction_bank: bank::Bank::randomized(),
        },
        FactionBundle {
            faction_attributes: Attributes {
                id: attributes::FactionID { id: 1 },
                name: "Sith".to_string(),
                colors: Color::rgb(1.0, 0.0, 0.0),
            },
            faction_bank: bank::Bank::randomized(),
        },
        FactionBundle {
            faction_attributes: Attributes {
                id: attributes::FactionID { id: 3 },
                name: "FlimFlams".to_string(),
                colors: Color::rgb(0.0, 1.0, 0.0),
            },
            faction_bank: bank::Bank::randomized(),
        },
    ];

    let faction_resource = FactionResourse {
        factions: factions_vec,
    };

    commands.insert_resource(faction_resource);
}
