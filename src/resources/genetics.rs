use bevy::prelude::*;
use crate::params::genetics::*;

#[derive(Resource)]
pub struct ElitePercent(pub f64);

impl Default for ElitePercent {
    fn default() -> Self {
        Self(ELITE_PERCENT)
    }
}

#[derive(Resource)]
pub struct MutationRate(pub f64);

impl Default for MutationRate {
    fn default() -> Self {
        Self(MUTATION_RATE)
    }
}