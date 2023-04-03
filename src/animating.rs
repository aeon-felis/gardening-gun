use bevy::prelude::*;
use bevy_tnua::TnuaManualTurningOutput;

use crate::AppState;

pub struct AnimatingPlugin;

impl Plugin for AnimatingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(apply_actors_rotation.in_set(OnUpdate(AppState::Game)));
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
