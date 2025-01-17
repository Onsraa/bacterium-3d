use bevy::prelude::*;
use super::bacterium_genome::BacteriumGenome;
use super::health::Health;
use super::diet::Diet;
use super::edible::Edible;

#[derive(Component, Clone, Copy, Debug)]
#[require(BacteriumGenome, Health)]
pub struct Bacterium {
    pub diet: Diet,
    pub nb_flagella: u32,
    pub speed: f32,
    pub greed: f32,
    pub energy: f32,
    pub active: bool,
    pub target: Option<Edible>,
}