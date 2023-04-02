// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiSettings};
use bevy_egui_kbgp::KbgpPlugin;
use gardening_gun::GardeningGunGamePlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(EguiPlugin);
    app.insert_resource(EguiSettings {
        scale_factor: 2.0,
        ..Default::default()
    });
    app.add_plugin(KbgpPlugin);
    app.add_plugin(GardeningGunGamePlugin);
    app.run();
}
