use bevy::prelude::*;
use crate::components::diet::Diet;

pub fn bacterium_color(diet: &Diet) -> Color {
    match diet {
        Diet::Vegetarian => Color::srgb_u8(0, 255, 0),
        Diet::Omnivore => Color::srgb_u8(0, 0, 255),
        Diet::Carnivore => Color::srgb_u8(255, 0, 0),
        Diet::None => Color::srgb_u8(127, 127, 127),
    }
}