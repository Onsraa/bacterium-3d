use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub enum Diet {
    Vegetarian,
    Omnivore,
    Carnivore,
    None
}