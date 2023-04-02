use bevy::prelude::*;
use bevy_yoleck::prelude::*;
use bevy_yoleck::vpeol::prelude::*;

use crate::editing_helpers::{GridSize, SnapToGrid};

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_entity_type({
            YoleckEntityType::new("Block")
                .with::<Vpeol3dPosition>()
                .with::<GridSize>()
                .insert_on_init_during_editor(|| VpeolDragPlane::XY)
                .insert_on_init_during_editor(|| SnapToGrid)
                .insert_on_init_during_editor(PreviousSize::default)
                .insert_on_init(|| IsBlock)
        });
        app.yoleck_populate_schedule_mut()
            .add_system(populate_block);
    }
}

#[derive(Component)]
struct IsBlock;

#[derive(Component, Default)]
struct PreviousSize(UVec2);

fn populate_block(
    mut populate: YoleckPopulate<(&GridSize, Option<&mut PreviousSize>), With<IsBlock>>,
    asset_server: Res<AssetServer>,
    marking: YoleckMarking,
) {
    populate.populate(|_ctx, mut cmd, (size, mut previous_size)| {
        let should_populate = if let Some(previous_size) = previous_size.as_mut() {
            if size.0 == previous_size.0 {
                false
            } else {
                previous_size.0 = size.0;
                true
            }
        } else {
            // Not in editor
            true
        };
        if should_populate {
            let botright = -0.5 * size.0.as_vec2();

            marking.despawn_marked(&mut cmd);
            cmd.insert(VpeolWillContainClickableChildren);
            cmd.insert(VisibilityBundle::default());
            cmd.with_children(|commands| {
                for row in 0..size.0.y {
                    for col in 0..size.0.x {
                        let pos = botright + Vec2::new(0.5 + col as f32, 0.5 + row as f32);
                        commands.spawn((
                            SceneBundle {
                                scene: asset_server.load("Block.glb#Scene0"),
                                transform: Transform::from_translation(pos.extend(0.0)),
                                ..Default::default()
                            },
                            marking.marker(),
                        ));
                    }
                }
            });
        }
    })
}
