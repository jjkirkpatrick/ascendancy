use bevy::prelude::*;

/// A stargate is a device within the game world that allows agents to travel between solar systems.
#[derive(Component, Reflect, Clone, Debug, PartialEq, PartialOrd, Hash, Eq)]
#[reflect(Component)]
pub struct Stargate {
    /// Stargate ID
    pub id: u32,
    /// The name of the star gate
    pub name: String,
    /// The distance from the gate to the destination
    pub distance: u32,
    /// The destination gates id
    pub destination_gate_id: u32,
    /// The Solar system this gate is in (the source)
    pub origin_system_id: u32,
    /// The destination solar system id
    pub destination_system_id: u32,
    /// Whether the gate is active or not
    pub is_active: bool,
}

impl Default for Stargate {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::from("Abandoned Star Gate"),
            distance: 0,
            destination_gate_id: 0,
            origin_system_id: 0,
            destination_system_id: 0,
            is_active: true,
        }
    }
}

impl Stargate {
    /// Creates a new stargate with the given ID and name.
    pub fn new(
        id: u32,
        name: String,
        distance: u32,
        destination_gate_id: u32,
        origin_system_id: u32,
        destination_system_id: u32,
        is_active: bool,
    ) -> Self {
        Stargate {
            id,
            name,
            distance,
            destination_gate_id,
            origin_system_id,
            destination_system_id,
            is_active,
        }
    }

    /// Get the stargate id
    pub fn id(&self) -> u32 {
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
    pub fn set_destination_gate_id(&mut self, destination: u32) {
        self.destination_gate_id = destination;
    }

    /// Set a `SystemGate` origin
    pub fn set_origin_system_id(&mut self, origin: u32) {
        self.origin_system_id = origin;
    }

    /// Set a `SystemGate` destination
    pub fn set_destination_system_id(&mut self, destination: u32) {
        self.destination_system_id = destination;
    }

    /// Get the ID of the origin system
    pub fn origin_system_id(&self) -> u32 {
        self.origin_system_id
    }

    /// Get the Gates system destination
    pub fn destination_system_id(&self) -> u32 {
        self.destination_system_id
    }
}

/// Vec of `SystemGates`
#[derive(Component, Default, Reflect, Clone, Debug, PartialEq, PartialOrd)]
#[reflect(Component)]
pub struct JumpGates(pub Vec<Stargate>);

impl JumpGates {
    /// add a gate to the system
    pub fn add_gate(&mut self, gate: Stargate) {
        self.0.push(gate);
    }
}
