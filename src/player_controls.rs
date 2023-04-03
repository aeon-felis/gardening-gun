use bevy::prelude::*;
use bevy_yoleck::prelude::*;
use leafwing_input_manager::axislike::VirtualAxis;
use leafwing_input_manager::prelude::*;

use crate::player::IsPlayer;

#[derive(Actionlike, Clone, Debug)]
enum PlayerAction {
    Run,
    Jump,
}

pub struct PlayerControlsPlugin;

impl Plugin for PlayerControlsPlugin {
    fn build(&self, app: &mut App) {
        app.yoleck_populate_schedule_mut()
            .add_system(add_controls_to_player);
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
