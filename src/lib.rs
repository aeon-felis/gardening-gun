mod menu;

use bevy::prelude::*;

use self::menu::MenuPlugin;

pub struct GardeningGunGamePlugin;

impl Plugin for GardeningGunGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>();
        app.add_plugin(MenuPlugin);
    }
}

#[derive(States, Default, Clone, Hash, Debug, PartialEq, Eq)]
pub enum AppState {
    #[default]
    MainMenu,
}
