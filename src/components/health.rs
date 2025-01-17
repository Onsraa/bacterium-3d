use bevy::prelude::*;

#[derive(Component, Default, Debug, Clone)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(health: f32) -> Self {
        Self {
            current: health,
            max: health,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }
}
