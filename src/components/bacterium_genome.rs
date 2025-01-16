use crate::algorithms::bacterium_genome::*;
use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct BacteriumGenome {
    pub genome: Genome,
    pub score: f64,
}

impl BacteriumGenome {
    pub fn new(genome: Genome) -> Self {
        Self { genome, score: 0.0 }
    }
}

impl Default for BacteriumGenome {
    fn default() -> Self {
        Self::new(random_genome())
    }
}
