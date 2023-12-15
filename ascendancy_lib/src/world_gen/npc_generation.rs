use crate::{
    solar_system::attributes::SystemAttributes,
    units::{
        agent::Agent,
        fly_to_system_action::{
            FlyToSystem,
            //fly_to_system, jump_stargate_system, move_to_stargate_system,
            WantToFlyToSystem,
        },
        wandering_action::{Wander, WantToWander},
    },
};
use bevy::prelude::*;
use big_brain::{prelude::Highest, thinker::Thinker};
use rand::seq::SliceRandom;
use rand::Rng;

/// The number of agents to spawn
const AGENTS_TO_SPAWN: u32 = 10000;

/// Spawns a new agents `AGENTS_TO_SPAWN` number of times
pub fn spawn_agent(
    mut commands: Commands,
    query: Query<(Entity, &SystemAttributes, &Transform)>,
    asset_server: Res<AssetServer>,
) {
    // Build the thinker

    // Collect all solar systems and their positions into a vector
    let systems_with_positions: Vec<_> = query.iter().collect();

    // Get a thread RNG (Random Number Generator)
    let mut rng = rand::thread_rng();

    // Choose a random solar system and its position
    for _ in 0..AGENTS_TO_SPAWN {
        let thinker = Thinker::build()
            .label("WandererThinker")
            .picker(Highest {})
            .when(WantToWander, Wander { target: None }) // Always wander as we have set the score high.
            .when(
                WantToFlyToSystem,
                FlyToSystem {
                    target: None,
                    desire: 0.0,
                },
            );

        if let Some((_, solar_system, position)) = systems_with_positions.choose(&mut rng) {
            let spawn_position =
                random_position_in_system(Vec2::splat(512.0), position.translation);

            let _e = commands
                .spawn((
                    SpriteBundle {
                        texture: asset_server.load("sprites/icons/ships/small-trader.png"),
                        transform: Transform {
                            translation: spawn_position,
                            scale: Vec3::splat(1.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Agent::new(0, String::from("Trader"), (*solar_system).clone()),
                    Wander::new(),
                    FlyToSystem {
                        target: None,
                        desire: 1.0,
                    },
                    thinker,
                    Name::new("Trader"),
                ))
                .id();
            // Assuming the Agent component has an `id` field that you want to update
        } else {
            //eprintln!("No SolarSystem entities found to spawn agent at!");
        }
    }
}

/// Returns a random position in the system.
pub fn random_position_in_system(hex_size: Vec2, system_position: Vec3) -> Vec3 {
    let buffer = hex_size.x * 0.5; // Using 1/4 of the hex size as buffer
    let random_x = rand::thread_rng().gen_range(
        (system_position.x - hex_size.x + buffer)..(system_position.x + hex_size.x - buffer),
    );
    let random_y = rand::thread_rng().gen_range(
        (system_position.y - hex_size.y + buffer)..(system_position.y + hex_size.y - buffer),
    );
    Vec3::new(random_x, random_y, system_position.z)
}
