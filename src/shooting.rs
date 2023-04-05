use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::ammunition::{CanCarry, UseUpShotEvent};
use crate::player_controls::ShootEvent;
use crate::utils::sensor_events_both_ways;
use crate::AppState;

pub struct ShootingPlugin;

impl Plugin for ShootingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (update_can_shoot_cooldown, destroy_bullets_after_timeout)
                .in_set(OnUpdate(AppState::Game)),
        );
        app.add_system(apply_shooting);
        app.add_system(destroy_bullet_when_colliding_with_wall);
    }
}

#[derive(Component)]
pub struct CanShoot {
    cooldown: Timer,
}

impl Default for CanShoot {
    fn default() -> Self {
        Self {
            cooldown: {
                let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
                // trick it to start as finished
                timer.tick(timer.duration());
                timer
            },
        }
    }
}

fn update_can_shoot_cooldown(time: Res<Time>, mut query: Query<&mut CanShoot>) {
    for mut can_shoot in query.iter_mut() {
        can_shoot.cooldown.tick(time.delta());
    }
}

#[derive(Component)]
pub struct Bullet {
    timeout: Timer,
}

#[derive(Component)]
pub struct DestroysBullets;

fn apply_shooting(
    mut reader: EventReader<ShootEvent>,
    mut shooter_query: Query<(&mut CanShoot, &CanCarry, &GlobalTransform)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut use_up_shots_writer: EventWriter<UseUpShotEvent>,
) {
    for event in reader.iter() {
        let Ok((mut can_shoot, can_carry, shooter_transform)) = shooter_query.get_mut(event.shooter_entity) else { continue };
        if !can_shoot.cooldown.finished() {
            continue;
        }
        let carried_ammunition_entity = if let Some(entity) = can_carry.carries {
            entity
        } else {
            continue;
        };
        can_shoot.cooldown.reset();
        let mut cmd = commands.spawn_empty();
        cmd.insert(SceneBundle {
            scene: asset_server.load("Bullet.glb#Scene0"),
            transform: Transform::from_translation(
                shooter_transform.translation() + event.direction,
            ),
            ..Default::default()
        });
        cmd.insert(RigidBody::Dynamic);
        cmd.insert(GravityScale(0.0));
        cmd.insert(Velocity::linear(20.0 * event.direction.truncate()));
        cmd.insert(Collider::ball(0.2));
        cmd.insert(Sensor);
        cmd.insert(Bullet {
            // This should ensure out-of-screen bullets don't live forever.
            timeout: Timer::from_seconds(20.0, TimerMode::Once),
        });
        cmd.insert(ActiveEvents::COLLISION_EVENTS);
        use_up_shots_writer.send(UseUpShotEvent {
            carrier_entity: event.shooter_entity,
            carried_ammunition_entity,
            ejecet_direction: -event.direction,
        });
    }
}

fn destroy_bullet_when_colliding_with_wall(
    mut reader: EventReader<CollisionEvent>,
    bullets_query: Query<(), With<Bullet>>,
    destroy_bullets_query: Query<(), With<DestroysBullets>>,
    mut commands: Commands,
) {
    for (e1, e2) in sensor_events_both_ways(&mut reader) {
        if bullets_query.contains(e1) && destroy_bullets_query.contains(e2) {
            commands.entity(e1).despawn_recursive();
        }
    }
}

fn destroy_bullets_after_timeout(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Bullet)>,
    mut commands: Commands,
) {
    for (bullet_entity, mut bullet) in query.iter_mut() {
        if bullet.timeout.tick(time.delta()).finished() {
            commands.entity(bullet_entity).despawn_recursive();
        }
    }
}
