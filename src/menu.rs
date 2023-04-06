use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_egui_kbgp::prelude::*;

use crate::{AppState, MenuActionForKbgp};

#[derive()]
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FrameUi>();
        app.add_system(pause_unpause_game);
        app.add_systems(
            (
                prepare_menu,
                menu_header,
                main_menu.in_set(OnUpdate(AppState::MainMenu)),
                pause_menu.in_set(OnUpdate(AppState::PauseMenu)),
                #[cfg(not(target_arch = "wasm32"))]
                exit_button,
                draw_menu,
            )
                .chain(),
        );
    }
}

fn pause_unpause_game(
    mut egui_contexts: EguiContexts,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if matches!(state.0, AppState::Game) {
        let egui_context = egui_contexts.ctx_mut();
        if egui_context.kbgp_user_action() == Some(MenuActionForKbgp) {
            next_state.set(AppState::PauseMenu);
            egui_context.kbgp_clear_input();
        }
    }
}

#[derive(PartialEq)]
enum FocusLabel {
    Start,
    Exit,
}

#[derive(Resource, Default)]
struct FrameUi(Option<egui::Ui>);

fn prepare_menu(
    state: Res<State<AppState>>,
    mut egui_contexts: EguiContexts,
    mut frame_ui: ResMut<FrameUi>,
) {
    if state.0.is_menu() {
        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(egui_contexts.ctx_mut(), |ui| {
                let layout = egui::Layout::top_down(egui::Align::Center);
                ui.with_layout(layout, |ui| {
                    let frame = egui::Frame::none();
                    let prepared = frame.begin(ui);
                    frame_ui.0 = Some(prepared.content_ui);
                });
            });
    } else {
        frame_ui.0 = None;
    }
}

fn draw_menu(mut egui_contexts: EguiContexts, mut frame_ui: ResMut<FrameUi>) {
    let Some(frame_ui) = frame_ui.0.take() else { return };
    egui::CentralPanel::default()
        .frame(egui::Frame::none())
        .show(egui_contexts.ctx_mut(), |ui| {
            let layout = egui::Layout::top_down(egui::Align::Center);
            ui.with_layout(layout, |ui| {
                let frame = egui::Frame::none();
                let mut prepared = frame.begin(ui);
                prepared.content_ui = frame_ui;
                prepared.end(ui);
            });
        });
}

fn menu_header(mut frame_ui: ResMut<FrameUi>) {
    let Some(ui) = frame_ui.0.as_mut() else { return };
    ui.add_space(50.0);
    ui.label(
        egui::RichText::new("Gardening Gun")
            .size(30.0)
            .strong()
            .underline(),
    );
    ui.add_space(20.0);
}

fn main_menu(mut frame_ui: ResMut<FrameUi>) {
    let Some(ui) = frame_ui.0.as_mut() else { return };
    if ui
        .button("Start")
        .kbgp_navigation()
        .kbgp_focus_label(FocusLabel::Start)
        .kbgp_initial_focus()
        .clicked()
    {}
}

fn pause_menu(mut frame_ui: ResMut<FrameUi>, mut next_state: ResMut<NextState<AppState>>) {
    let Some(ui) = frame_ui.0.as_mut() else { return };
    if ui
        .button("Resume")
        .kbgp_navigation()
        .kbgp_initial_focus()
        .clicked()
        || ui.kbgp_user_action() == Some(MenuActionForKbgp)
    {
        next_state.set(AppState::Game);
    }
    if ui.button("Retry").kbgp_navigation().clicked() {
        next_state.set(AppState::LoadLevel);
    }
}

#[allow(dead_code)]
fn exit_button(mut frame_ui: ResMut<FrameUi>, mut exit: EventWriter<bevy::app::AppExit>) {
    let Some(ui) = frame_ui.0.as_mut() else { return };
    if ui
        .button("Exit")
        .kbgp_navigation()
        .kbgp_focus_label(FocusLabel::Exit)
        .clicked()
    {
        exit.send(Default::default());
    }
}
