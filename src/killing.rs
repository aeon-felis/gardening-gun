use bevy::prelude::*;
use ordered_float::OrderedFloat;

use crate::animating::AnimationsOwner;
use crate::editing_helpers::GridSize;
use crate::player::IsPlayer;
use crate::shooting::DestroysBullets;
use crate::AppState;

pub struct KillingPlugin;

impl Plugin for KillingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<KillEvent>();
        app.add_systems((handle_killing, game_over_when_killing_player));
        app.add_system(kill_player_when_they_fall.in_set(OnUpdate(AppState::Game)));
    }
}

pub struct KillEvent {
    pub entity_to_kill: Entity,
}

#[derive(Component)]
pub struct Killable {
    pub still_alive: bool,
}

impl Default for Killable {
    fn default() -> Self {
        Self { still_alive: true }
    }
}

fn handle_killing(
    mut reader: EventReader<KillEvent>,
    mut query: Query<(&mut Killable, &AnimationsOwner)>,
    mut commands: Commands,
    mut animation_players_query: Query<&mut AnimationPlayer>,
) {
    for event in reader.iter() {
        let Ok((mut killable, animations_owner)) = query.get_mut(event.entity_to_kill) else {
            error!("Entity {:?} is not killable", event.entity_to_kill);
            continue;
        };
        commands
            .entity(event.entity_to_kill)
            .remove::<DestroysBullets>();
        if !killable.still_alive {
            continue;
        }
        killable.still_alive = false;
        let clip = animations_owner
            .clips
            .get("Death")
            .expect("Does not have death animation");
        let animation_player = animations_owner
            .players
            .get("Armature")
            .expect("Does not have armature");
        let mut animation_player = animation_players_query
            .get_mut(*animation_player)
            .expect("Armature AnimationPlayer does not exist");
        animation_player.play(clip.clone());
    }
}

fn game_over_when_killing_player(
    mut reader: EventReader<KillEvent>,
    query: Query<(), With<IsPlayer>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for event in reader.iter() {
        if query.contains(event.entity_to_kill) {
            next_state.set(AppState::GameOver);
        }
    }
}

#[derive(Component)]
pub struct KillPlayerWhenBelow;

fn kill_player_when_they_fall(
    objects_query: Query<(&GlobalTransform, Option<&GridSize>), With<KillPlayerWhenBelow>>,
    players_query: Query<(Entity, &GlobalTransform), With<IsPlayer>>,
    mut kill_events_writer: EventWriter<KillEvent>,
) {
    let Some(lowest_y) = objects_query.iter().map(|(transform, grid_size)| {
        let position = transform.translation();
        if let Some(grid_size) = grid_size {
            position.y - 0.5 * grid_size.0.y as f32
        } else {
            position.y
        }
    }).min_by_key(|y| OrderedFloat(*y)) else { return };
    for (player_entity, player_transform) in players_query.iter() {
        let player_below_level = player_transform.translation().y - lowest_y;
        if player_below_level < -50.0 {
            kill_events_writer.send(KillEvent {
                entity_to_kill: player_entity,
            });
        }
    }
}
