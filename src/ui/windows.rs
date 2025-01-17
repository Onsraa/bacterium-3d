use std::time::Duration;
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use egui::ProgressBar;
use crate::resources::bacterium::BacteriaData;
use crate::resources::simulation::SimulationData;
use crate::states::simulation::SimulationState;

pub fn ui_simulation_panel(
    mut commands: Commands,
    mut bacteria_data: ResMut<BacteriaData>,
    mut simulation_data: ResMut<SimulationData>,
    state: Res<State<SimulationState>>,
    mut next_state: ResMut<NextState<SimulationState>>,
    mut contexts: EguiContexts,
) {
    egui::Window::new("Simulation panel").show(contexts.ctx_mut(), |ui| {
        if let SimulationState::Ready = state.get() {
            ui.collapsing("Parameters", |ui| {
                let slider_bacteria_quantity = egui::Slider::new(&mut bacteria_data.quantity, 1..=100).text("Number of Bacteria");
                if ui.add(slider_bacteria_quantity).changed() {
                    next_state.set(SimulationState::Refreshing)
                }
            });
        }
    });
}