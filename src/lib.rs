mod animating;
mod arena;
mod camera;
mod editing_helpers;
mod menu;
mod player;
mod player_controls;

use bevy::prelude::*;
use bevy_rapier2d::prelude::RapierConfiguration;
use bevy_yoleck::prelude::*;

use self::animating::AnimatingPlugin;
use self::arena::ArenaPlugin;
use self::camera::GardeningGunCameraPlugin;
use self::editing_helpers::EditingHelpersPlugin;
use self::menu::MenuPlugin;
use self::player::PlayerPlugin;
use self::player_controls::PlayerControlsPlugin;

pub struct GardeningGunGamePlugin {
    pub is_editor: bool,
    pub start_at_level: Option<String>,
}

impl Plugin for GardeningGunGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>();
        app.add_plugin(GardeningGunCameraPlugin);

        if self.is_editor {
            app.add_plugin(YoleckSyncWithEditorState {
                when_editor: AppState::Editor,
                when_game: AppState::Game,
            });
            app.add_plugin(EditingHelpersPlugin);
        } else {
            app.add_plugin(MenuPlugin);
            if let Some(start_at_level) = &self.start_at_level {
                let start_at_level = if start_at_level.ends_with(".yol") {
                    start_at_level.clone()
                } else {
                    format!("{}.yol", start_at_level)
                };
                app.add_startup_system(
                    move |mut yoleck_loading_command: ResMut<YoleckLoadingCommand>,
                          asset_server: Res<AssetServer>,
                          mut app_state: ResMut<NextState<AppState>>| {
                        *yoleck_loading_command = YoleckLoadingCommand::FromAsset(
                            asset_server.load(format!("levels/{}", start_at_level)),
                        );
                        app_state.set(AppState::Game); // TODO: change to level loading state?
                    },
                );
            }
        }

        app.add_plugin(ArenaPlugin);
        app.add_plugin(PlayerPlugin);
        app.add_plugin(PlayerControlsPlugin);
        app.add_plugin(AnimatingPlugin);
        app.add_system(enable_disable_physics);
    }
}

#[derive(States, Default, Clone, Hash, Debug, PartialEq, Eq)]
pub enum AppState {
    #[default]
    MainMenu,
    Editor,
    Game,
}

impl AppState {
    pub fn is_menu(&self) -> bool {
        match self {
            AppState::MainMenu => true,
            AppState::Editor => false,
            AppState::Game => false,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct MenuActionForKbgp;

fn enable_disable_physics(
    state: Res<State<AppState>>,
    mut rapier_configuration: ResMut<RapierConfiguration>,
) {
    rapier_configuration.physics_pipeline_active = state.0 == AppState::Game;
}
