use bevy::prelude::*;

/// System Gates
#[derive(Component, Reflect, Clone, Copy, Debug, PartialEq, PartialOrd)]
#[reflect(Component)]
pub struct SystemGate {
    /// The ID of the gate
    pub id: u16, // or String, or other type of unique identifier
    /// The distance from the gate to the destination
    pub distance: u32,
    /// The destination gates id
    pub destination_gate_id: u16,

    /// The Solar system this gate is in (the source)
    pub origin_system_id: u16,

    /// The destination solar system id
    pub destination_system_id: u16,
    /// Whether the gate is active or not
    pub is_active: bool,
}

impl Default for SystemGate {
    fn default() -> Self {
        Self {
            id: 0,
            distance: 0,
            destination_gate_id: 0,
            origin_system_id: 0,
            destination_system_id: 0,
            is_active: true,
        }
    }
}

impl SystemGate {
    /// Create a new system gate
    pub fn new_placeholder() -> Self {
        Self {
            id: 0,
            distance: 0,
            destination_gate_id: 0,
            origin_system_id: 0,
            destination_system_id: 0,
            is_active: true,
        }
    }

    /// Get the stargate id
    pub fn id(&self) -> u16 {
        self.id
    }

    /// Deactivate the gate
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    /// Activate the gate
    pub fn activate(&mut self) {
        self.is_active = true;
    }

    /// Check if the gate is active
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Get the distance to the destination
    pub fn distance(&self) -> u32 {
        self.distance
    }

    /// Set a `SystemGate` destination
    pub fn set_destination_gate_id(&mut self, destination: u16) {
        self.destination_gate_id = destination;
    }

    /// Set a `SystemGate` origin
    pub fn set_origin_system_id(&mut self, origin: u16) {
        self.origin_system_id = origin;
    }

    /// Set a `SystemGate` destination
    pub fn set_destination_system_id(&mut self, destination: u16) {
        self.destination_system_id = destination;
    }

    /// Get the ID of the origin system
    pub fn origin_system_id(&self) -> u16 {
        self.origin_system_id
    }

    /// Get the Gates system destination
    pub fn destination_system_id(&self) -> u16 {
        self.destination_system_id
    }
}

/// Vec of `SystemGates`
#[derive(Component, Default, Reflect, Clone, Debug, PartialEq, PartialOrd)]
#[reflect(Component)]
pub struct JumpGates(pub Vec<SystemGate>);

impl JumpGates {
    /// add a gate to the system

    pub fn add_gate(&mut self, gate: SystemGate) {
        self.0.push(gate);
    }
}
