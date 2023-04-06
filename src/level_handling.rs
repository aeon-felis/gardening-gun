use bevy::prelude::*;
use bevy_yoleck::prelude::*;

use crate::AppState;

pub struct LevelHandlingPlugin;

impl Plugin for LevelHandlingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LevelProgress>();
        app.add_systems(
            (clear_old_entities, launch_level_loading_command)
                .chain()
                .in_schedule(OnEnter(AppState::LoadLevel)),
        );
    }
}

#[derive(Resource, Default)]
pub struct LevelProgress {
    pub just_completed: Option<String>,
    pub current_level: Option<String>,
    pub num_levels_available: usize,
}

fn clear_old_entities(query: Query<Entity, With<YoleckBelongsToLevel>>, mut commands: Commands) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn launch_level_loading_command(
    level_progress: Res<LevelProgress>,
    mut yoleck_loading_command: ResMut<YoleckLoadingCommand>,
    asset_server: Res<AssetServer>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    let current_level = level_progress
        .current_level
        .as_ref()
        .expect("`LoadLevel` state entered without setting `current_level`");
    *yoleck_loading_command =
        YoleckLoadingCommand::FromAsset(asset_server.load(format!("levels/{}", current_level)));
    app_state.set(AppState::Game);
}
