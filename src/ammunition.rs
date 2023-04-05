use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_yoleck::prelude::*;
use bevy_yoleck::vpeol::prelude::*;

use crate::animating::{ApplyRotationToChild, RotateAroundScaledAxis};
use crate::editing_helpers::SnapToGrid;
use crate::planting::{FlyingSeed, PlantType};
use crate::utils::sensor_events_both_ways;

pub struct AmmunitionPlugin;

impl Plugin for AmmunitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PickEvent>();
        app.add_event::<UseUpShotEvent>();
        app.add_yoleck_entity_type({
            YoleckEntityType::new("PickableAmmo")
                .with::<Vpeol3dPosition>()
                .with::<PlantType>()
                .insert_on_init_during_editor(|| SnapToGrid)
                .insert_on_init(|| Pickable)
        });
        app.yoleck_populate_schedule_mut()
            .add_system(populate_pickable_ammo);
        app.add_systems((initiate_pickup, handle_carrying).chain());
        app.add_system(handle_useup);
    }
}

fn populate_pickable_ammo(
    mut populate: YoleckPopulate<&PlantType>,
    asset_server: Res<AssetServer>,
    marking: YoleckMarking,
) {
    populate.populate(|ctx, mut cmd, plant_type| {
        if ctx.is_first_time() {
            cmd.insert(VpeolWillContainClickableChildren);
            cmd.insert(VisibilityBundle::default());
            cmd.with_children(|commands| {
                let mut child = commands.spawn(marking.marker());
                child.insert(SceneBundle {
                    scene: asset_server.load(plant_type.scene_name()),
                    transform: Transform {
                        translation: Default::default(),
                        rotation: Quat::from_rotation_x(0.5),
                        scale: Vec3::ONE * 0.4,
                    },
                    ..Default::default()
                });
                child.insert(RotateAroundScaledAxis(2.0 * Vec3::Y));
            });
            cmd.insert(RigidBody::Fixed);
            cmd.insert(Collider::capsule_y(0.5, 0.5));
            cmd.insert(Sensor);
            cmd.insert(ActiveEvents::COLLISION_EVENTS);
        }
    });
}

#[derive(Component)]
pub struct CanPick;

#[derive(Component)]
pub struct Pickable;

pub struct PickEvent {
    pub picker: Entity,
    pub pickable: Entity,
}

fn initiate_pickup(
    mut reader: EventReader<CollisionEvent>,
    picker_query: Query<Entity, With<CanPick>>,
    pickable_query: Query<Entity, With<Pickable>>,
    mut writer: EventWriter<PickEvent>,
) {
    for (e1, e2) in sensor_events_both_ways(&mut reader) {
        let (Ok(picker), Ok(pickable)) = (picker_query.get(e1), pickable_query.get(e2)) else { continue };
        writer.send(PickEvent { picker, pickable });
    }
}

#[derive(Component)]
pub struct CarriedAmmunition {
    remaining_shots: usize,
}

#[derive(Component, Default, Debug)]
pub struct CanCarry {
    pub carries: Option<Entity>,
}

fn handle_carrying(
    mut reader: EventReader<PickEvent>,
    mut can_carry_query: Query<(&mut CanCarry, &ApplyRotationToChild)>,
    plant_type_query: Query<&PlantType>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for event in reader.iter() {
        let Ok((mut can_carry, ApplyRotationToChild(model_entity))) = can_carry_query.get_mut(event.picker) else { continue };
        if can_carry.carries.is_some() {
            continue;
        }
        let Ok(plant_type) = plant_type_query.get(event.pickable) else { continue };
        commands.entity(event.pickable).despawn_recursive();
        commands.entity(*model_entity).with_children(|commands| {
            let mut cmd = commands.spawn_empty();
            cmd.insert(CarriedAmmunition { remaining_shots: 1 });
            cmd.insert(plant_type.clone());
            cmd.insert(SceneBundle {
                scene: asset_server.load(plant_type.scene_name()),
                transform: Transform::from_xyz(0.0, 1.0, 1.0).with_scale(Vec3::ONE * 0.5),
                ..Default::default()
            });
            can_carry.carries = Some(cmd.id());
        });
    }
}

pub struct UseUpShotEvent {
    pub carrier_entity: Entity,
    pub carried_ammunition_entity: Entity,
    pub ejecet_direction: Vec3,
}

fn handle_useup(
    mut reader: EventReader<UseUpShotEvent>,
    mut query: Query<(&mut CarriedAmmunition, &mut Transform, &GlobalTransform)>,
    mut carrier_query: Query<&mut CanCarry>,
    mut commands: Commands,
) {
    for event in reader.iter() {
        let Ok((mut carried_ammunition, mut transform, global_transform)) = query.get_mut(event.carried_ammunition_entity) else { continue };
        carried_ammunition.remaining_shots -= 1;
        transform.scale -= Vec3::ONE * 0.1;

        if carried_ammunition.remaining_shots == 0 {
            if let Ok(mut can_carry) = carrier_query.get_mut(event.carrier_entity) {
                can_carry.carries = None;
            }
            *transform = global_transform.compute_transform();
            let mut cmd = commands.entity(event.carried_ammunition_entity);
            cmd.remove_parent();
            cmd.insert(RigidBody::Dynamic);
            cmd.insert(Collider::capsule_y(0.5, 0.5));
            cmd.insert(Velocity {
                linvel: 3.0 * event.ejecet_direction.truncate() + 20.0 * Vec2::Y,
                angvel: -10.0 * event.ejecet_direction.x,
            });
            cmd.insert(ActiveEvents::COLLISION_EVENTS);
            cmd.insert(FlyingSeed);
        }
    }
}
