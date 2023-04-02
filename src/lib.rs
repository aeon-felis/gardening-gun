mod arena;
mod camera;
mod editing_helpers;
mod menu;

use bevy::prelude::*;
use bevy_yoleck::prelude::*;

use self::arena::ArenaPlugin;
use self::camera::GardeningGunCameraPlugin;
use self::editing_helpers::EditingHelpersPlugin;
use self::menu::MenuPlugin;

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
        }

        app.add_plugin(ArenaPlugin);
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
