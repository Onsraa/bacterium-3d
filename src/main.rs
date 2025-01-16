mod components;
mod params;
mod plugins;
mod systems;
mod resources;
mod ui;
mod algorithms;

use bevy::prelude::*;
use crate::components::bacterium::Bacterium;
use crate::components::diet::Diet;
use crate::params::bacterium::BACTERIUM_SPHERE_RADIUS;
use crate::params::global::GLOBAL_SIZE_RATIO;
use crate::plugins::setup::SetupPlugin;
use crate::systems::bacterium::bacterium_color;
use crate::systems::flagella::add_flagella;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SetupPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let mut bacteria: Vec<Bacterium> = vec![];

    bacteria.push(Bacterium {
        diet: Diet::Vegetarian,
        nb_flagella: 10,
        speed: 2.0,
    });
    bacteria.push(Bacterium {
        diet: Diet::Omnivore,
        nb_flagella: 10,
        speed: 2.0,
    });
    bacteria.push(Bacterium {
        diet: Diet::Carnivore,
        nb_flagella: 10,
        speed: 2.0,
    });
    bacteria.push(Bacterium {
        diet: Diet::None,
        nb_flagella: 10,
        speed: 2.0,
    });

    let mut y = -5.0;

    for mut bacterium in bacteria.iter_mut() {
        let parent = commands.spawn_empty()
            .insert(*bacterium)
            .insert(Mesh3d(meshes.add(Sphere::new(BACTERIUM_SPHERE_RADIUS * GLOBAL_SIZE_RATIO).mesh().uv(32, 18))))
            .insert(MeshMaterial3d(materials.add(bacterium_color(&bacterium.diet))))
            .insert(Transform::from_xyz(0.0, y, 0.0))
            .id();
        add_flagella(&mut commands, parent, &mut bacterium, &mut meshes, &mut materials);
        y += 5.0;
    }
}