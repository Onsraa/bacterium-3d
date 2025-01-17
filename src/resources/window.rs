use bevy::prelude::*;
use crate::params::windows::*;

#[derive(Resource)]
pub struct CircleRadius(pub f32);

impl Default for CircleRadius {
    fn default() -> Self {
        Self(DEFAULT_CIRCLE_RADIUS)
    }
}

#[derive(Resource)]
pub struct WindowSize {
    pub width: f32,
    pub height: f32,
}

impl Default for WindowSize {
    fn default() -> Self {
        Self {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
        }
    }
}