use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_yoleck::prelude::*;
use bevy_yoleck::vpeol::prelude::*;
use serde::{Deserialize, Serialize};

use crate::animating::RotateAroundScaledAxis;
use crate::editing_helpers::SnapToGrid;

pub struct PickablePlugin;

impl Plugin for PickablePlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_entity_type({
            YoleckEntityType::new("PickableAmmo")
                .with::<Vpeol3dPosition>()
                .with::<PlantType>()
                .insert_on_init_during_editor(|| SnapToGrid)
        });
        app.yoleck_populate_schedule_mut()
            .add_system(populate_pickable_ammo);
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
        }
    });
}
