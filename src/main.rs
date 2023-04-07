// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_egui::{EguiPlugin, EguiSettings};
use bevy_egui_kbgp::{KbgpNavBindings, KbgpNavCommand, KbgpPlugin, KbgpSettings};
use bevy_pkv::PkvStore;
use bevy_rapier2d::prelude::*;
use bevy_tnua::{TnuaPlatformerPlugin, TnuaRapier2dPlugin};
use bevy_yoleck::prelude::*;
use bevy_yoleck::vpeol::prelude::*;
use clap::Parser;
use gardening_gun::{GardeningGunGamePlugin, MenuActionForKbgp};

#[derive(Parser, Debug)]
struct Args {
    #[clap(long)]
    editor: bool,
    #[clap(long)]
    level: Option<String>,
}

fn main() {
    let args = Args::parse();

    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Gardening Gun".to_owned(),
            resolution: WindowResolution::new(800.0, 600.0),
            ..Default::default()
        }),
        ..Default::default()
    }));
    app.insert_resource(PkvStore::new("AeonFelis", "GardeningGun"));
    app.add_plugin(EguiPlugin);
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
    app.add_plugin(TnuaPlatformerPlugin);
    app.add_plugin(TnuaRapier2dPlugin);
    if args.editor {
        app.add_plugin(YoleckPluginForEditor);
        app.add_plugin(Vpeol3dPluginForEditor::sidescroller());
    } else {
        app.add_plugin(YoleckPluginForGame);
        app.add_plugin(Vpeol3dPluginForGame);
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
    }

    app.add_plugin(GardeningGunGamePlugin {
        is_editor: args.editor,
        start_at_level: args.level,
    });

    app.run();
}
