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
        .run();
}