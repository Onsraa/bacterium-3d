use std::time::Duration;
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use egui::ProgressBar;
use crate::resources::bacterium::BacteriaData;
use crate::resources::genetics::{ElitePercent, MutationRate};
use crate::resources::simulation::SimulationData;
use crate::states::simulation::SimulationState;

pub fn ui_simulation_panel(
    mut commands: Commands,
    mut bacteria_data: ResMut<BacteriaData>,
    mut simulation_data: ResMut<SimulationData>,
    mut elite_percent: ResMut<ElitePercent>,
    mut mutation_rate: ResMut<MutationRate>,
    state: Res<State<SimulationState>>,
    mut next_state: ResMut<NextState<SimulationState>>,
    mut contexts: EguiContexts,
) {
    egui::Window::new("Simulation panel").show(contexts.ctx_mut(), |ui| {
        if let SimulationState::Ready = state.get() {
            ui.collapsing("Parameters", |ui| {
                let slider_bacteria_quantity = egui::Slider::new(&mut bacteria_data.quantity, 1..=120).text("Number of Bacteria");
                if ui.add(slider_bacteria_quantity).changed() {
                    next_state.set(SimulationState::Refreshing)
                }
                let mut gen_timer_ms = simulation_data.generation_duration.duration().as_millis() as u64;
                let slider_gen_timer = egui::Slider::new(&mut gen_timer_ms, 20..=60000)
                    .text("Generation Duration (ms)");
                if ui.add(slider_gen_timer).changed() {
                    simulation_data.generation_duration.set_duration(Duration::from_millis(gen_timer_ms));
                    simulation_data.generation_duration.reset();
                }
            });
            ui.add_space(10.0);
            ui.collapsing("Algorithms parameters", |ui| {
                ui.add(egui::Slider::new(&mut elite_percent.0, 0.0..=1.0).text("Elite Percent"));
                ui.add(egui::Slider::new(&mut mutation_rate.0, 0.0..=0.2).text("Mutation Rate"));
            });
            ui.add_space(10.0);
        }
        ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
            match state.get() {
                SimulationState::Ready => {
                    if ui.button("Start").clicked() {
                        next_state.set(SimulationState::Running)
                    }
                },
                _ => {}
            }
            match state.get() {
                SimulationState::Running => {
                    let button_name = "Pause";
                    if ui.button(button_name).clicked() {
                        next_state.set(SimulationState::Paused)
                    }
                },
                SimulationState::Updating => {
                    let button_name = "Pause";
                    if ui.button(button_name).clicked() {
                        next_state.set(SimulationState::Paused)
                    }
                },
                SimulationState::Paused => {
                    let button_name = "Run";
                    if ui.button(button_name).clicked() {
                        next_state.set(SimulationState::Running)
                    }
                },
                _ => {}
            }
            if ui.button("Reset").clicked() {
                next_state.set(SimulationState::Refreshing);
                simulation_data.generation_duration.reset();
            }
        });
        ui.add_space(10.0);
        ui.label(format!("Generation n°{}", simulation_data.generation_count));
        ui.add_space(10.0);
        ui.add(ProgressBar::new(simulation_data.generation_duration.elapsed().as_secs_f32() / simulation_data.generation_duration.duration().as_secs_f32())
            .text(format!("Simulation duration : [{}s/{}s]", simulation_data.generation_duration.elapsed().as_secs(), simulation_data.generation_duration.duration().as_secs())));
    });
}