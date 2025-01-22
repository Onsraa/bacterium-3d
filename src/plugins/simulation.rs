use bevy::prelude::*;
use crate::resources::bacterium::BacteriaData;
use crate::resources::genetics::*;
use crate::resources::simulation::SimulationData;
use crate::resources::window::*;
use crate::states::simulation::SimulationState;
use crate::systems::bacterium::*;
use crate::systems::simulation::*;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>();
        app.init_resource::<ElitePercent>();
        app.init_resource::<MutationRate>();
        app.init_resource::<CircleRadius>();
        app.init_resource::<WindowSize>();
        app.init_resource::<BacteriaData>();
        app.init_resource::<SimulationData>();
        app.add_systems(Update, (destroy_bacteria, initialize_bacteria, spawn_bacteria).chain().run_if(in_state(SimulationState::Refreshing)));
        app.add_systems(Update, run_simulation.run_if(in_state(SimulationState::Running)));
        app.add_systems(Update, (destroy_flagella, update_genomes, update_bacteria, update_models, update_positions).chain().run_if(in_state(SimulationState::Updating)));
    }
}