use bevy::prelude::*;
use bevy_tnua::TnuaPlatformerControls;
use bevy_yoleck::prelude::*;
use leafwing_input_manager::axislike::VirtualAxis;
use leafwing_input_manager::prelude::*;

use crate::AppState;
use crate::player::IsPlayer;

#[derive(Actionlike, Clone, Debug)]
enum PlayerAction {
    Run,
    Jump,
}

pub struct PlayerControlsPlugin;

impl Plugin for PlayerControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default());
        app.yoleck_populate_schedule_mut()
            .add_system(add_controls_to_player);
        app.add_system(apply_controls.in_set(OnUpdate(AppState::Game)));
    }
}

fn add_controls_to_player(mut populate: YoleckPopulate<(), With<IsPlayer>>) {
    populate.populate(|ctx, mut cmd, ()| {
        if ctx.is_in_editor() {
            return;
        }
        cmd.insert(InputManagerBundle::<PlayerAction> {
            action_state: Default::default(),
            input_map: {
                let mut input_map = InputMap::default();
                input_map.insert(VirtualAxis::horizontal_arrow_keys(), PlayerAction::Run);
                input_map.insert(VirtualAxis::ad(), PlayerAction::Run);
                input_map.insert(KeyCode::Z, PlayerAction::Jump);
                input_map.insert(KeyCode::J, PlayerAction::Jump);
                input_map
            },
        });
    });
}

fn apply_controls(mut query: Query<(&ActionState<PlayerAction>, &mut TnuaPlatformerControls)>) {
    for (input, mut controls) in query.iter_mut() {
        let movement = Vec3::X * input.clamped_value(PlayerAction::Run);
        let jump = Some(input.clamped_value(PlayerAction::Jump)).filter(|jump| 0.0 < *jump);
        *controls = TnuaPlatformerControls {
            desired_velocity: movement,
            desired_forward: movement,
            jump,
        };
    }
}
