use std::collections::HashSet;

use bevy::prelude::*;
use bevy::utils::uuid;
use rand::Rng; // Bring the trait into scope

use crate::solar_system::SolarSystem;
use crate::structures::stargate::Stargate;

use bevy_mod_picking::prelude::*;

use super::solar_system_generation::GalaxyConfig;

/// Spawns stargates between solar systems.
pub fn spawn_stargates(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    solar_systems: Query<(Entity, &Transform, &SolarSystem)>,
    config: Res<GalaxyConfig>,
) {
    println!("Spawning stargates");

    let all_systems = collect_all_solar_systems(&solar_systems);
    let mut established_connections: HashSet<(Entity, Entity)> = HashSet::new();


    for (system_entity, system_transform, solar_system) in solar_systems.iter() {
        println!("Spawning stargates for system: {:?}", solar_system.attributes.name);
        let num_stargates: usize = rand::thread_rng().gen_range(0..=2);
        for _ in 0..num_stargates {
            if let Some(destination_system) = select_destination_system(
                system_entity,
                &all_systems,
                &mut established_connections,
                system_transform,
            ) {
                spawn_stargate_pair(
                    &mut commands,
                    &asset_server,
                    (system_entity, system_transform, &solar_system),
                    destination_system,
                    &solar_systems,
                    &config,
                );
            }
        }
    }
}

/// Collect all solar systems into a Vec for potential destinations.
fn collect_all_solar_systems(
    solar_systems: &Query<(Entity, &Transform, &SolarSystem)>,
) -> Vec<(Entity, Vec3)> {
    solar_systems
        .iter()
        .map(|(entity, transform, _)| (entity, transform.translation))
        .collect()
}

/// Select a destination system for the stargate.
fn select_destination_system(
    system_entity: Entity,
    all_systems: &[(Entity, Vec3)],
    established_connections: &mut HashSet<(Entity, Entity)>,
    system_transform: &Transform,
) -> Option<Entity> {
    let mut rng = rand::thread_rng();
    let mut possible_destinations: Vec<(Entity, Vec3)> = all_systems
        .iter()
        .cloned()
        .filter(|&(entity, _)| {
            entity != system_entity
                && !established_connections.contains(&(system_entity, entity))
                && !established_connections.contains(&(entity, system_entity))
        })
        .collect();

    if possible_destinations.is_empty() {
        return None;
    }

    possible_destinations.sort_by(|&(_, a), &(_, b)| {
        a.distance_squared(system_transform.translation)
            .partial_cmp(&b.distance_squared(system_transform.translation))
            .unwrap()
    });

    let destination_system = possible_destinations[0].0;
    established_connections.insert((system_entity, destination_system));

    Some(destination_system)
}

/// Spawn a pair of stargates: one in the origin system and another in the destination system.
fn spawn_stargate_pair(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    origin_data: (Entity, &Transform, &SolarSystem),
    destination_system: Entity,
    solar_systems: &Query<(Entity, &Transform, &SolarSystem)>,
    config: &Res<GalaxyConfig>,
) {
    let (origin_system_entity, origin_system_transform, origin_solar_system) = origin_data;

    let (destination_system_entity, destination_system_transform, destination_solar_system) =
        solar_systems.get(destination_system).unwrap();

    // Generate properties for origin and destination stargates
    let (origin_stargate, destination_stargate) = generate_stargate_properties(
        origin_solar_system.attributes.id,
        destination_solar_system.attributes.id,
    );

    let origin_relative_stargate_position =
        get_relative_stargate_position(origin_system_transform, config);

    // Spawn origin stargate
    let origin_stargate_entity = spawn_stargate(
        commands,
        asset_server,
        &origin_relative_stargate_position,
        &origin_stargate,
    );

    let destination_relative_stargate_position =
        get_relative_stargate_position(destination_system_transform, config);

    // Spawn destination stargate
    spawn_stargate(
        commands,
        asset_server,
        &destination_relative_stargate_position,
        &destination_stargate,
    );

    // Optionally, draw a line between the stargates for visual representation
    create_line_between_stargates(
        commands,
        asset_server,
        &origin_relative_stargate_position,
        &destination_relative_stargate_position,
        origin_stargate_entity,
    );
}

