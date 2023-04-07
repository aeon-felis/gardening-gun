use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_tnua::TnuaManualTurningOutput;

use crate::AppState;

pub struct AnimatingPlugin;

impl Plugin for AnimatingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (apply_actors_rotation, apply_rotate_around_axis).in_set(OnUpdate(AppState::Game)),
        );
        app.add_systems((detect_animation_players, detect_animation_clips));
    }
}

#[derive(Component)]
pub struct ApplyRotationToChild(pub Entity);

fn apply_actors_rotation(
    query: Query<(&TnuaManualTurningOutput, &ApplyRotationToChild)>,
    mut transform_query: Query<&mut Transform>,
) {
    for (manual_turning, ApplyRotationToChild(child_to_rotate)) in query.iter() {
        if 0.0 < manual_turning.forward.length_squared() {
            if let Ok(mut transform) = transform_query.get_mut(*child_to_rotate) {
                transform.look_to(manual_turning.forward, Vec3::Y);
            }
        }
    }
}

#[derive(Component)]
pub struct RotateAroundScaledAxis(pub Vec3);

fn apply_rotate_around_axis(
    time: Res<Time>,
    mut query: Query<(&RotateAroundScaledAxis, &mut Transform)>,
) {
    for (RotateAroundScaledAxis(scaled_axis), mut transform) in query.iter_mut() {
        transform.rotate(Quat::from_scaled_axis(time.delta_seconds() * *scaled_axis));
    }
}

#[derive(Component, Default, Debug)]
pub struct AnimationsOwner {
    pub players: HashMap<String, Entity>,
    pub clips: HashMap<String, Handle<AnimationClip>>,
}

#[derive(Component)]
pub struct GetClipsFrom(pub Handle<Gltf>);

fn detect_animation_players(
    query: Query<(Entity, &Name), Added<AnimationPlayer>>,
    parents_query: Query<&Parent>,
    mut animation_owners_query: Query<&mut AnimationsOwner>,
) {
    for (entity, name) in query.iter() {
        let mut parent_entity = entity;
        while let Ok(parent) = parents_query.get(parent_entity) {
            parent_entity = **parent;
            if let Ok(mut animation_owners) = animation_owners_query.get_mut(parent_entity) {
                animation_owners
                    .players
                    .insert(name.as_str().to_owned(), entity);
                break;
            }
        }
    }
}

fn detect_animation_clips(
    mut query: Query<(Entity, &mut AnimationsOwner, &GetClipsFrom)>,
    gltf_assets: Res<Assets<Gltf>>,
    mut commands: Commands,
) {
    for (entity, mut animation_owners, GetClipsFrom(get_clips_from)) in query.iter_mut() {
        let Some(gltf) = gltf_assets.get(get_clips_from) else { continue };
        commands.entity(entity).remove::<GetClipsFrom>();
        for (name, clip) in gltf.named_animations.iter() {
            animation_owners.clips.insert(name.clone(), clip.clone());
        }
    }
}
