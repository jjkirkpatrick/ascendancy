use bevy::prelude::*;
use rand::Rng;
/// Returns a random position within a hex when provided with the hex's size and position.
/// Adds a 5% buffer around the edge.
pub fn random_position_in_hex(hex_size: Vec2, hex_position: Vec3) -> Vec3 {
    let vectors = [
        (-1.0f32, 0.0f32),
        (0.5f32, (3.0f32).sqrt() / 2.0f32),
        (0.5f32, -(3.0f32).sqrt() / 2.0f32),
    ];

    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..3) as usize;

    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..3);
    let v1 = vectors[x];
    let v2 = vectors[(x + 1) % 3];
    let (rand_x, rand_y) = (rng.gen::<f32>(), rng.gen::<f32>());
    let (random_x, random_y) = (rand_x * v1.0 + rand_y * v2.0, rand_x * v1.1 + rand_y * v2.1);

    // Add a 5% buffer around the edge
    let buffer = 0.1;
    let buffered_hex_size = Vec2::new(hex_size.x * (1.0 - buffer), hex_size.y * (1.0 - buffer));

    Vec3::new(
        random_x * buffered_hex_size.x as f32 + hex_position.x as f32,
        random_y * buffered_hex_size.y as f32 + hex_position.y as f32,
        hex_position.z as f32,
    )
}
