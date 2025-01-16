use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct BacteriaData {
    quantity: u32,
    vegetarian_count: u32,
    omnivore_count: u32,
    carnivore_count: u32,
    alive_count: u32,
    // dead_count = quantity - alive_count
    avg_speed: f32, // normalize to 1.0
    avg_vitality: f32, // normalize to 1.0
    avg_greed: f32, // normalize to 1.0
}