use bevy::prelude::*;
use bevy_yoleck::vpeol::prelude::*;

pub struct GardeningGunCameraPlugin;

impl Plugin for GardeningGunCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera);
    }
}

pub fn setup_camera(mut commands: Commands) {
    let mut cmd = commands.spawn_empty();
    cmd.insert(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 10.0)
            .looking_to(Vec3::new(0.0, -3.0, -10.0), Vec3::Y),
        ..Default::default()
    });
    cmd.insert(VpeolCameraState::default());
    cmd.insert(Vpeol3dCameraControl::sidescroller());

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 50_000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 100.0, 0.0).looking_to(-Vec3::Y, Vec3::Z),
        ..Default::default()
    });
}
