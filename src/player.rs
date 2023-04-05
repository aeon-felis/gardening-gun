use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tnua::{
    TnuaFreeFallBehavior, TnuaManualTurningOutput, TnuaPlatformerBundle, TnuaPlatformerConfig,
    TnuaPlatformerControls, TnuaRapier2dSensorShape,
};
use bevy_yoleck::prelude::*;
use bevy_yoleck::vpeol::prelude::*;

use crate::ammunition::{CanCarry, CanPick};
use crate::animating::ApplyRotationToChild;
use crate::editing_helpers::SnapToGrid;
use crate::shooting::CanShoot;

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
                float_height: 1.5,
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
    });
}
