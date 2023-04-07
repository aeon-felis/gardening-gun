use bevy::prelude::*;
use bevy_yoleck::prelude::*;
use bevy_yoleck::vpeol::prelude::*;

use crate::animating::{AnimationsOwner, GetClipsFrom};
use crate::editing_helpers::SnapToGrid;
use crate::AppState;

pub struct GatePlugin;

impl Plugin for GatePlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_entity_type({
            YoleckEntityType::new("Gate")
                .with::<Vpeol3dPosition>()
                .insert_on_init(|| Gate { is_open: false })
                .insert_on_init_during_editor(|| SnapToGrid)
        });
        app.yoleck_populate_schedule_mut().add_system(populate_gate);
        app.add_system(initiate_gate_opening.in_set(OnUpdate(AppState::Game)));
    }
}

#[derive(Component)]
struct Gate {
    is_open: bool,
}

fn populate_gate(mut populate: YoleckPopulate<(), With<Gate>>, asset_server: Res<AssetServer>) {
    populate.populate(|ctx, mut cmd, ()| {
        if ctx.is_first_time() {
            cmd.insert(SceneBundle {
                scene: asset_server.load("Gate.glb#Scene0"),
                ..Default::default()
            });
            cmd.insert(VpeolWillContainClickableChildren);
            cmd.insert(AnimationsOwner::default());
            cmd.insert(GetClipsFrom(asset_server.load("Gate.glb")));
        }
    });
}

fn initiate_gate_opening(
    mut query: Query<(&mut Gate, &AnimationsOwner)>,
    mut animation_players_query: Query<&mut AnimationPlayer>,
) {
    // TODO: decide if the gates need to be opened
    for (mut gate, animations_owner) in query.iter_mut() {
        if gate.is_open {
            continue;
        }
        let Some(animation_clip) = animations_owner.clips.get("Open") else { continue };
        let Some(animation_player_entity) = animations_owner.players.get("GateOpener") else { continue };
        let Ok(mut animation_player) = animation_players_query.get_mut(*animation_player_entity) else { continue };
        animation_player.start(animation_clip.clone());
        gate.is_open = true;
    }
}
