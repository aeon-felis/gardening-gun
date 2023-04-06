use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_yoleck::prelude::*;
use serde::{Deserialize, Serialize};

use crate::utils::events_both_ways;
use crate::AppState;

pub struct PlantingPlugin;

impl Plugin for PlantingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(initiate_planting);
        app.add_system(apply_growing.in_set(OnUpdate(AppState::Game)));
    }
}

#[derive(YoleckComponent, Default, Clone, PartialEq, Eq, Component, Serialize, Deserialize)]
pub enum PlantType {
    #[default]
    Tree,
}

impl PlantType {
    pub fn scene_name(&self) -> &'static str {
        match self {
            PlantType::Tree => "Tree.glb#Scene0",
        }
    }
}

#[derive(Component)]
pub struct FertileGround;

#[derive(Component)]
pub struct FlyingSeed;

#[derive(Component)]
struct Growing;

fn initiate_planting(
    mut reader: EventReader<CollisionEvent>,
    seed_query: Query<(&GlobalTransform, &PlantType), With<FlyingSeed>>,
    ground_query: Query<(), With<FertileGround>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (e1, e2) in events_both_ways(&mut reader) {
        let (Ok((transform, plant_type)), Ok(_)) = (seed_query.get(e1), ground_query.get(e2)) else { continue };
        commands.entity(e1).despawn_recursive();
        let mut cmd = commands.spawn_empty();
        cmd.insert(plant_type.clone());
        cmd.insert(SceneBundle {
            scene: asset_server.load(plant_type.scene_name()),
            transform: Transform::from_translation(transform.translation() + Vec3::Y)
                .with_scale(Vec3::ONE * 0.1),
            ..Default::default()
        });
        cmd.insert(RigidBody::Dynamic);
        cmd.insert(Collider::cuboid(1.0, 1.5));
        cmd.insert(LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_X);
        cmd.insert(Growing);
        cmd.insert(YoleckBelongsToLevel);
    }
}

fn apply_growing(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform), With<Growing>>,
    mut commands: Commands,
) {
    for (entity, mut transform) in query.iter_mut() {
        let pace = 2.0 * time.delta_seconds();
        transform.scale += Vec3::ONE * pace;
        transform.translation += Vec3::Y * pace;
        if 1.0 <= transform.scale.length_squared() {
            transform.scale = Vec3::ONE;
            commands.entity(entity).remove::<Growing>();
        }
    }
}
