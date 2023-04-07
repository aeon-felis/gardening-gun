mod ammunition;
mod animating;
mod arena;
mod camera;
mod editing_helpers;
mod floating_text;
mod gate;
mod level_handling;
mod menu;
mod planting;
mod player;
mod player_controls;
mod shooting;
mod utils;

use bevy::prelude::*;
use bevy_rapier2d::prelude::RapierConfiguration;
use bevy_yoleck::prelude::*;

use self::ammunition::AmmunitionPlugin;
use self::animating::AnimatingPlugin;
use self::arena::ArenaPlugin;
use self::camera::GardeningGunCameraPlugin;
use self::editing_helpers::EditingHelpersPlugin;
use self::floating_text::FloatingTextPlugin;
use self::gate::GatePlugin;
use self::level_handling::{LevelHandlingPlugin, LevelProgress};
use self::menu::MenuPlugin;
use self::planting::PlantingPlugin;
use self::player::PlayerPlugin;
use self::player_controls::PlayerControlsPlugin;
use self::shooting::ShootingPlugin;

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
            app.add_plugin(LevelHandlingPlugin);
            if let Some(start_at_level) = &self.start_at_level {
                let start_at_level = if start_at_level.ends_with(".yol") {
                    start_at_level.clone()
                } else {
                    format!("{}.yol", start_at_level)
                };
                app.add_startup_system(
                    move |mut level_progress: ResMut<LevelProgress>,
                          mut app_state: ResMut<NextState<AppState>>| {
                        level_progress.current_level = Some(start_at_level.clone());
                        app_state.set(AppState::LoadLevel);
                    },
                );
            }
        }

        app.add_plugin(ArenaPlugin);
        app.add_plugin(PlayerPlugin);
        app.add_plugin(PlayerControlsPlugin);
        app.add_plugin(AnimatingPlugin);
        app.add_plugin(AmmunitionPlugin);
        app.add_plugin(FloatingTextPlugin);
        app.add_plugin(ShootingPlugin);
        app.add_plugin(PlantingPlugin);
        app.add_plugin(GatePlugin);
        app.add_system(enable_disable_physics);
    }
}

#[derive(States, Default, Clone, Hash, Debug, PartialEq, Eq)]
pub enum AppState {
    #[default]
    MainMenu,
    PauseMenu,
    LevelSelectMenu,
    LoadLevel,
    Editor,
    Game,
}

impl AppState {
    pub fn is_menu(&self) -> bool {
        match self {
            AppState::MainMenu => true,
            AppState::PauseMenu => true,
            AppState::LevelSelectMenu => true,
            AppState::LoadLevel => false,
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
