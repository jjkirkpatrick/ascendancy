use bevy::prelude::*;
use bevy::{reflect::Reflect, utils::uuid};

use crate::structures::station::ResourceManager;

// structures/services/market.rs
use super::StationServiceTrait;

/// The `Market` struct represents the market service
/// The `Market` struct represents the market service
#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct Market {
    /// The ID of the dock
    pub id: u32,
    /// The name of the dock
    pub name: String,
    ///flat rate of energy consumption
    pub base_energy_consumption: f32,
    /// Whether the dock is active or not
    pub is_active: bool,
    /// fulctuation of energy consumption as a percentage
    energy_fluctuation: f32,
    /// Energy consumnption timer
    consumption_timer: Timer, // Add a Timer for consumption logic
}

impl Market {
    /// Creates a new market service
    pub fn new() -> Self {
        Market {
            id: uuid::Uuid::new_v4().as_u128() as u32,
            name: "Market".to_string(),
            base_energy_consumption: 400.0,
            is_active: true,
            energy_fluctuation: 0.2,
            consumption_timer: Timer::from_seconds(5.0, bevy::time::TimerMode::Repeating), // Initialize the timer
        }
    }
}

impl StationServiceTrait for Market {
    fn id(&self) -> u32 {
        // Market-specific ID logic
        1
    }

    fn enable(&mut self) {
        self.is_active = true;
    }

    fn disable(&mut self) {
        self.is_active = false;
    }

    // Separate method for energy consumption
    fn consume_energy(&mut self, resources: &mut ResourceManager, time: &Res<Time>) -> bool {
        self.consumption_timer.tick(time.delta());
        if self.consumption_timer.finished() {
            let fluctuated_energy_consumption =
                self.base_energy_consumption * (1.0 + self.energy_fluctuation);

            if resources.energy >= fluctuated_energy_consumption {
                resources.energy -= fluctuated_energy_consumption;
                if self.is_active != true {
                    self.enable();
                }
                true // Successfully consumed energy
            } else {
                if self.is_active != false {
                    self.disable();
                }
                false
            }
        } else {
            false
        }
    }

    fn run(&mut self, _: &mut ResourceManager, _: &Res<Time>) {
        if self.is_active {
        } else {
        }
    }
}

impl PartialOrd for Market {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.base_energy_consumption
            .partial_cmp(&other.base_energy_consumption)
    }
}
