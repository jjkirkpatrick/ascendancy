use bevy::{prelude::*, utils::uuid};
use std::time;

use crate::structures::station::ResourceManager;

// structures/services/SolarGenerator.rs
use super::StationServiceTrait;

/// The `solar_generator` struct represents the Solar Generator service
#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct SolarGenerator {
    /// The ID of the SolarGenerator
    pub id: u32,

    /// The name of the SolarGenerator
    pub name: String,

    /// The amount of energy the solar generator can produce per second
    pub energy_production: f32,

    /// The amount of energy the solar generator can store
    pub energy_storage: f32,

    /// The amount of energy the solar generator has stored
    pub stored_energy: f32,
    /// Energ production timer
    pub production_timer: Timer, // Add a Timer for consumption logic
    /// Energy consumption timer
    pub consumption_timer: Timer, // Add a Timer for consumption logic
}

impl SolarGenerator {
    /// Creates a new SolarGenerator service
    pub fn new(name: String) -> Self {
        SolarGenerator {
            id: uuid::Uuid::new_v4().as_u128() as u32,
            name: name,
            energy_production: 1000.0,
            energy_storage: 10000.0,
            stored_energy: 0.0,
            production_timer: Timer::from_seconds(5.0, bevy::time::TimerMode::Repeating), // Initialize the timer
            consumption_timer: Timer::from_seconds(5.0, bevy::time::TimerMode::Repeating), // Initialize the timer
        }
    }
}

impl StationServiceTrait for SolarGenerator {
    fn id(&self) -> u32 {
        // SolarGenerator-specific ID logic
        1
    }

    fn enable(&mut self) {
        
    }

    fn disable(&mut self) {
    }

    /// Consumes energy
    fn consume_energy(&mut self, resources: &mut ResourceManager, time: &Res<Time>) -> bool {
        self.production_timer.tick(time.delta());
        if self.production_timer.finished() {
            true
        } else {
            false
        }
    }

    fn run(&mut self, resources: &mut ResourceManager, time: &Res<Time>) {
        self.production_timer.tick(time.delta());

        if self.production_timer.finished() {
            let available_storage = resources.max_energy - resources.energy;
            let producible_energy = self.energy_production.min(available_storage);

            resources.energy += producible_energy;
            self.stored_energy += self.energy_production - producible_energy;

            // Ensure stored_energy does not exceed energy_storage
            if self.stored_energy > self.energy_storage {
                self.stored_energy = self.energy_storage;
            }

            let remaining_storage = resources.max_energy - resources.energy;
            let transferable_energy = self.stored_energy.min(remaining_storage);

            resources.energy += transferable_energy;
            self.stored_energy -= transferable_energy;
        }
    }
}

impl PartialOrd for SolarGenerator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.energy_production.partial_cmp(&other.energy_production)
    }
}
