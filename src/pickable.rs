use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;
use bevy_yoleck::prelude::*;
use bevy_yoleck::vpeol::prelude::*;
use serde::{Deserialize, Serialize};

use crate::animating::{ApplyRotationToChild, RotateAroundScaledAxis};
use crate::editing_helpers::SnapToGrid;
use crate::utils::entities_ordered_by_type;

pub struct PickablePlugin;

impl Plugin for PickablePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PickEvent>();
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
    }
}

#[derive(YoleckComponent, Default, Clone, PartialEq, Eq, Component, Serialize, Deserialize)]
enum PlantType {
    #[default]
    Tree,
}

impl PlantType {
    fn scene_name(&self) -> &'static str {
        match self {
            PlantType::Tree => "Tree.glb#Scene0",
        }
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
    picker_query: Query<(), With<CanPick>>,
    pickable_query: Query<(), With<Pickable>>,
    mut writer: EventWriter<PickEvent>,
) {
    for event in reader.iter() {
        let CollisionEvent::Started(e1, e2, CollisionEventFlags::SENSOR) = event else { continue };
        let Some([picker, pickable]) = entities_ordered_by_type!([*e1, *e2], picker_query, pickable_query) else { continue };
        writer.send(PickEvent { picker, pickable });
    }
}

#[derive(Component)]
pub struct Carried;

#[derive(Component, Default)]
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
            cmd.insert((Carried, plant_type.clone()));
            cmd.insert(SceneBundle {
                scene: asset_server.load(plant_type.scene_name()),
                transform: Transform::from_xyz(0.0, 1.0, 1.0).with_scale(Vec3::ONE * 0.5),
                ..Default::default()
            });
            can_carry.carries = Some(cmd.id());
        });
    }
}
