use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_yoleck::prelude::*;
use bevy_yoleck::vpeol::prelude::*;

use crate::animating::{AnimationsOwner, GetClipsFrom};
use crate::editing_helpers::SnapToGrid;
use crate::killing::Killable;
use crate::player::IsPlayer;
use crate::utils::sensor_events_both_ways;
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
        app.add_yoleck_edit_system(edit_gate_z_depth);
        app.yoleck_populate_schedule_mut().add_system(populate_gate);
        app.add_system(initiate_gate_opening.in_set(OnUpdate(AppState::Game)));
        app.add_system(pass_through_gate);
    }
}

#[derive(Component)]
struct Gate {
    is_open: bool,
}

#[derive(Component)]
pub struct KeepGatesClosedWhenAlive;

fn edit_gate_z_depth(mut edit: YoleckEdit<&mut Vpeol3dPosition, With<Gate>>) {
    let Ok(mut position) = edit.get_single_mut() else { return };
    position.0.z = -0.5;
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
            cmd.insert(RigidBody::Fixed);
            cmd.insert(Collider::cuboid(1.0, 1.5));
            cmd.insert(Sensor);
        }
    });
}

fn initiate_gate_opening(
    killables_query: Query<&Killable, With<KeepGatesClosedWhenAlive>>,
    mut gates_query: Query<(&mut Gate, &AnimationsOwner)>,
    mut animation_players_query: Query<&mut AnimationPlayer>,
) {
    for killable in killables_query.iter() {
        if killable.still_alive {
            return;
        }
    }
    for (mut gate, animations_owner) in gates_query.iter_mut() {
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

fn pass_through_gate(
    mut reader: EventReader<CollisionEvent>,
    players_query: Query<(), With<IsPlayer>>,
    gates_query: Query<&Gate>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (e1, e2) in sensor_events_both_ways(&mut reader) {
        let (Ok(_), Ok(gate)) = (
            players_query.get(e1),
            gates_query.get(e2),
        ) else { continue };
        if gate.is_open {
            next_state.set(AppState::LevelCompleted);
        }
    }
}
