use bevy::prelude::*;

use crate::animating::AnimationsOwner;
use crate::shooting::DestroysBullets;

pub struct KillingPlugin;

impl Plugin for KillingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<KillEvent>();
        app.add_system(handle_killing);
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
