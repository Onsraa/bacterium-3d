use std::time::Duration;
use bevy::prelude::*;
use crate::params::simulation::GENERATION_DURATION;

#[derive(Resource)]
pub struct SimulationData {
    generation_count: u32,
    generation_duration: Timer,
}

impl Default for SimulationData {
    fn default() -> Self {
        Self {
            generation_count: 0,
            generation_duration: Timer::new(Duration::from_millis(GENERATION_DURATION), TimerMode::Repeating),
        }
    }
}