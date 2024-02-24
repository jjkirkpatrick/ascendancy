use crate::structures::services::{StationServiceTrait, StationServices};
use bevy::{prelude::*, ui::debug};
use std::fmt;

/// A station is a location within the game world that provides services to agents.
#[derive(Component, PartialEq, PartialOrd, Reflect)]
#[reflect(Component)]
pub struct Station {
    /// Station ID
    pub id: u32,
    /// The name of the station
    pub name: String,
    /// The solar system this station is in
    pub system_id: u16,
    /// The resource manager for the station
    pub resource_manager: ResourceManager,
    /// The services provided by the station
    pub services: Vec<StationServices>, // Store any service dynamically
    /// Whether the station is active or not
    pub is_active: bool,
}

/// The `ResourceManager` struct represents the resource manager for a station
#[derive(Component, PartialEq, PartialOrd, Reflect, Debug)]
pub struct ResourceManager {
    /// The amount of energy the station has
    pub energy: f32,
    /// The maximum amount of energy the station can have
    pub max_energy: f32,
}

impl ResourceManager {
    fn consume_energy(&mut self, amount: f32) -> bool {
        if self.energy >= amount {
            self.energy -= amount;
            true
        } else {
            false
        }
    }

    fn produce_energy(&mut self, amount: f32) {
        self.energy += amount;
    }

    // Methods for managing materials...
}

impl Station {
    /// Creates a new station with the given ID and name.
    pub fn new(id: u32, name: String, system_id: u16) -> Self {
        Station {
            id,
            name,
            system_id,
            resource_manager: ResourceManager {
                energy: 0.,
                max_energy: 10000.0,
            },
            services: Vec::with_capacity(5), // Initialize with capacity for 5 services
            is_active: true,
        }
    }

    /// Add service to the station
    pub fn add_service(&mut self, service: StationServices) -> Result<(), String> {
        if self.services.len() < 5 {
            if !self.services.contains(&service) {
                self.services.push(service);
                Ok(())
            } else {
                Err(format!("Service {:?} already exists.", service))
            }
        } else {
            Err("Maximum number of services reached.".to_string())
        }
    }

    /// Remove service from the station
    pub fn remove_service(&mut self, service: &StationServices) -> Result<(), String> {
        if let Some(index) = self.services.iter().position(|x| x == service) {
            self.services.remove(index);
            Ok(())
        } else {
            Err("Service not found.".to_string())
        }
    }
    /// Placeholder method to disable a service
    pub fn disable_service(&self, service: &StationServices) {
        // Implementation depends on your game logic
        println!("Disabling service: {:?}", service);
    }
}

impl fmt::Debug for Station {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Station")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("system_id", &self.system_id)
            // Since we can't automatically derive Debug for services, we might choose to simply print their count or a placeholder.
            .field(
                "services",
                &format_args!("{} services", self.services.len()),
            )
            .field("is_active", &self.is_active)
            .field("resources", &self.resource_manager)
            .finish()
    }
}

/// Run all active services on stations
pub fn run_active_services(mut query: Query<&mut Station>, time: Res<Time>) {
    for mut station in query.iter_mut() {
        let mut services = station.services.clone(); // Clone the services
        for service in &mut services {
            match service {
                StationServices::Dock(dock) => {
                    dock.consume_energy(&mut station.resource_manager, &time);
                    dock.run(&mut station.resource_manager, &time);
                }
                StationServices::Market(market) => {
                    market.consume_energy(&mut station.resource_manager, &time);
                    market.run(&mut station.resource_manager, &time);
                }
                StationServices::SolarGenerator(solar_generator) => {
                    solar_generator.run(&mut station.resource_manager, &time);
                }
            }
        }
        station.services = services; // Update the station's services
    }
}
