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
                .insert_on_init(|| IsBlock)
        });
        app.yoleck_populate_schedule_mut()
            .add_system(populate_block);
    }
}

#[derive(Component)]
struct IsBlock;

fn populate_block(mut populate: YoleckPopulate<(), With<IsBlock>>, asset_server: Res<AssetServer>) {
    populate.populate(|ctx, mut cmd, ()| {
        if ctx.is_first_time() {
            cmd.insert(VpeolWillContainClickableChildren);
            cmd.insert(SceneBundle {
                scene: asset_server.load("Block.glb#Scene0"),
                ..Default::default()
            });
        }
    })
}
