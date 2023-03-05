use bevy::prelude::*;
use crop::CropPlugin;
use player::PlayerPlugin;
use bevy_rapier2d::prelude::*;

mod player;
mod crop;

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
pub struct Velocity(Vec2);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(40.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .add_plugin(PlayerPlugin)
        .add_plugin(CropPlugin)
        .add_startup_system(setup)
        .insert_resource(ClearColor(Color::hex("005500").unwrap()))
        .add_system(bevy::window::close_on_esc)
        .add_system(display_events)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: ResMut<Windows>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    commands.spawn(Camera2dBundle::default());

    let texture = asset_server.load("fence.png");

    commands.spawn(SpriteBundle {
        texture,
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..Default::default()
    })
        .insert(Collider::cuboid(32., 24.))
        .insert(ExternalForce {
            force: Vec2::ZERO,
            ..Default::default()
        })
        .insert(Damping {
            linear_damping: 100.,
            angular_damping: 100.
        })
        .insert(RigidBody::KinematicPositionBased)
    ;

    let window = windows.get_primary_mut().unwrap();
    window.set_title("Rusty Farm".to_string());
}