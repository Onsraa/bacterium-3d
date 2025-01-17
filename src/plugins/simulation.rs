use bevy::prelude::*;
use crate::resources::bacterium::BacteriaData;
use crate::resources::simulation::SimulationData;
use crate::resources::window::*;
use crate::states::simulation::SimulationState;
use crate::systems::bacterium::*;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>();
        app.init_resource::<CircleRadius>();
        app.init_resource::<WindowSize>();
        app.init_resource::<BacteriaData>();
        app.init_resource::<SimulationData>();
        app.add_systems(Update, (destroy_bacteria, initialize_bacteria, spawn_bacteria).chain().run_if(in_state(SimulationState::Refreshing)));
    }
}