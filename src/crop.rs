use bevy::prelude::*;
use bevy_rapier2d::{prelude::{Collider, RigidBody, ExternalForce, Damping, ActiveEvents, Sensor, RapierContext}, na::Scale};

use crate::{EDGE_BUFFER, AnimationTimer, player::Player};

const PLANT_SPACING: f32 = 32.;

pub enum CropType {
    Potato,
    Carrots,
    Corn,
    Cabbage
}

#[derive(Component)]
pub struct Crop {
    stage: usize,
    crop_type: CropType
}

impl Crop {
    fn new(crop_type: CropType) -> Self {
        Self { 
            stage: 1, 
            crop_type
        }
    }
}

#[derive(Default, Resource)]
pub struct CropCollider {
    pub collider: Option<Entity>
}

pub struct CropPlugin;

impl Plugin for CropPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CropCollider::default())
            .add_startup_system(crop_setup)
            .add_system(crop_lifetimes)
        ;
    }
}

fn crop_setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut windows: ResMut<Windows>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    let window = windows.get_primary_mut().unwrap();
    window.set_title("Rusty Farm".to_string());

    let window_width = window.width();
    let window_height = window.height();

    let corn_texture = asset_server.load("crops/corn_growth.png");

    let corn_atlas = TextureAtlas::from_grid(
        corn_texture,
        Vec2::new(16., 29.),
        5,
        1,
        None,
        None
    );

    let handle = texture_atlases.add(corn_atlas);
    
    for row in 0..9 {
        for col in 0..9 {
            let x = EDGE_BUFFER + -(window_width / 2.) + (col as f32 * PLANT_SPACING) as f32;
            let y = (window_height / 2.) - (row as f32 * PLANT_SPACING) as f32 - EDGE_BUFFER;

            commands.spawn(SpriteSheetBundle {
                texture_atlas: handle.clone(),
                transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                ..Default::default()
            })
                .insert(Collider::cuboid(3., 2.5))
                .insert(RigidBody::KinematicPositionBased)
                .insert(ExternalForce {
                    force: Vec2::ZERO,
                    torque: 0.
                })
                .insert(Damping {
                    linear_damping: 100.,
                    angular_damping: 100.
                })
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(Crop::new(CropType::Corn))
                .insert(AnimationTimer(Timer::from_seconds(5., TimerMode::Repeating)))
            ;
        }
    }
}

fn crop_lifetimes(
    time: Res<Time>,
    mut crop_query: Query<(
        &mut AnimationTimer,
        &mut Crop,
        &mut TextureAtlasSprite,
        Entity
    )>,
    context: Res<RapierContext>,
    player_query: Query<Entity, With<Player>>,
    mut crop_colider: ResMut<CropCollider>
) {
    let player_entity = player_query.single();

    let mut colliosion = false;

    for crop in crop_query.iter_mut() {
        let (mut timer, mut crop, mut sprite, crop_entity) = crop;

        if let Some(_) = context.contact_pair(player_entity, crop_entity) {
            colliosion = true;
            if crop_colider.collider != Some(crop_entity){
                crop_colider.collider = Some(crop_entity);
            }
        }
        
        if crop.stage + 1 <= 5 {
            timer.tick(time.delta());
            if timer.finished() {
                timer.reset();
                sprite.index = crop.stage;
                crop.stage += 1;
            }
        }
    }
    if !colliosion {
        *crop_colider = CropCollider::default();
    }
}
