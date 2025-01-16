use crate::params::global::GLOBAL_SIZE_RATIO;
use crate::params::bacterium::*;
use crate::params::flagella::*;

use crate::components::bacterium::*;
use crate::components::diet::*;
use crate::components::flagella::*;

use crate::systems::bacterium::*;

use bevy::prelude::*;

pub fn add_flagella(
    commands: &mut Commands,
    parent: Entity,
    bacterium: &mut Bacterium,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let directions = fibonacci_sphere_directions(bacterium.nb_flagella);

    let flagella_mesh = match flagella_shape_from_diet(&bacterium.diet) {
        FlagellaShape::Sphere => {
            Some(Mesh3d(meshes.add(Sphere::new(FLAGELLA_SPHERE_RADIUS * GLOBAL_SIZE_RATIO).mesh().uv(32, 18))))
        },
        FlagellaShape::Cube => {
            Some(Mesh3d(meshes.add(Cuboid::new(FLAGELLA_CUBOID_X * GLOBAL_SIZE_RATIO, FLAGELLA_CUBOID_Y * GLOBAL_SIZE_RATIO, FLAGELLA_CUBOID_Z * GLOBAL_SIZE_RATIO))))
        },
        FlagellaShape::Cone => {
            Some(Mesh3d(meshes.add(Cone::new(FLAGELLA_CONE_RADIUS * GLOBAL_SIZE_RATIO, FLAGELLA_CONE_HEIGHT * GLOBAL_SIZE_RATIO))))
        },
        FlagellaShape::None => None,
    };

    let flagella_mesh = match flagella_mesh {
        Some(mesh) => mesh,
        None => return,
    };

    let material = MeshMaterial3d(materials.add(bacterium_color(&bacterium.diet)));

    for dir in directions {
        let translation = dir * BACTERIUM_SPHERE_RADIUS * GLOBAL_SIZE_RATIO;
        let rotation = Quat::from_rotation_arc(Vec3::Y, dir);
        let transform = Transform {
            translation,
            rotation,
            ..Default::default()
        };
        let cone_entity = commands.spawn((
            flagella_mesh.clone(),
            material.clone(),
            transform,
        )).id();
        commands.entity(parent).add_child(cone_entity);
    }
}

fn fibonacci_sphere_directions(nb: u32) -> Vec<Vec3> {
    let mut dirs = Vec::with_capacity(nb as usize);
    let golden_angle = std::f32::consts::PI * (3.0 - 5.0_f32.sqrt());

    for i in 0..nb {
        let y = 1.0 - (2.0 * i as f32) / (nb - 1) as f32;
        let radius = (1.0 - y * y).sqrt();

        let theta = golden_angle * i as f32;

        let x = theta.cos() * radius;
        let z = theta.sin() * radius;

        dirs.push(Vec3::new(x, y, z));
    }
    dirs
}

fn flagella_shape_from_diet(diet: &Diet) -> FlagellaShape {
    match diet {
        Diet::Vegetarian => FlagellaShape::Sphere,
        Diet::Omnivore => FlagellaShape::Cube,
        Diet::Carnivore => FlagellaShape::Cone,
        Diet::None => FlagellaShape::None,
    }
}