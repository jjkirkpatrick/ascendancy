use crate::player_interactions::selection::UpdateSelectedItemEvent;
use crate::GameState;
use crate::{
    agent::{
        agent::Agent,
        fly_to_system_action::{
            FlyToSystem,
            //fly_to_system, jump_stargate_system, move_to_stargate_system,
            WantToFlyToSystem,
        },
        idle::{Idle, WantToWander},
    },
    loading::loading::TextureAssets,
    solar_system::attributes::SystemAttributes,
};
use bevy::prelude::*;
use bevy_mod_picking::events::{Down, Pointer};
use bevy_mod_picking::prelude::On;
use bevy_mod_picking::PickableBundle;
use big_brain::prelude::*;
use fakeit::name;
use rand::seq::SliceRandom;
use rand::Rng;

/// The number of agents to spawn
const AGENTS_TO_SPAWN: u32 = 10000;

/// Spawns a new agents `AGENTS_TO_SPAWN` number of times
pub fn spawn_agent(
    mut commands: Commands,
    query: Query<(Entity, &SystemAttributes, &Transform)>,
    mut state: ResMut<NextState<GameState>>,
    textures: Res<TextureAssets>,
) {
    // Build the thinker

    // Collect all solar systems and their positions into a vector
    let systems_with_positions: Vec<_> = query.iter().collect();

    // Get a thread RNG (Random Number Generator)
    let mut rng = rand::thread_rng();

    // Choose a random solar system and its position
    for _ in 0..AGENTS_TO_SPAWN {
        //let find_and_execute_trade = Steps::build()
        //.label("FindAndExecuteTrade")
        //// ...move to the water source...
        //.step(FindTrade)
        //.step(FlyToTarget)
        //.step(ExecuteTrade);

        let thinker = Thinker::build()
            .label("WandererThinker")
            .picker(Highest {})
            .when(WantToWander, Idle { target: None }) // Always wander as we have set the score high.
            .when(
                WantToFlyToSystem,
                FlyToSystem {
                    target: None,
                    desire: 0.0,
                }
            )
            //.when(
            //    WantToTrade,
            //    find_and_execute_trade
            //)
            ;

        if let Some((_, solar_system, position)) = systems_with_positions.choose(&mut rng) {
            let mut spawn_position =
                random_position_in_system(Vec2::splat(512.0), position.translation);
            spawn_position.z = 0.1;
            let _e = commands
                .spawn((
                    SpriteBundle {
                        texture: textures.small_trader.clone().into(),
                        transform: Transform {
                            translation: spawn_position,
                            scale: Vec3::splat(1.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    PickableBundle::default(),
                    On::<Pointer<Down>>::send_event::<UpdateSelectedItemEvent>(),
                    Agent::new(0, String::from(name::full()), (*solar_system).clone()),
                    Idle::new(),
                    FlyToSystem {
                        target: None,
                        desire: 1.0,
                    },
                    thinker,
                    Name::new("Agent"),
                ))
                .id();
            // Assuming the Agent component has an `id` field that you want to update
        } else {
            //eprintln!("No SolarSystem entities found to spawn agent at!");
        }
    }

    state.set(GameState::Playing);
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
