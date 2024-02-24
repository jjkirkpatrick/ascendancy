use std::collections::HashMap;
use std::collections::HashSet;

use bevy::math::vec2;
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::{mesh::Indices, render_resource::PrimitiveTopology};
use bevy::utils::uuid;
use hexx::*;
use rand::Rng; // Bring the trait into scope

use crate::faction::attributes::FactionID;
use crate::player_interactions::selection::UpdateSelectedItemEvent;
use crate::solar_system::attributes::SystemAttributes;
use crate::solar_system::SolarSystem;
use crate::structures::services::dock::Dock;
use crate::structures::services::market::Market;
use crate::structures::services::solar_generator::SolarGenerator;
use crate::structures::services::StationServices;
use crate::structures::stargate::JumpGates;
use crate::structures::stargate::Stargate;
use crate::structures::station::Station;

use bevy_mod_picking::prelude::*;

/// World size of the hexagons (outer radius)
const HEX_SIZE: f32 = 512.0;
/// The radius of the map.
const MAP_RADIUS: i32 = 10;

/// The map resource.
#[derive(Debug, Resource)]
pub struct Map {
    /// The layout of the map.
    pub layout: HexLayout,
    /// The entities in the map.
    pub entities: HashMap<Hex, Entity>,
}

/// Struct to hold the configuration for the galaxy.
#[derive(Resource)]
pub struct GalaxyConfig {
    hex_size: f32,
    map_radius: i32,
    proximity_threshold: i32,
    clump_centers: Vec<Hex>,
}

impl GalaxyConfig {

    pub fn default() -> Self {
        GalaxyConfig {
            hex_size: HEX_SIZE,
            map_radius: MAP_RADIUS,
            proximity_threshold: 3,
            clump_centers: vec![Hex::new(0, 0)],
        }
    }


    // A method to determine spawn chance based on hex proximity to clump centers
    fn spawn_chance_for_hex(&self, hex: Hex) -> f64 {
        let closest_distance_to_clump = self.clump_centers
            .iter()
            .map(|center| hex.distance_to(*center))
            .min()
            .unwrap_or(i32::MAX);

        if closest_distance_to_clump <= self.proximity_threshold {
            0.65
        } else {
            0.35
        }
    }
}

