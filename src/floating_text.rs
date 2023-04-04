use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::render::view::{check_visibility, RenderLayers, VisibilitySystems, VisibleEntities};
use bevy_egui::egui;
use bevy_yoleck::prelude::*;
use bevy_yoleck::vpeol::prelude::*;
use serde::{Deserialize, Serialize};

pub struct FloatingTextPlugin;

impl Plugin for FloatingTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_entity_type({
            YoleckEntityType::new("FloatingText")
                .with::<Vpeol3dPosition>()
                .with::<Vpeol3dScale>()
                .with::<FloatingText>()
        });
        app.add_yoleck_edit_system(edit_text);
        app.yoleck_populate_schedule_mut().add_system(populate_text);
        app.add_system(
            override_visible_entities
                .in_set(VisibilitySystems::CheckVisibility)
                .after(check_visibility),
        );
    }
}

#[derive(YoleckComponent, Clone, PartialEq, Component, Serialize, Deserialize)]
struct FloatingText {
    text: String,
    font_size: f32,
}

impl Default for FloatingText {
    fn default() -> Self {
        Self {
            text: "<TEXT>".to_owned(),
            font_size: 24.0,
        }
    }
}

fn edit_text(
    mut ui: ResMut<YoleckUi>,
    mut edit: YoleckEdit<(&mut FloatingText, &mut Vpeol3dPosition, &mut Vpeol3dScale)>,
) {
    let Ok((mut text_content, mut position, mut scale)) = edit.get_single_mut() else { return };
    position.0.z = -1.0;
    ui.text_edit_multiline(&mut text_content.text);
    ui.add(egui::Slider::new(&mut text_content.font_size, 50.0..=200.0).logarithmic(true));
    let mut ratio = scale.0.y / scale.0.x;
    ui.add(egui::Slider::new(&mut scale.0.x, 0.5..=20.0).logarithmic(true));
    ui.add(egui::Slider::new(&mut ratio, 0.1..=10.0).logarithmic(true));
    scale.0.y = ratio * scale.0.x;
}

#[derive(Component)]
struct FloatingTextChildren {
    text_entity: Entity,
    camera_entity: Entity,
}

fn populate_text(
    mut populate: YoleckPopulate<(&FloatingText, Option<&FloatingTextChildren>)>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut image_assets: ResMut<Assets<Image>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    populate.populate(|ctx, mut cmd, (text_content, child_entities)| {
        let text_entity;
        if let Some(child_entities) = child_entities {
            text_entity = child_entities.text_entity;
        } else {
            let mesh = mesh_assets.add(Mesh::from(shape::Quad {
                size: Vec2::new(4.0, 1.0),
                flip: false,
            }));
            let size = Extent3d {
                width: 2048,
                height: 512,
                ..Default::default()
            };
            let mut texture = Image {
                texture_descriptor: TextureDescriptor {
                    label: None,
                    size,
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: TextureDimension::D2,
                    format: TextureFormat::Bgra8UnormSrgb,
                    usage: TextureUsages::TEXTURE_BINDING
                        | TextureUsages::COPY_DST
                        | TextureUsages::RENDER_ATTACHMENT,
                    view_formats: &[],
                },
                ..Default::default()
            };
            texture.resize(size);
            let texture = image_assets.add(texture);
            let material = material_assets.add(StandardMaterial {
                base_color_texture: Some(texture.clone()),
                unlit: true,
                // alpha_mode: AlphaMode::Blend,
                alpha_mode: AlphaMode::Mask(0.5),
                ..Default::default()
            });
            cmd.insert(PbrBundle {
                mesh,
                material,
                ..Default::default()
            });

            text_entity = cmd
                .commands()
                .spawn(Text2dBundle {
                    ..Default::default()
                })
                .insert(RenderLayers::layer(1))
                .id();
            cmd.add_child(text_entity);
            let camera_entity = cmd
                .commands()
                .spawn(Camera2dBundle {
                    camera: Camera {
                        order: -1,
                        target: RenderTarget::Image(texture),
                        ..Default::default()
                    },
                    camera_2d: Camera2d {
                        clear_color: if ctx.is_in_editor() {
                            ClearColorConfig::Custom(Color::GRAY)
                        } else {
                            // ClearColorConfig::Custom(Color::WHITE.with_a(0.0))
                            ClearColorConfig::None
                        },
                    },
                    ..Default::default()
                })
                .insert(RenderLayers::layer(1))
                .id();
            cmd.add_child(camera_entity);
            cmd.insert(FloatingTextChildren {
                text_entity,
                camera_entity,
            });
        }
        cmd.commands()
            .entity(text_entity)
            .insert(Text::from_section(
                &text_content.text,
                TextStyle {
                    font: asset_server.load("FiraSans-Bold.ttf"),
                    font_size: text_content.font_size,
                    color: Color::WHITE,
                },
            ));
    });
}

fn override_visible_entities(
    parents_query: Query<&FloatingTextChildren>,
    mut visible_entities_query: Query<&mut VisibleEntities>,
) {
    for FloatingTextChildren {
        text_entity,
        camera_entity,
    } in parents_query.iter()
    {
        let Ok(mut visible_entities) = visible_entities_query.get_mut(*camera_entity) else { continue };
        visible_entities.entities = vec![*text_entity];
    }
}
