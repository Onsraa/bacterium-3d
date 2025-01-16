use bevy::prelude::*;
pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    let camera = (
        Camera3d::default(),
        Transform::from_xyz(20.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::ZERO),
    );

    let mut point_light = (
        PointLight::default(),
        Transform::from_xyz(8.0, 8.0, 0.0),
    );
    point_light.0.intensity = 10_000_000.0;

    commands.spawn(camera);
    commands.spawn(point_light);
}