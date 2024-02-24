use bevy::prelude::*;
use bevy::utils::Uuid;

use self::attributes::SystemAttributes;
use crate::faction::attributes::FactionID;
use crate::structures::stargate::JumpGates;
use crate::structures::stargate::Stargate;

/// Solar system attributes
pub mod attributes;
/// System Gates

/// Set the game state to align systems with their respective runtimes
pub struct SolarSystemPlugin;

impl Plugin for SolarSystemPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SystemAttributes>()
            .register_type::<JumpGates>()
            .register_type::<Stargate>()
            .register_type::<Uuid>();
    }
}

/// The solar system
#[derive(Bundle)]
pub struct SolarSystem {
    /// The solar systems attributes
    pub attributes: SystemAttributes,

    /// a Vec of jumpgates in the system
    pub jumpgates: JumpGates,
}

impl SolarSystem {
    /// Create a new solar system
    pub fn new_placeholder() -> Self {
        Self {
            attributes: SystemAttributes {
                id: 0,
                name: "Placeholder".to_string(),
                owner: FactionID { id: 0 },
            },
            jumpgates: Default::default(),
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

    /// Add a jumpgate to the system
    pub fn add_jumpgate(&mut self, jumpgate: Stargate) {
        self.jumpgates.0.push(jumpgate);
    }

    /// Remove a jumpgate from the system
    pub fn remove_jumpgate(&mut self, jumpgate: Stargate) {
        self.jumpgates.0.retain(|x| *x != jumpgate);
    }
}

#[cfg(test)]
mod tests {
    use crate::agent::agent::StargatePath;

    use super::*;

    #[test]
    fn test_create_new_placeholder() {
        let system = SolarSystem::new_placeholder();
        assert_eq!(system.attributes.name, "Placeholder");
        // Add any other assertions related to placeholder attributes here
    }

    #[test]
    fn test_update_system_name() {
        let mut system = SolarSystem::new_placeholder();
        system.update_system_name("NewSystem".to_string());
        assert_eq!(system.attributes.name, "NewSystem");
    }

    #[test]
    fn test_update_system_owner() {
        let mut system = SolarSystem::new_placeholder();
        let new_owner_id = FactionID { id: 0 };
        system.update_system_owner(new_owner_id.clone());
        assert_eq!(system.attributes.owner.id, new_owner_id.id);
    }

    #[test]
    fn test_add_jumpgate() {
        let mut system = SolarSystem::new_placeholder();
        let gate = Stargate {
            id: 0,
            name: "TestGate".to_string(),
            distance: 0,
            destination_gate_id: 0,
            origin_system_id: 0,
            destination_system_id: 0,
            is_active: true,
        };
        system.add_jumpgate(gate.clone());
        assert!(system.jumpgates.0.contains(&gate));
    }

    #[test]
    fn test_remove_jumpgate() {
        let mut system = SolarSystem::new_placeholder();
        let gate = Stargate {
            id: 0,
            name: "TestGate".to_string(),
            distance: 0,
            destination_gate_id: 0,
            origin_system_id: 0,
            destination_system_id: 0,
            is_active: true,
        };
        system.add_jumpgate(gate.clone());
        system.remove_jumpgate(gate.clone());
        assert!(!system.jumpgates.0.contains(&gate));
    }
}
