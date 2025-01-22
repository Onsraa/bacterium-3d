mod components;
mod params;
mod plugins;
mod systems;
mod resources;
mod ui;
mod algorithms;
mod states;

use bevy::prelude::*;
use crate::plugins::setup::SetupPlugin;
use crate::plugins::simulation::SimulationPlugin;
use crate::ui::plugin::UiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SetupPlugin)
        .add_plugins(SimulationPlugin)
        .add_plugins(UiPlugin)
        .add_systems(Update, move_camera)
        .run();
}

fn move_camera(
    timer: Res<Time>,
    mut query: Query<&mut Transform, With<PointLight>>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    let t = timer.delta_secs();
    let mut light = query.single_mut();
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        light.translation.x -= 10.0 * t;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        light.translation.x += 10.0 * t;
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        light.translation.y += 10.0 * t;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        light.translation.y -= 10.0 * t;
    }
}