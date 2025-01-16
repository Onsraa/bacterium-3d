mod components;
mod params;
mod plugins;
mod systems;
mod resources;
mod ui;

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
    let mut bacterium = Bacterium {
        diet: Diet::Vegetarian,
        nb_flagella: 10,
        speed: 2.0,
    };

    let mut bacterium2 = Bacterium {
        diet: Diet::Omnivore,
        nb_flagella: 10,
        speed: 2.0,
    };

    let mut bacterium3 = Bacterium {
        diet: Diet::Carnivore,
        nb_flagella: 10,
        speed: 2.0,
    };

    let entity = (
        bacterium,
        Mesh3d(meshes.add(Sphere::new(BACTERIUM_SPHERE_RADIUS * GLOBAL_SIZE_RATIO).mesh().uv(32, 18))),
        MeshMaterial3d(materials.add(bacterium_color(&bacterium.diet))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    );

    let entity2 = (
        bacterium2,
        Mesh3d(meshes.add(Sphere::new(BACTERIUM_SPHERE_RADIUS * GLOBAL_SIZE_RATIO).mesh().uv(32, 18))),
        MeshMaterial3d(materials.add(bacterium_color(&bacterium2.diet))),
        Transform::from_xyz(0.0, 0.0, -5.0),
    );

    let entity3 = (
        bacterium3,
        Mesh3d(meshes.add(Sphere::new(BACTERIUM_SPHERE_RADIUS * GLOBAL_SIZE_RATIO).mesh().uv(32, 18))),
        MeshMaterial3d(materials.add(bacterium_color(&bacterium3.diet))),
        Transform::from_xyz(0.0, 0.0, 5.0),
    );

    let parent = commands.spawn(entity).id();
    add_flagella(&mut commands, parent, &mut bacterium, &mut meshes, &mut materials);

    let parent = commands.spawn(entity2).id();
    add_flagella(&mut commands, parent, &mut bacterium2, &mut meshes, &mut materials);

    let parent = commands.spawn(entity3).id();
    add_flagella(&mut commands, parent, &mut bacterium3, &mut meshes, &mut materials);
}