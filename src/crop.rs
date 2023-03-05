use bevy::prelude::*;
use bevy_rapier2d::{prelude::{Collider, RigidBody, ExternalForce, Damping, ActiveEvents, Sensor}, na::Scale};

use crate::EDGE_BUFFER;

const PLANT_SPACING: f32 = 32.;

pub enum CropType {
    Tomato,
    Carrots,
    SuperCarrot,
    Cabbage
}

#[derive(Component)]
pub struct Crop {
    stage: usize,
    timer: Timer,
    crop_type: CropType
}

impl Crop {
    fn new(crop_type: CropType) -> Self {
        Self { 
            stage: 1, 
            timer: Timer::from_seconds(15., TimerMode::Repeating), 
            crop_type: crop_type 
        }
    }
}

pub struct CropPlugin;

impl Plugin for CropPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(crop_setup)
        ;
    }
}

fn crop_setup(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_title("Rusty Farm".to_string());

    let window_width = window.width();
    let window_height = window.height();

    let tomato_texture = asset_server.load("tomato_1.png");
    
    for row in 0..9 {
        for col in 0..9 {
            let x = EDGE_BUFFER + -(window_width / 2.) + (col as f32 * PLANT_SPACING) as f32;
            let y = (window_height / 2.) - (row as f32 * PLANT_SPACING) as f32 - EDGE_BUFFER;

            // println!("x: {} y: {}", x, y);

            commands.spawn(SpriteBundle {
                texture: tomato_texture.clone(),
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
                .insert(Sensor)
                .insert(ActiveEvents::COLLISION_EVENTS)
            ;
        }
    }
}
