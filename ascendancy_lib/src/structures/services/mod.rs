use self::{dock::Dock, market::Market, solar_generator::SolarGenerator};
use bevy::prelude::*;
use bevy::reflect::Reflect;

use super::station::ResourceManager;

/// The `Dock` struct represents the Dock service
pub mod dock;
/// The `Market` struct represents the market service
pub mod market;
/// The `solar_generator` struct represents the Solar Generator service
pub mod solar_generator;

/// The `services` module contains all the services that can be run on a station
pub trait StationServiceTrait: std::fmt::Debug + Send + Sync + PartialEq {
    /// Get the service id
    fn id(self: &Self) -> u32; // Provide a unique identifier for the service
    /// Enable the service
    fn enable(&mut self);
    /// Disable the service
    fn disable(&mut self);
    /// Consumes energy
    fn consume_energy(&mut self, resources: &mut ResourceManager, time: &Res<Time>) -> bool;
    /// Run the service
    fn run(&mut self, resources: &mut ResourceManager, time: &Res<Time>);
}

/// The `StationServices` enum represents all the services that can be run on a station
#[derive(Debug, Clone, PartialEq, PartialOrd, Reflect)]
pub enum StationServices {
    /// Dock service
    Dock(Dock),
    /// Market service
    Market(Market),
    /// Solar Generator service
    SolarGenerator(SolarGenerator),
}

impl StationServiceTrait for StationServices {
    /// Get the service id
    fn id(self: &Self) -> u32 {
        match self {
            StationServices::Dock(dock) => dock.id(),
            StationServices::Market(market) => market.id(),
            StationServices::SolarGenerator(solar_generator) => solar_generator.id(),
        }
    }

    /// Enable the service
    fn enable(&mut self) {
        match self {
            StationServices::Dock(dock) => dock.enable(),
            StationServices::Market(market) => market.enable(),
            StationServices::SolarGenerator(solar_generator) => solar_generator.enable(),
        }
    }

    /// Disable the service
    fn disable(&mut self) {
        match self {
            StationServices::Dock(dock) => dock.disable(),
            StationServices::Market(market) => market.disable(),
            StationServices::SolarGenerator(solar_generator) => solar_generator.disable(),
        }
    }

    /// Consumes energy
    /// Consumes energy
    fn consume_energy(&mut self, resources: &mut ResourceManager, time: &Res<Time>) -> bool {
        match self {
            StationServices::Dock(dock) => dock.consume_energy(resources, time),
            StationServices::Market(market) => market.consume_energy(resources, time),
            StationServices::SolarGenerator(solar_generator) => {
                solar_generator.consume_energy(resources, time)
            }
        }
    }

    /// Run the service
    fn run(&mut self, resources: &mut ResourceManager, time: &Res<Time>) {
        match self {
            StationServices::Dock(dock) => dock.run(resources, time),
            StationServices::Market(market) => market.run(resources, time),
            StationServices::SolarGenerator(solar_generator) => {
                solar_generator.run(resources, time)
            }
        }
    }
}
