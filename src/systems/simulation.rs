use crate::algorithms::bacterium_genome::{
    get_energy, get_feeding_mode, get_flagella, get_greedy, get_health, Genome,
};
use crate::algorithms::genetics::*;
use crate::components::bacterium::Bacterium;
use crate::components::bacterium_genome::BacteriumGenome;
use crate::components::diet::Diet;
use crate::components::health::Health;
use crate::params::bacterium::{BACTERIUM_SPEED, BACTERIUM_SPHERE_RADIUS};
use crate::params::global::GLOBAL_SIZE_RATIO;
use crate::resources::bacterium::{BacteriaData, Flagella};
use crate::resources::genetics::*;
use crate::resources::simulation::SimulationData;
use crate::resources::window::{CircleRadius, WindowSize};
use crate::states::simulation::SimulationState;
use crate::systems::bacterium::bacterium_color;
use crate::systems::flagella::add_flagella;
use bevy::prelude::*;
use std::cmp::Ordering;
use std::f32::consts::PI;

pub fn run_simulation(
    time: Res<Time>,
    mut simulation_data: ResMut<SimulationData>,
    mut next_state: ResMut<NextState<SimulationState>>,
) {
    simulation_data.generation_duration.tick(time.delta());
    if !simulation_data.generation_duration.finished() {
        return;
    }
    simulation_data.generation_count += 1;
    next_state.set(SimulationState::Updating);
}

pub fn update_genomes(
    mut query: Query<&mut BacteriumGenome>,
    bacteria_data: Res<BacteriaData>,
    elite_percent: Res<ElitePercent>,
    mutation_rate: Res<MutationRate>,
) {
    let n = bacteria_data.quantity as usize;
    if n == 0 {
        return;
    }

    // Récupération de tous les génomes + calcul du fitness
    let population: Vec<(Genome, f64)> = query
        .iter()
        .map(|bacterium_genome| {
            let g = bacterium_genome.genome;
            (g, calculate_fitness(g))
        })
        .collect();

    // Calcul du nombre d’élites
    let elite_count = ((elite_percent.0 * n as f64).round() as usize).min(n);

    let mut sorted_pop = population.clone();
    sorted_pop.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));

    let mut new_genomes = Vec::with_capacity(n);

    // 1) On conserve l’élite
    for i in 0..elite_count {
        new_genomes.push(sorted_pop[i].0);
    }

    // 2) On remplit le reste de la population par sélection + crossover + mutation
    while new_genomes.len() < n {
        // Sélection de 2 parents via la roulette
        let parents = roulette_wheel_selection(&population, 2);

        // Crossover
        let (mut child_a, mut child_b) = random_crossover(&parents[0], &parents[1]);

        // Mutation
        mutate(&mut child_a, mutation_rate.0);
        mutate(&mut child_b, mutation_rate.0);

        // Insertion des enfants dans le nouveau pool
        new_genomes.push(child_a);

        if new_genomes.len() < n {
            new_genomes.push(child_b);
        }
    }

    for (mut bacterium_genome, &new_g) in query.iter_mut().zip(new_genomes.iter()) {
        bacterium_genome.genome = new_g;
    }
}

pub fn update_bacteria(mut query: Query<(&BacteriumGenome, &mut Bacterium, &mut Health)>) {
    for (gen, mut bacterium, mut health) in query.iter_mut() {
        let genome = gen.genome;
        let new_health = get_health(genome) as f32;
        let greed = get_greedy(genome) as f32;
        let energy = get_energy(genome) as f32;
        let nb_flagella = get_flagella(genome);
        let speed = BACTERIUM_SPEED * nb_flagella as f32;

        let diet = match get_feeding_mode(genome) {
            0 => Diet::Vegetarian,
            1 => Diet::Omnivore,
            2 => Diet::Carnivore,
            _ => Diet::None,
        };

        bacterium.diet = diet;
        bacterium.nb_flagella = nb_flagella;
        bacterium.speed = speed;
        bacterium.greed = greed;
        bacterium.energy = energy;
        bacterium.diet = diet;
        bacterium.active = true;
        bacterium.target = None;

        health.max = new_health;
        health.current = new_health;
    }
}

#[test]
pub fn update_bacteria_alt(mut commands: Commands, query: Query<(Entity, &BacteriumGenome)>) {
    for (e, gen) in query.iter() {
        let genome = gen.genome;
        let health = Health::new(get_health(genome) as f32);
        let greed = get_greedy(genome) as f32;
        let energy = get_energy(genome) as f32;
        let nb_flagella = get_flagella(genome);
        let speed = BACTERIUM_SPEED * nb_flagella as f32;

        let diet = match get_feeding_mode(genome) {
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

        commands.entity(e).insert(bacterium).insert(health);
    }
}

pub fn update_models(
    mut commands: Commands,
    query: Query<(Entity, &Bacterium)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (parent, bacterium) in query.iter() {
        let diet: Diet = bacterium.diet;

        commands
            .entity(parent)
            .insert(Mesh3d(
                meshes.add(
                    Sphere::new(BACTERIUM_SPHERE_RADIUS * GLOBAL_SIZE_RATIO)
                        .mesh()
                        .uv(32, 18),
                ),
            ))
            .insert(MeshMaterial3d(materials.add(bacterium_color(&diet))));

        add_flagella(
            &mut commands,
            parent,
            &bacterium,
            &mut meshes,
            &mut materials,
        );
    }
}

pub fn update_positions(
    bacteria_data: Res<BacteriaData>,
    mut query: Query<&mut Transform, With<Bacterium>>,
    circle_radius: Res<CircleRadius>,
    window_size: Res<WindowSize>,
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
}

pub fn destroy_flagella(
    mut commands: Commands,
    query: Query<Entity, With<Flagella>>,
    mut next_state: ResMut<NextState<SimulationState>>,
) {
    for flagella in query.iter() {
        commands.entity(flagella).despawn_recursive();
    }
    next_state.set(SimulationState::Running);
}