/// Creates all the solar systems in the galaxy.
pub fn create_galaxy_solar_systems(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config: Res<GalaxyConfig>, // Use GalaxyConfig as a resource
) {
    let layout = HexLayout {
        hex_size: vec2(config.hex_size, config.hex_size),
        ..default()
    };

    let mesh_handle = meshes.add(hexagonal_plane(&layout));
    let mut rng = rand::thread_rng();

    // Use the configuration to adjust galaxy generation logic
    let entities = Hex::ZERO
        .spiral_range(0..=(config.map_radius / 2)as u32)
        .filter_map(|hex| {
            let pos = layout.hex_to_world_pos(hex);
            let spawn_chance = config.spawn_chance_for_hex(hex);

            if rng.gen_bool(spawn_chance) {
                Some(spawn_solar_system_entity(&mut commands, &mesh_handle, pos, hex))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();

    commands.insert_resource(Map { layout, entities });
}

/// Function to encapsulate solar system entity spawning logic.
fn spawn_solar_system_entity(
    commands: &mut Commands,
    mesh_handle: &Handle<Mesh>,
    pos: Vec2,
    hex: Hex,
) -> (Hex, Entity) {
    let entity_id = commands
        .spawn((
            SolarSystem {
                attributes: SystemAttributes {
                    id: uuid::Uuid::new_v4().as_u128() as u32,
                    name: "Placeholder".to_string(),
                    owner: FactionID { id: 0 },
                },
                jumpgates: JumpGates::default(),
            },
            PickableBundle::default(),
            On::<Pointer<Down>>::send_event::<UpdateSelectedItemEvent>(),
            ColorMesh2dBundle {
                transform: Transform::from_xyz(pos.x, pos.y, -1.0)
                    .with_scale(Vec3::splat(0.9)),
                mesh: mesh_handle.clone().into(),
                ..default()
            },
            Name::new(format!("System - {},{}", hex.x, hex.y)),
        ))
        .with_children(|parent| {
            parent.spawn(Text2dBundle {
                text: Text::from_section(
                    format!("{},{}", hex.x, hex.y),
                    TextStyle {
                        font_size: 64.0,
                        color: Color::BLACK,
                        ..Default::default()
                    },
                ),
                transform: Transform::from_xyz(0.0, 390.0, 10.0),
                ..Default::default()
            });
        })
        .id();
    (hex, entity_id)
}

/// Spawns stargates between solar systems.
pub fn spawn_stargates(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    solar_systems: Query<(Entity, &Transform, &SystemAttributes, &mut JumpGates), With<JumpGates>>,
) {
    println!("Spawning stargates");

    // Step 1: Create origin stargates with placeholder destinations.
    let mut rng = rand::thread_rng();
    let all_systems: Vec<(Entity, Vec3)> = solar_systems
        .iter()
        .map(|(entity, transform, _, _)| (entity, transform.translation))
        .collect();
    let mut established_connections: HashSet<(Entity, Entity)> = HashSet::new();

    let mut star_gate_count = 0;

    for (system_entity, system_transform, solar_system_attributes, _) in solar_systems.iter() {
        let num_stargates: usize = rng.gen_range(0..=2);
        for _ in 0..num_stargates {
            let mut possible_destinations = all_systems.clone();

            // Ensure no duplicate connections are formed.
            possible_destinations.retain(|&(x, _)| {
                x != system_entity
                    && !established_connections.contains(&(system_entity, x))
                    && !established_connections.contains(&(x, system_entity))
            });

            // If there are no more valid destinations, break.
            if possible_destinations.is_empty() {
                break;
            }

            possible_destinations.sort_by(|&(_, a), &(_, b)| {
                a.distance_squared(system_transform.translation)
                    .partial_cmp(&b.distance_squared(system_transform.translation))
                    .unwrap()
            });
            let destination_system = possible_destinations[0].0;

            let (_, _, destination_system_attributes, _) =
                solar_systems.get(destination_system).unwrap();

            established_connections.insert((system_entity, destination_system));

            let relative_stargate_position = random_stargate_position(Vec2::splat(HEX_SIZE), Vec3::ZERO);

            let origin_transform = Transform::from_xyz(
                system_transform.translation.x + relative_stargate_position.x,
                system_transform.translation.y + relative_stargate_position.y,
                0.0,
            )
            .with_scale(Vec3::splat(1.0));

            let origin_stargate_id = star_gate_count;
            star_gate_count += 1;
            let destination_stargate_id = star_gate_count;
            star_gate_count += 1;

            let origin_system_gate = Stargate {
                id: origin_stargate_id,
                name: format!(
                    "Stargate connected to  {:?} from {:?}",
                    destination_stargate_id, origin_stargate_id
                ),
                distance: 10000,
                destination_gate_id: destination_stargate_id, // Using placeholder for now
                origin_system_id: solar_system_attributes.id,
                destination_system_id: destination_system_attributes.id,
                is_active: true,
            };

            let destination_system_gate = Stargate {
                id: destination_stargate_id,
                name: format!(
                    "Stargate connected to {:?} from {:?}",
                    origin_stargate_id, destination_stargate_id
                ),
                distance: 10000,
                destination_gate_id: origin_stargate_id, // Using placeholder for now
                origin_system_id: destination_system_attributes.id,
                destination_system_id: solar_system_attributes.id,
                is_active: true,
            };

            let origin_stargate = commands
                .spawn((
                    SpriteBundle {
                        texture: asset_server.load("sprites/icons/systemGate.png"),
                        transform: origin_transform,
                        ..Default::default()
                    },
                    origin_system_gate,
                    PickableBundle::default(),
                    //RaycastPickTarget::default(),
                    On::<Pointer<Down>>::send_event::<UpdateSelectedItemEvent>(),
                    Name::new(format!(
                        "Stargate connected to  {:?} from {:?}",
                        destination_stargate_id, origin_stargate_id
                    )),
                ))
                .id();

            let (_, dest_system_transform, _, _) = solar_systems.get(destination_system).unwrap();
            let relative_dest_position = random_stargate_position(Vec2::splat(HEX_SIZE), Vec3::ZERO);

            let dest_transform = Transform::from_xyz(
                dest_system_transform.translation.x + relative_dest_position.x,
                dest_system_transform.translation.y + relative_dest_position.y,
                0.0,
            )
            .with_scale(Vec3::splat(1.0));

            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("sprites/icons/systemGate.png"),
                    transform: dest_transform,
                    ..Default::default()
                },
                destination_system_gate,
                PickableBundle::default(),
                On::<Pointer<Down>>::send_event::<UpdateSelectedItemEvent>(),
                Name::new(format!(
                    "Stargate connected to {:?} from {:?}",
                    origin_stargate_id, destination_stargate_id
                )),
            ));

            // Draw a line between the origin stargate and the destination stargate
            let midpoint = (origin_transform.translation + dest_transform.translation) * 0.5;
            let distance = origin_transform
                .translation
                .distance(dest_transform.translation);
            let angle = (dest_transform.translation.y - origin_transform.translation.y)
                .atan2(dest_transform.translation.x - origin_transform.translation.x);

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

            commands
                .entity(origin_stargate)
                .with_children(|b: &mut ChildBuilder| {
                    b.spawn((
                        SpriteBundle {
                            texture: asset_server.load("sprites/icons/line.png"),
                            transform: Transform {
                                translation: local_transform.translation,
                                rotation: Quat::from_rotation_z(angle),
                                scale: Vec3::new(distance, 1.0, 1.0),
                            },
                            ..Default::default()
                        },
                        Name::new(format!("Line for {:?}", system_entity)),
                    ));
                });
        }
    }
}

pub fn spawn_space_station(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    solar_systems: Query<(&Transform, &SystemAttributes), With<SystemAttributes>>,
) {
    for (system_transform, system_attributes) in solar_systems.iter() {
        let mut station = Station::new(
            system_attributes.id as u32,
            format!("Station {}", system_attributes.id),
            system_attributes.id,
        );

        station
            .add_service(StationServices::Market(Market::new()))
            .unwrap();
        station
            .add_service(StationServices::Dock(Dock::new(
                String::from("Docking Bay 1"),
                20,
            )))
            .unwrap();
        station
            .add_service(StationServices::SolarGenerator(SolarGenerator::new(
                String::from("Solar Generator 1"),
            )))
            .unwrap();

        commands.spawn((
            station,
            PickableBundle::default(),
            On::<Pointer<Down>>::send_event::<UpdateSelectedItemEvent>(),
            SpriteBundle {
                texture: asset_server.load("sprites/icons/structures/station_1.png"),
                transform: Transform {
                    translation: Vec3::new(
                        system_transform.translation.x,
                        system_transform.translation.y,
                        system_transform.translation.z.max(1.0),
                    ),
                    ..Default::default()
                },
                ..Default::default()
            },
            Name::new(format!("Station {}", system_attributes.id)),
        ));
    }
}

/// Returns a mesh for a hexagonal plane.
fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(hex_layout).facing(Vec3::Z).build();
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs);
    mesh.insert_indices(Indices::U16(mesh_info.indices));
    mesh
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