/// Generate properties for a stargate.
fn generate_stargate_properties(
    origin_system_id: u32,
    destination_system_id: u32,
) -> (Stargate, Stargate) {
    let mut origin_stargate = Stargate {
        id: uuid::Uuid::new_v4().as_u128() as u32,
        name: "placeholder".to_string(), // "Stargate 1"
        distance: 100,
        destination_gate_id: 0,
        origin_system_id: origin_system_id,
        destination_system_id: destination_system_id,
        is_active: true,
    };

    let mut destination_stargate = Stargate {
        id: uuid::Uuid::new_v4().as_u128() as u32,
        name: "placeholder".to_string(), // "Stargate 2"
        distance: 100,
        destination_gate_id: origin_stargate.id,
        origin_system_id: destination_system_id,
        destination_system_id: origin_system_id,
        is_active: true,
    };

    origin_stargate.set_destination_gate_id(destination_stargate.id);
    origin_stargate.set_name(format!("Stargate {}", origin_stargate.id));
    destination_stargate.set_name(format!("Stargate {}", destination_stargate.id));

    (origin_stargate, destination_stargate)
}

/// Spawns a stargate entity with given properties and transform.
fn spawn_stargate(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    system_transform: &Transform,
    stargate: &Stargate,
) -> Entity {
    let texture_handle = asset_server.load("sprites/icons/systemGate.png");

    commands
        .spawn(SpriteBundle {
            texture: texture_handle,
            transform: *system_transform,
            ..Default::default()
        })
        .insert(stargate.clone()) // Assuming Stargate is cloneable. Otherwise, create a new instance.
        .insert(PickableBundle::default()) // Optional, for interactivity.
        .insert(Name::new(format!("Stargate {}", stargate.id))) // Optional, for debugging.
        .id()
}

fn get_relative_stargate_position(
    system_transform: &Transform,
    config: &Res<GalaxyConfig>,
) -> Transform {
    let relative_stargate_position =
        random_stargate_position(Vec2::splat(config.hex_size), Vec3::ZERO);

    let transform = Transform::from_xyz(
        system_transform.translation.x + relative_stargate_position.x,
        system_transform.translation.y + relative_stargate_position.y,
        0.0,
    )
    .with_scale(Vec3::splat(1.0));
    transform
}

/// Creates a visual line between two stargates.
fn create_line_between_stargates(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    origin_transform: &Transform,
    destination_transform: &Transform,
    parent_entity: Entity,
) {
    // Draw a line between the origin stargate and the destination stargate
    let midpoint = (origin_transform.translation + destination_transform.translation) * 0.5;
    let distance = origin_transform
        .translation
        .distance(destination_transform.translation);
    let angle = (destination_transform.translation.y - origin_transform.translation.y)
        .atan2(destination_transform.translation.x - origin_transform.translation.x);

    let _parent_global_transform = origin_transform; // Since origin_transform already appears to be global.
    let child_intended_global_transform = Transform {
        translation: midpoint,
        rotation: Quat::from_rotation_z(angle),
        scale: Vec3::new(distance, 1.0, 1.0),
    };
    // Inverse rotation
    let inverse_rotation = origin_transform.rotation.conjugate();

    // Inverse translation rotated by the inverse rotation
    let inverse_translation = inverse_rotation * -origin_transform.translation;

    let parent_inverse_transform = Transform {
        translation: inverse_translation,
        rotation: inverse_rotation,
        ..Default::default() // Assuming the scale is just 1, 1, 1
    };

    let local_transform = parent_inverse_transform * child_intended_global_transform;

    commands.entity(parent_entity).with_children(|parent| {
        parent
            .spawn(SpriteBundle {
                texture: asset_server.load("sprites/icons/line.png"),
                transform: Transform {
                    translation: local_transform.translation,
                    rotation: Quat::from_rotation_z(angle),
                    scale: Vec3::new(distance, 1.0, 1.0),
                },
                ..Default::default()
            })
            .insert(Name::new("Stargate Connection Line")); // Optional, for debugging.
    });
}

/// Returns a random position within a system when provided with the system's position and size.
fn random_stargate_position(hex_size: Vec2, system_position: Vec3) -> Vec3 {
    let buffer = hex_size.x * 0.5; // Using 1/4 of the hex size as buffer
    let random_x = rand::thread_rng().gen_range(
        (system_position.x - hex_size.x + buffer)..(system_position.x + hex_size.x - buffer),
    );
    let random_y = rand::thread_rng().gen_range(
        (system_position.y - hex_size.y + buffer)..(system_position.y + hex_size.y - buffer),
    );
    Vec3::new(random_x, random_y, system_position.z) // Keeping the z-coordinate the same
}
