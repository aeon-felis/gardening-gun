use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_egui_kbgp::prelude::*;

use crate::AppState;

#[derive()]
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FrameUi>();
        app.add_systems(
            (
                prepare_menu,
                menu_header,
                main_menu.in_set(OnUpdate(AppState::MainMenu)),
                #[cfg(not(target_arch = "wasm32"))]
                exit_button,
                draw_menu,
            )
                .chain(),
        );
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
    egui::CentralPanel::default()
        .frame(egui::Frame::none())
        .show(egui_contexts.ctx_mut(), |ui| {
            let layout = egui::Layout::top_down(egui::Align::Center);
            ui.with_layout(layout, |ui| {
                let frame = egui::Frame::none();
                let mut prepared = frame.begin(ui);
                prepared.content_ui = frame_ui.0.take().unwrap();
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
        .clicked()
    {}
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
