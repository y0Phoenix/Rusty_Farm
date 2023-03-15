use bevy::prelude::*;
// use bevy_animations::*;
use crate::animations::*;
use crop::CropPlugin;
use player::PlayerPlugin;
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use ldtk::FarmWorldPlugin;

mod player;
mod crop;
mod ldtk;
mod animations;

pub const EDGE_BUFFER: f32 = 25.;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum MoveDirection {
    Left,
    Right,
    Up,
    Down,
    Still
}

/* A system that displays the events. */
fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}

impl MoveDirection {
    fn get_direction(direction: &Self) -> Vec2 {
        match direction {
            MoveDirection::Left => Vec2::new(-1., 0.),
            MoveDirection::Right => Vec2::new(1., 0.),
            MoveDirection::Up => Vec2::new(0., 1.),
            MoveDirection::Down => Vec2::new(0., -1.),
            MoveDirection::Still => Vec2::new(0., 0.),
        }
    }
    fn from(vector: Vec2) -> Self {
        if vector.x == -1. && vector.y == 0. {
            MoveDirection::Left
        }
        else if vector.x == 1. && vector.y == 0. {
            MoveDirection::Right
        }
        else if vector.x == 0. && vector.y == 1. {
            MoveDirection::Up
        }
        else if vector.x == 0. && vector.y == -1. {
            MoveDirection::Down
        }
        else {
            MoveDirection::Still
        }
    }
}


#[derive(Component, Deref, DerefMut)]
pub struct EntityVelocity(Vec2);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(20.0))
        .add_plugin(AnimationsPlugin {
            pixels_per_meter: 20.
        })
        .add_plugin(LdtkPlugin)
        .add_plugin(FarmWorldPlugin)
        // .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        // .add_plugin(PlayerPlugin)
        // .add_plugin(CropPlugin)
        // .add_startup_system(setup)
        // .insert_resource(ClearColor(Color::hex("005500").unwrap()))
        .add_system(bevy::window::close_on_esc)
        // .add_system(display_events)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: ResMut<Windows>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    commands.spawn(Camera2dBundle::default());

    // let ldtk_handle = asset_server.load("Rusty_Farm_World_2.ldtk");
    // let ldtk_map_size = Vec2::new(1280., 720.); // replace with the actual size of your LDtk map
    // let ldtk_center_offset = ldtk_map_size / 2.; // calculate the offset needed to center the map

    // commands.spawn(LdtkWorldBundle {
    //     ldtk_handle,
    //     transform: Transform::from_translation(-ldtk_center_offset.extend(0.)), // apply the offset to the position
    //     ..Default::default()
    // });
    
    let window = windows.get_primary_mut().unwrap();
    window.set_title("Rusty Farm".to_string());
}
