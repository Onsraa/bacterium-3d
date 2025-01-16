use bevy::prelude::*;
use super::diet::Diet;

#[derive(Component, Clone, Copy)]
pub struct Bacterium {
    pub diet: Diet,
    pub nb_flagella: u32,
    pub speed: f32,
}