use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tnua::{
    TnuaFreeFallBehavior, TnuaManualTurningOutput, TnuaPlatformerBundle, TnuaPlatformerConfig,
    TnuaPlatformerControls, TnuaRapier2dSensorShape,
};
use bevy_yoleck::prelude::*;
use bevy_yoleck::vpeol::prelude::*;

use crate::animating::{AnimationsOwner, ApplyRotationToChild, GetClipsFrom, InitialAnimation};
use crate::editing_helpers::SnapToGrid;
use crate::gate::KeepGatesClosedWhenAlive;
use crate::killing::{KillEvent, Killable};
use crate::player::IsPlayer;
use crate::shooting::{Bullet, DestroysBullets};
use crate::utils::events_both_ways;
use crate::AppState;

pub struct GoblinPlugin;

impl Plugin for GoblinPlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_entity_type({
            YoleckEntityType::new("Goblin")
                .with::<Vpeol3dPosition>()
                .insert_on_init(|| IsGoblin)
                .insert_on_init(|| Vpeol3dRotatation(Quat::from_rotation_y(PI)))
                .insert_on_init_during_editor(|| SnapToGrid)
        });
        app.yoleck_populate_schedule_mut()
            .add_system(populate_goblin);
        app.add_system(handle_goblin_hitting_stuff);
        app.add_system(goblins_face_player.in_set(OnUpdate(AppState::Game)));
    }
}

#[derive(Component)]
pub struct IsGoblin;

fn populate_goblin(
    mut populate: YoleckPopulate<(), With<IsGoblin>>,
    asset_server: Res<AssetServer>,
) {
    populate.populate(|ctx, mut cmd, ()| {
        if ctx.is_first_time() {
            cmd.insert(VpeolWillContainClickableChildren);
            let child = cmd
                .commands()
                .spawn(SceneBundle {
                    scene: asset_server.load("Goblin.glb#Scene0"),
                    ..Default::default()
                })
                .id();
            cmd.add_child(child);
            cmd.insert(ApplyRotationToChild(child));
        }
        cmd.insert(VisibilityBundle::default());
        cmd.insert(RigidBody::Dynamic);
        cmd.insert(Velocity::default());
        cmd.insert(Collider::capsule_y(0.7, 0.5));

        cmd.insert(TnuaPlatformerBundle::new_with_config(
            TnuaPlatformerConfig {
                full_speed: 12.0,
                full_jump_height: 4.0,
                up: Vec3::Y,
                forward: Vec3::X,
                float_height: 1.2,
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
                turning_angvel: 10.0,
            },
        ));
        cmd.insert(LockedAxes::ROTATION_LOCKED);
        cmd.insert(TnuaRapier2dSensorShape(Collider::cuboid(0.45, 0.0)));
        cmd.insert(TnuaManualTurningOutput::default());
        cmd.insert(ActiveEvents::COLLISION_EVENTS);
        cmd.insert(AnimationsOwner::default());
        cmd.insert(GetClipsFrom(asset_server.load("Goblin.glb")));
        cmd.insert(DestroysBullets);
        cmd.insert(Killable::default());
        cmd.insert(SolverGroups {
            memberships: crate::solver_groups::GOBLIN,
            filters: Group::NONE,
        });
        cmd.insert(InitialAnimation::new("Armature", "Dance"));
        cmd.insert(KeepGatesClosedWhenAlive);
    });
}

fn handle_goblin_hitting_stuff(
    mut reader: EventReader<CollisionEvent>,
    goblin_query: Query<&Killable, With<IsGoblin>>,
    bullet_query: Query<(), With<Bullet>>,
    player_query: Query<(), With<IsPlayer>>,
    mut kill_events_writer: EventWriter<KillEvent>,
) {
    for (e1, e2) in events_both_ways(&mut reader) {
        if let Ok(goblin_killable) = goblin_query.get(e1) {
            if !goblin_killable.still_alive {
                continue;
            }
        } else {
            continue; // not a goblin
        }
        if bullet_query.contains(e2) {
            kill_events_writer.send(KillEvent { entity_to_kill: e1 })
        } else if player_query.contains(e2) {
            kill_events_writer.send(KillEvent { entity_to_kill: e2 })
        }
    }
}

fn goblins_face_player(
    player_query: Query<&GlobalTransform, With<IsPlayer>>,
    mut goblins_query: Query<
        (&Killable, &GlobalTransform, &mut TnuaPlatformerControls),
        With<IsGoblin>,
    >,
) {
    let Ok(player_transform) = player_query.get_single() else { return };
    let player_position = player_transform.translation();

    for (killable, goblin_transform, mut goblin_controls) in goblins_query.iter_mut() {
        if !killable.still_alive {
            continue;
        }
        let goblin_position = goblin_transform.translation();
        let vector_to_player = player_position - goblin_position;
        goblin_controls.desired_forward = Vec3::X * vector_to_player.x.signum();
    }
}
