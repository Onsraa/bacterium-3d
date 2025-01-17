use bevy::prelude::*;
use super::health::*;
use crate::params::food::FEEDING_VALUE;

#[derive(Component)]
#[require(Health)]
pub struct Food {
    pub feeding_value: f32,
}

impl Default for Food {
    fn default() -> Self {
        Self { feeding_value: FEEDING_VALUE }
    }
}
