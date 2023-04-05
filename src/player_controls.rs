use bevy::prelude::*;
use bevy_tnua::TnuaPlatformerControls;
use bevy_yoleck::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::player::IsPlayer;
use crate::AppState;

#[derive(Actionlike, Clone, Debug)]
enum PlayerAction {
    Run,
    Jump,
    Shoot,
}

pub struct PlayerControlsPlugin;

impl Plugin for PlayerControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShootEvent>();
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

                input_map.insert(VirtualDPad::arrow_keys(), PlayerAction::Run);
                input_map.insert(VirtualDPad::wasd(), PlayerAction::Run);
                input_map.insert(VirtualDPad::dpad(), PlayerAction::Run);
                input_map.insert(DualAxis::left_stick(), PlayerAction::Run);

                input_map.insert(KeyCode::Z, PlayerAction::Jump);
                input_map.insert(KeyCode::J, PlayerAction::Jump);
                input_map.insert(GamepadButtonType::South, PlayerAction::Jump);

                input_map.insert(KeyCode::X, PlayerAction::Shoot);
                input_map.insert(KeyCode::K, PlayerAction::Shoot);
                input_map.insert(GamepadButtonType::West, PlayerAction::Shoot);

                input_map
            },
        });
    });
}

pub struct ShootEvent {
    pub shooter_entity: Entity,
    pub direction: Vec3,
}

fn apply_controls(
    mut query: Query<(
        Entity,
        &ActionState<PlayerAction>,
        &mut TnuaPlatformerControls,
    )>,
    mut shoot_events_writer: EventWriter<ShootEvent>,
) {
    for (player_entity, input, mut controls) in query.iter_mut() {
        if let Some(axis_pair) = input.clamped_axis_pair(PlayerAction::Run) {
            controls.desired_velocity = Vec3::X * axis_pair.x();
            if 0.1 < axis_pair.x().abs() {
                controls.desired_forward = Vec3::X * axis_pair.x().signum();
            }
        } else {
            controls.desired_velocity = Vec3::ZERO;
        };
        controls.jump = Some(input.clamped_value(PlayerAction::Jump)).filter(|jump| 0.0 < *jump);

        if input.just_pressed(PlayerAction::Shoot) {
            shoot_events_writer.send(ShootEvent {
                shooter_entity: player_entity,
                direction: controls.desired_forward,
            });
        }
    }
}
