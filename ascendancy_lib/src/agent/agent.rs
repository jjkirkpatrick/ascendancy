use crate::solar_system::attributes::SystemAttributes;
use crate::structures::stargate::Stargate;

use bevy::prelude::*;

/// Represents an agent in the game world. This is the most important component, and it should be added to all entities that represent agents.
#[derive(Component, Default, Reflect, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct Agent {
    /// The unique ID of the agent.
    pub id: u32,
    /// The name of the agent.
    pub name: String,
    /// The agent's wallet.
    pub wallet: Wallet,
    /// The agent's current goal.
    pub current_goal: CurrentGoal,
    /// The agent's health.
    pub health: Health,
    /// The agent's home system.
    pub home_system: HomeSystem,
    /// The agent's current system.
    pub current_system: CurrentSystem,
    /// The agent's target location.
    pub target_system: TargetSystem,
    /// The agests path to the target system.
    pub stargate_path: StargatePath,
    /// The targers current destination in local space
    pub target_destination: Option<Vec3>,
    /// The speed of the agent.
    pub speed: f32,
}

impl Agent {
    /// Creates a new agent with the given ID and name.
    pub fn new(id: u32, name: String, home_system: SystemAttributes) -> Self {
        Agent {
            id,
            name,
            wallet: Wallet { money: 100.0 },
            current_goal: CurrentGoal { goal: None },
            health: Health {
                current: 100.0,
                max: 100.0,
            },
            home_system: HomeSystem {
                home: home_system.clone(),
            },
            current_system: CurrentSystem {
                system: home_system.clone(),
            },
            target_system: TargetSystem { location: None },
            speed: 100.0,
            stargate_path: StargatePath { path: Vec::new() },
            target_destination: None,
        }
    }

    /// Set the path to the target system.
    pub fn set_stargate_path(&mut self, path: Vec<Stargate>) {
        self.stargate_path.path = path;
    }
}

/// Represents the financial assets of an agent.
#[derive(Component, Default, Reflect, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct Wallet {
    /// The amount of money the agent has.
    pub money: f32,
}

/// Represents the agent's current goal.
#[derive(Component, Default, Reflect, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub enum Goal {
    #[default]
    /// The agent has no goal.
    Rest,
    /// The agent is exploring the world.
    Explore,
    /// The agent is defending a location.
    Defend,
    /// The agent is trading with another agent.
    Trade,
}

/// Represents the agent's current goal.
#[derive(Component, Default, Reflect, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct CurrentGoal {
    /// The agent's current goal.
    pub goal: Option<Goal>,
}

/// Represents the health of the agent. This might be essential if there's any form of combat or danger in your game.
#[derive(Component, Default, Reflect, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct Health {
    /// The agent's current health.
    pub current: f32,
    /// The agent's maximum health.
    pub max: f32,
}

/// Represents the faction or group the agent belongs to.
#[derive(Component, Default, Reflect, Clone, PartialEq)]
#[reflect(Component)]
pub enum Faction {
    #[default]
    /// The agent is neutral or independent.
    Neutral,
    /// The agent is part of the player's faction.
    Trader,
    /// The agent is part of the enemy faction.
    Explorer,
    /// The agent is part of the enemy faction.
    Defender,
}

/// Represents the agent's home system.
#[derive(Component, Default, Reflect, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct HomeSystem {
    /// The agent's home system.
    pub home: SystemAttributes,
}

/// Represents the agent's current system.
#[derive(Component, Default, Reflect, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct CurrentSystem {
    /// The agent's current system.
    pub system: SystemAttributes,
}

/// Represents the agent's target system.
#[derive(Component, Default, Reflect, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct TargetSystem {
    /// The agent's target system.
    pub location: Option<SystemAttributes>,
}

/// vec list of stargates to travel through to reach target system
#[derive(Component, Default, Reflect, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct StargatePath {
    /// The list of stargates to travel through to reach target system.
    pub path: Vec<Stargate>,
}
