use std::f32::consts::PI;
use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tnua::{
    TnuaAnimatingState, TnuaFreeFallBehavior, TnuaManualTurningOutput,
    TnuaPlatformerAnimatingOutput, TnuaPlatformerBundle, TnuaPlatformerConfig,
    TnuaPlatformerControls, TnuaRapier2dSensorShape,
};
use bevy_yoleck::prelude::*;
use bevy_yoleck::vpeol::prelude::*;

use crate::ammunition::{CanCarry, CanPick};
use crate::animating::{AnimationsOwner, ApplyRotationToChild, GetClipsFrom};
use crate::editing_helpers::SnapToGrid;
use crate::killing::Killable;
use crate::shooting::CanShoot;
use crate::AppState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_entity_type({
            YoleckEntityType::new("Player")
                .with::<Vpeol3dPosition>()
                .insert_on_init(|| IsPlayer)
                .insert_on_init(|| Vpeol3dRotatation(Quat::from_rotation_y(PI)))
                .insert_on_init_during_editor(|| SnapToGrid)
        });
        app.yoleck_populate_schedule_mut()
            .add_system(populate_player);
        app.add_system(animate_player.in_set(OnUpdate(AppState::Game)));
    }
}

#[derive(Component)]
pub struct IsPlayer;

fn populate_player(
    mut populate: YoleckPopulate<(), With<IsPlayer>>,
    asset_server: Res<AssetServer>,
) {
    populate.populate(|ctx, mut cmd, ()| {
        if ctx.is_first_time() {
            cmd.insert(VpeolWillContainClickableChildren);
            let child = cmd
                .commands()
                .spawn(SceneBundle {
                    scene: asset_server.load("Player.glb#Scene0"),
                    ..Default::default()
                })
                .id();
            cmd.add_child(child);
            cmd.insert(ApplyRotationToChild(child));
            cmd.insert(AnimationsOwner::default());
            cmd.insert(GetClipsFrom(asset_server.load("Player.glb")));
        }
        cmd.insert(VisibilityBundle::default());
        cmd.insert(RigidBody::Dynamic);
        cmd.insert(Velocity::default());
        cmd.insert(Collider::capsule_y(0.5, 0.5));

        cmd.insert(TnuaPlatformerBundle::new_with_config(
            TnuaPlatformerConfig {
                full_speed: 12.0,
                full_jump_height: 4.0,
                up: Vec3::Y,
                forward: Vec3::X,
                float_height: 2.0,
                cling_distance: 1.0,
                spring_strengh: 400.0,
                spring_dampening: 1.4,
                acceleration: 40.0,
                air_acceleration: 20.0,
                coyote_time: 0.15,
                jump_start_extra_gravity: 30.0,
                jump_fall_extra_gravity: 20.0,
                jump_shorten_extra_gravity: 40.0,
                free_fall_behavior: TnuaFreeFallBehavior::LikeJumpShorten,
                tilt_offset_angvel: 5.0,
                tilt_offset_angacl: 500.0,
                turning_angvel: 30.0,
            },
        ));
        cmd.insert(TnuaPlatformerControls {
            desired_forward: Vec3::X,
            ..Default::default()
        });
        cmd.insert(LockedAxes::ROTATION_LOCKED);
        cmd.insert(TnuaRapier2dSensorShape(Collider::cuboid(0.45, 0.0)));
        cmd.insert(TnuaManualTurningOutput::default());
        cmd.insert(ActiveEvents::COLLISION_EVENTS);
        cmd.insert(CanPick);
        cmd.insert(CanCarry::default());
        cmd.insert(CanShoot::default());
        cmd.insert(SolverGroups {
            memberships: crate::solver_groups::PLAYER,
            filters: crate::solver_groups::PLANTED,
        });
        cmd.insert(Killable::default());
        cmd.insert(TnuaAnimatingState::<PlayerAnimationState>::default());
        cmd.insert(TnuaPlatformerAnimatingOutput::default());
    });
}

pub enum PlayerAnimationState {
    Standing,
    Running(f32),
    Jumping,
}

#[allow(clippy::type_complexity)]
fn animate_player(
    mut query: Query<(
        &mut TnuaAnimatingState<PlayerAnimationState>,
        &TnuaPlatformerAnimatingOutput,
        &Killable,
        &AnimationsOwner,
    )>,
    mut animation_players_query: Query<&mut AnimationPlayer>,
) {
    for (mut animating_state, animating_output, killable, animations_owner) in query.iter_mut() {
        if !killable.still_alive {
            // Death animation is handled elsewhere
            continue;
        }
        let Some(animation_player) = animations_owner.players.get("Armature") else { continue };
        let Ok(mut animation_player) = animation_players_query.get_mut(*animation_player) else { continue };
        match animating_state.update_by_discriminant({
            if animating_output.jumping_velocity.is_some() {
                PlayerAnimationState::Jumping
            } else {
                let speed = animating_output.running_velocity.length();
                if 0.01 < speed {
                    PlayerAnimationState::Running(0.1 * speed)
                } else {
                    PlayerAnimationState::Standing
                }
            }
        }) {
            bevy_tnua::TnuaAnimatingStateDirective::Maintain { state } => {
                if let PlayerAnimationState::Running(speed) = state {
                    animation_player.set_speed(*speed);
                }
            }
            bevy_tnua::TnuaAnimatingStateDirective::Alter {
                old_state: _,
                state,
            } => match state {
                PlayerAnimationState::Standing => {
                    let Some(clip) = animations_owner.clips.get("Standing") else { continue };
                    animation_player
                        .play_with_transition(clip.clone(), Duration::from_secs_f32(0.25))
                        .set_speed(1.0);
                }
                PlayerAnimationState::Running(speed) => {
                    let Some(clip) = animations_owner.clips.get("Running") else { continue };
                    animation_player
                        .play(clip.clone())
                        .repeat()
                        .set_speed(*speed);
                }
                PlayerAnimationState::Jumping => {
                    let Some(clip) = animations_owner.clips.get("Jumping") else { continue };
                    animation_player.play(clip.clone()).set_speed(3.0);
                }
            },
        }
    }
}
