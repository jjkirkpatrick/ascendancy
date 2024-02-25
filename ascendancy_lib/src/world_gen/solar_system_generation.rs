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
use crate::solar_system::EntityList;
use crate::solar_system::SolarSystem;
use crate::structures::services::dock::Dock;
use crate::structures::services::market::Market;
use crate::structures::services::solar_generator::SolarGenerator;
use crate::structures::services::StationServices;
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
    /// The size of the hexagons.
    pub hex_size: f32,
    /// The radius of the map.
    pub map_radius: i32,
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
        let closest_distance_to_clump = self
            .clump_centers
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
        .spiral_range(0..=(config.map_radius / 2) as u32)
        .filter_map(|hex| {
            let pos = layout.hex_to_world_pos(hex);
            let spawn_chance = config.spawn_chance_for_hex(hex);

            if rng.gen_bool(spawn_chance) {
                Some(spawn_solar_system_entity(
                    &mut commands,
                    &mesh_handle,
                    pos,
                    hex,
                ))
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
                entities: EntityList::default(),
            },
            PickableBundle::default(),
            On::<Pointer<Down>>::send_event::<UpdateSelectedItemEvent>(),
            ColorMesh2dBundle {
                transform: Transform::from_xyz(pos.x, pos.y, -1.0).with_scale(Vec3::splat(0.9)),
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
pub fn spawn_space_station(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    solar_systems: Query<(&Transform, &SolarSystem)>,
) {
    for (system_transform, solar_system) in solar_systems.iter() {
        let system_attributes = &solar_system.attributes;
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
