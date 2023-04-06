use bevy::prelude::*;
use bevy_pkv::PkvStore;
use bevy_yoleck::prelude::*;

use crate::AppState;

pub struct LevelHandlingPlugin;

impl Plugin for LevelHandlingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LevelProgress>();
        app.add_system(read_last_finished_level);
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
    pub level_index: Handle<YoleckLevelIndex>,
}

const LEVEL_PKV_KEY: &str = "completed_up_to_level";

fn read_last_finished_level(
    pkv: Res<PkvStore>,
    mut level_progress: ResMut<LevelProgress>,
    asset_server: Res<AssetServer>,
    level_index_assets: Res<Assets<YoleckLevelIndex>>,
) {
    if 0 < level_progress.num_levels_available {
        return;
    }
    level_progress.level_index = asset_server.load("levels/index.yoli");
    if let Ok(completed_up_to_level) = pkv.get::<String>(LEVEL_PKV_KEY) {
        let Some(level_index) = level_index_assets.get(&level_progress.level_index) else { return };
        if let Some(index) = level_index.iter().enumerate().find_map(|(index, level)| {
            if level.filename == completed_up_to_level {
                Some(index)
            } else {
                None
            }
        }) {
            level_progress.num_levels_available = index + 2;
        } else {
            error!(
                "Unable to find level {:?}, starting anew",
                completed_up_to_level
            );
            level_progress.num_levels_available = 1;
        }
    } else {
        level_progress.num_levels_available = 1;
    }
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
