use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use bevy_yoleck::prelude::*;
use bevy_yoleck::vpeol::prelude::*;
use serde::{Deserialize, Serialize};

pub struct EditingHelpersPlugin;

impl Plugin for EditingHelpersPlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_edit_system(apply_snap_to_grid);
        app.add_yoleck_edit_system(grid_resize);
    }
}

#[derive(Component)]
pub struct SnapToGrid;

#[derive(Clone, PartialEq, Serialize, Deserialize, Component, YoleckComponent)]
#[serde(transparent)]
pub struct GridSize(pub UVec2);

impl Default for GridSize {
    fn default() -> Self {
        Self(UVec2::new(1, 1))
    }
}

fn apply_snap_to_grid(
    mut edit: YoleckEdit<(&mut Vpeol3dPosition, Option<&GridSize>), With<SnapToGrid>>,
) {
    let Ok((mut position, size)) = edit.get_single_mut() else { return };
    let position = &mut position.0;
    let size = size.cloned().unwrap_or_default().0;
    for (coord, size_coord) in [(&mut position.x, size.x), (&mut position.y, size.y)] {
        let offset = size_coord as f32 * 0.5;
        *coord = (*coord - offset).round() + offset;
    }
}

fn grid_resize(
    mut edit: YoleckEdit<(&mut Vpeol3dPosition, &mut GridSize)>,
    mut knobs: YoleckKnobs,
    asset_server: Res<AssetServer>,
) {
    let Ok((mut position, mut size)) = edit.get_single_mut() else { return };

    for (i, diagonal) in [
        IVec2::new(1, 1),
        IVec2::new(-1, 1),
        IVec2::new(-1, -1),
        IVec2::new(1, -1),
    ]
    .into_iter()
    .enumerate()
    {
        let offset = 0.5 * diagonal.as_vec2() * size.0.as_vec2();
        let mut knob = knobs.knob(("resize-marker", i));
        if knob.is_new {
            knob.cmd.insert(VpeolWillContainClickableChildren);
            knob.cmd.insert(SceneBundle {
                scene: asset_server.load("ResizeMarker.glb#Scene0"),
                ..Default::default()
            });
        }
        knob.cmd.insert({
            Transform::from_translation(position.0 + offset.extend(0.0))
                .with_rotation(Quat::from_rotation_z(i as f32 * FRAC_PI_2))
        });

        if let Some(new_marker_pos) = knob.get_passed_data::<Vec3>() {
            let other_corner = position.0.truncate() - offset;
            let size_f = new_marker_pos.truncate() - other_corner;
            let size_i =
                IVec2::from_array(size_f.to_array().map(|coord| coord.round() as i32)) * diagonal;
            let size_u = UVec2::from_array(size_i.to_array().map(|coord| coord.max(1) as u32));
            size.0 = size_u;
            position.0 =
                (other_corner + 0.5 * diagonal.as_vec2() * size_u.as_vec2()).extend(position.0.z);
        }
    }
}
