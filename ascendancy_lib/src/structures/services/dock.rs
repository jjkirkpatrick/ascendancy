use bevy::prelude::*;
use bevy::{reflect::Reflect, utils::uuid};

use crate::{agent::agent::Agent, structures::station::ResourceManager};

// structures/services/Dock.rs
use super::StationServiceTrait;

/// The `Dock` struct represents the Dock service
#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct Dock {
    /// The ID of the dock
    pub id: u32,
    /// The name of the dock
    pub name: String,
    /// The amount of copacity the dock has
    pub capacity: u32,
    /// A list of ships currently docked
    pub docked_ships: Vec<Agent>,
    ///flat rate of energy consumption
    pub base_energy_consumption: f32,
    /// Whether the dock is active or not
    pub is_active: bool,
    /// fulctuation of energy consumption as a percentage
    energy_fluctuation: f32,
    /// Energy consumnption timer
    consumption_timer: Timer, // Add a Timer for consumption logic
}

impl Dock {
    /// Creates a new Dock service
    pub fn new(name: String, capacity: u32) -> Self {
        Dock {
            id: uuid::Uuid::new_v4().as_u128() as u32,
            name: name,
            capacity: capacity,
            docked_ships: Vec::with_capacity(capacity as usize),
            base_energy_consumption: 400.0,
            is_active: true,
            energy_fluctuation: 0.2,
            consumption_timer: Timer::from_seconds(5.0, bevy::time::TimerMode::Repeating), // Initialize the timer
        }
    }
}

impl StationServiceTrait for Dock {
    fn id(&self) -> u32 {
        // Dock-specific ID logic
        1
    }

    fn enable(&mut self) {
        self.is_active = true;
        // Dock-specific enable logic
        println!("Dock enabled");
    }

    fn disable(&mut self) {
        self.is_active = false;
        // Dock-specific disable logic
        println!("Dock disabled");
    }

    // Separate method for energy consumption
    fn consume_energy(&mut self, resources: &mut ResourceManager, time: &Res<Time>) -> bool {
        self.consumption_timer.tick(time.delta());

        if self.consumption_timer.finished() {
            let energy_consumption_per_ship =
                self.base_energy_consumption * (1.0 + self.energy_fluctuation);
            let total_energy_consumption =
                energy_consumption_per_ship * self.docked_ships.len() as f32;

            if resources.energy >= total_energy_consumption {
                resources.energy -= total_energy_consumption;
                if self.is_active != true {
                    self.enable();
                }
                return true; // Successfully consumed energy
            } else {
                if self.is_active != false {
                    self.disable();
                }
                return false;
            }
        }
        false
    }

    fn run(&mut self, _: &mut ResourceManager, _: &Res<Time>) {
        if self.is_active {
        } else {
        }
    }
}

impl PartialOrd for Dock {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.capacity.cmp(&other.capacity))
    }
}
