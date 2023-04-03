use bevy::prelude::*;
use bevy_tnua::TnuaManualTurningOutput;

use crate::AppState;

pub struct AnimatingPlugin;

impl Plugin for AnimatingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (apply_actors_rotation, apply_rotate_around_axis).in_set(OnUpdate(AppState::Game)),
        );
    }
}

#[derive(Component)]
pub struct ApplyRotationToChild(pub Entity);

fn apply_actors_rotation(
    query: Query<(&TnuaManualTurningOutput, &ApplyRotationToChild)>,
    mut transform_query: Query<&mut Transform>,
) {
    for (manual_turning, ApplyRotationToChild(child_to_rotate)) in query.iter() {
        if 0.0 < manual_turning.forward.length_squared() {
            if let Ok(mut transform) = transform_query.get_mut(*child_to_rotate) {
                transform.look_to(manual_turning.forward, Vec3::Y);
            }
        }
    }
}

#[derive(Component)]
pub struct RotateAroundScaledAxis(pub Vec3);

fn apply_rotate_around_axis(time: Res<Time>, mut query: Query<(&RotateAroundScaledAxis, &mut Transform)>) {
    for (RotateAroundScaledAxis(scaled_axis), mut transform) in query.iter_mut() {
        transform.rotate(Quat::from_scaled_axis(time.delta_seconds() * *scaled_axis));
    }
}
