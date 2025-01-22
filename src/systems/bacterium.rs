use crate::algorithms::bacterium_genome::*;
use crate::components::bacterium::*;
use crate::components::bacterium_genome::*;
use crate::components::diet::*;
use crate::components::health::*;
use crate::params::bacterium::*;
use crate::params::global::GLOBAL_SIZE_RATIO;
use crate::resources::bacterium::BacteriaData;
use crate::resources::simulation::SimulationData;
use crate::resources::window::*;
use crate::states::simulation::SimulationState;
use crate::systems::flagella::add_flagella;
use bevy::prelude::*;
use std::f32::consts::PI;

pub fn initialize_bacteria(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut bacteria_data: ResMut<BacteriaData>,
    mut simulation_data: ResMut<SimulationData>,
) {
    simulation_data.generation_count = 0;

    for _ in 0..bacteria_data.quantity {
        let bacterium_genome = BacteriumGenome::default();
        let health = Health::new(get_health(bacterium_genome.genome) as f32);
        let greed = get_greedy(bacterium_genome.genome) as f32;
        let energy = get_energy(bacterium_genome.genome) as f32;
        let nb_flagella = get_flagella(bacterium_genome.genome);
        let speed = BACTERIUM_SPEED * nb_flagella as f32;
        let diet = match get_feeding_mode(bacterium_genome.genome) {
            0 => Diet::Vegetarian,
            1 => Diet::Omnivore,
            2 => Diet::Carnivore,
            _ => Diet::None,
        };

        let bacterium = Bacterium {
            diet,
            nb_flagella,
            speed,
            greed,
            energy,
            active: true,
            target: None,
        };

        let parent = commands
            .spawn_empty()
            .insert(bacterium)
            .insert(health)
            .insert(Mesh3d(
                meshes.add(
                    Sphere::new(BACTERIUM_SPHERE_RADIUS * GLOBAL_SIZE_RATIO)
                        .mesh()
                        .uv(32, 18),
                ),
            ))
            .insert(MeshMaterial3d(
                materials.add(bacterium_color(&bacterium.diet)),
            ))
            .insert(Transform::default())
            .id();

        add_flagella(
            &mut commands,
            parent,
            &bacterium,
            &mut meshes,
            &mut materials,
        );
    }
}

pub fn spawn_bacteria(
    mut query: Query<&mut Transform, With<Bacterium>>,
    bacteria_data: Res<BacteriaData>,
    circle_radius: Res<CircleRadius>,
    window_size: Res<WindowSize>,
    mut next_state: ResMut<NextState<SimulationState>>,
) {
    let n = bacteria_data.quantity;
    if n == 0 {
        return;
    }
    let radius_max = 0.8 * window_size.width.min(window_size.height);
    let radius = circle_radius.0.min(radius_max);
    let center_y = 0.0;
    let center_z = 0.0;

    let angle_step = 2.0 * PI / (n as f32);

    let mut i = 0;
    for mut bacterium_transform in &mut query {
        let angle = i as f32 * angle_step;
        let y = center_y + radius * angle.cos();
        let z = center_z + radius * angle.sin();
        bacterium_transform.translation = Vec3::new(0.0, y, z);
        bacterium_transform.rotation = Quat::from_rotation_y(angle);

        i += 1;
    }
    next_state.set(SimulationState::Ready)
}

pub fn destroy_bacteria(mut commands: Commands, query: Query<Entity, With<Bacterium>>) {
    for bacterium in query.iter() {
        commands.entity(bacterium).despawn_recursive();
    }
}

pub fn bacterium_color(diet: &Diet) -> Color {
    match diet {
        Diet::Vegetarian => Color::srgb_u8(0, 255, 0),
        Diet::Omnivore => Color::srgb_u8(0, 0, 255),
        Diet::Carnivore => Color::srgb_u8(255, 0, 0),
        Diet::None => Color::srgb_u8(127, 127, 127),
    }
}
