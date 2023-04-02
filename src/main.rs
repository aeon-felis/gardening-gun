// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiSettings};
use bevy_egui_kbgp::{KbgpNavBindings, KbgpNavCommand, KbgpPlugin, KbgpSettings};
use gardening_gun::{GardeningGunGamePlugin, MenuActionForKbgp};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(EguiPlugin);
    app.insert_resource(EguiSettings {
        scale_factor: 2.0,
        ..Default::default()
    });
    app.add_plugin(KbgpPlugin);
    app.insert_resource(KbgpSettings {
        disable_default_navigation: true,
        disable_default_activation: false,
        prevent_loss_of_focus: true,
        focus_on_mouse_movement: true,
        allow_keyboard: true,
        allow_mouse_buttons: false,
        allow_mouse_wheel: false,
        allow_mouse_wheel_sideways: false,
        allow_gamepads: true,
        bindings: {
            KbgpNavBindings::default()
                .with_wasd_navigation()
                .with_key(KeyCode::Escape, KbgpNavCommand::user(MenuActionForKbgp))
                .with_gamepad_button(
                    GamepadButtonType::Start,
                    KbgpNavCommand::user(MenuActionForKbgp),
                )
        },
    });
    app.add_plugin(GardeningGunGamePlugin);
    app.run();
}
