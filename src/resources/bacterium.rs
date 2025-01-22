use bevy::prelude::*;

#[derive(Resource)]
pub struct BacteriaData {
    pub quantity: u32,
    pub vegetarian_count: u32,
    pub omnivore_count: u32,
    pub carnivore_count: u32,
    pub alive_count: u32,
    // dead_count = quantity - alive_count
    pub avg_speed: f32,    // normalize to 1.0
    pub avg_vitality: f32, // normalize to 1.0
    pub avg_greed: f32,    // normalize to 1.0
}

impl Default for BacteriaData {
    fn default() -> Self {
        Self {
            quantity: 1,
            vegetarian_count: 0,
            omnivore_count: 0,
            carnivore_count: 0,
            alive_count: 0,
            avg_speed: 0.0,
            avg_vitality: 0.0,
            avg_greed: 0.0,
        }
    }
}

#[derive(Component)]
pub struct Flagella;
