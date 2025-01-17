use bevy::prelude::*;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Diet {
    Vegetarian,
    Omnivore,
    Carnivore,
    #[default]
    None
}