use bevy::prelude::*;
use bevy_animations::{Animations, AnimationType, TransformAnimation, AnimationDirectionIndexes, AnimationEvent, AnimationDirection, TimedAnimation};
use bevy_rapier2d::prelude::{Collider, RigidBody, ExternalForce, Damping, ActiveEvents};

use crate::{AnimationTimer, EntityVelocity, MoveDirection, crop::{CropCollider, self}};

const WALKING_ANIMATION_FRAMES: [usize; 6] = [0, 1, 2, 3, 4, 5];
const RUNNUNG_ANIMATION_FRAMES: [usize; 6] = [0, 1, 6, 3, 4, 7];
const HARVESTING_ANIMATION_FRAMES: [usize; 4] = [0, 1, 2, 3]; 

pub struct Movement {
    running: bool,
    velocity: EntityVelocity
}

impl Movement {
    fn default() -> Self {
        Self { running: false, velocity: EntityVelocity(Vec2::new(0.85, 0.85)) }
    }
    fn set_running(&mut self) {
        self.running = true;
        self.velocity = EntityVelocity(Vec2::new(1.25, 1.25))
    }
    fn set_walking(&mut self) {
        self.running = false;
        self.velocity = EntityVelocity(Vec2::new(0.85, 0.85))
    } 
    fn is_running(&self) -> bool {
        self.running
    }
}

#[derive(Component)]
pub struct Player {
    movement: Movement,
}

impl Player {
    fn new() -> Self {
        Self { 
            movement: Movement::default(),
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(player_setup)
            .add_system(check_crop_collision.label("check_crop_collision"))
            .add_system(move_player.after("chech_crop_collsion"))
        ;
    }
}

fn player_setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut animations: ResMut<Animations>
) {
    let farmer = asset_server.load("farmer/char_a_p1_0bas_humn_v00.png");
    let farming = asset_server.load("farmer/farming_animation.png");
    let farmer_texture_atlas = TextureAtlas::from_grid(farmer, Vec2::new(64., 64.), 8, 8, None, None);
    let farming_texture_atlas = TextureAtlas::from_grid(farming, Vec2::new(64., 64.), 4, 4, None, None);
    let farmer_atlas_handle = texture_atlases.add(farmer_texture_atlas);
    let farming_atlas_handle = texture_atlases.add(farming_texture_atlas);
    let entity = commands.spawn((
        SpriteSheetBundle {
            texture_atlas: farmer_atlas_handle.clone(),
            transform: Transform::from_translation(Vec3::new(-100., -100., 0.)),
            ..Default::default()
        },
        Player::new(),
        AnimationTimer(Timer::from_seconds(0.25, TimerMode::Repeating)),
        Collider::cuboid(8., 18.),
        RigidBody::Dynamic,
        ExternalForce {
            force: Vec2::ZERO,
            torque: 0.
        },
        Damping {
            linear_damping: 100.,
            angular_damping: 100.
        },
        ActiveEvents::COLLISION_EVENTS,
        AnimationDirection::default()
    )).id();

    animations
        .insert_animation(entity, AnimationType::Transform(
            TransformAnimation::new(
                Vec::from(WALKING_ANIMATION_FRAMES), 
                0.65, 
                farmer_atlas_handle.clone(), 
                Vec2::new(8., 8.), 
                AnimationDirectionIndexes::new(8, 7, 6, 5), 
                true
            ), 
            "player_walking"
            )
        )
        .insert_animation(entity, AnimationType::Transform(
            TransformAnimation::new(
                Vec::from(RUNNUNG_ANIMATION_FRAMES), 
                0.70, 
                farmer_atlas_handle, 
                Vec2::new(8., 8.), 
                AnimationDirectionIndexes::new(8, 7, 6, 5), 
                true
            ), 
            "player_running"
            )
        )
        .insert_animation(entity, AnimationType::Timed(
            TimedAnimation::new(
                Vec::from(HARVESTING_ANIMATION_FRAMES), 
                vec![0.001, 0.300, 0.350, 0.375], 
                farming_atlas_handle, 
                Vec2::new(4., 4.), 
                AnimationDirectionIndexes::new(4, 3, 2, 1), 
                false, 
                true, 
                1
            ), 
            "player_harvesting"
            )
        )
    ;
}

fn check_crop_collision(
    mut commands: Commands,
    mut crop_collider: ResMut<CropCollider>,
    player_query: Query<Entity, With<Player>>,
    inputs: Res<Input<KeyCode>>,
    mut event_writer: EventWriter<AnimationEvent>
) {
    let inputs = inputs.get_pressed();

    let mut input = false;

    for key in inputs {
        if *key == KeyCode::Space {
            input = true;
        }
    }

    if input {
        if let Some(crop_entity) = crop_collider.collider {
            event_writer.send(AnimationEvent("player_harvesting", player_query.single()));
            commands.entity(crop_entity).despawn();
            *crop_collider = CropCollider::default();
        } 
    }
}

fn move_player(
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut AnimationTimer,
        &mut Player,
        &mut AnimationDirection
    )>, 
    keyboard_input: Res<Input<KeyCode>>,
    mut event_writer: EventWriter<AnimationEvent>
) {
    let (entity, mut transform, mut timer, mut player, mut direction) = query.single_mut();
    timer.tick(time.delta());

    let inputs = keyboard_input.get_pressed();

    player.movement.set_walking();

    let mut dir = AnimationDirection::Still;

    inputs.for_each(|key| {
        match *key {
            KeyCode::A => dir = AnimationDirection::Left,
            KeyCode::D => dir = AnimationDirection::Right,
            KeyCode::W => dir = AnimationDirection::Up,
            KeyCode::S => dir = AnimationDirection::Down,
            KeyCode::LShift => player.movement.set_running(),
            _ => {}
        }
    });

    *direction = dir;

    if player.movement.is_running() {
        event_writer.send(AnimationEvent("player_running", entity));
    }
    else {
        event_writer.send(AnimationEvent("player_walking", entity));
    }

    let translation = transform.translation;
    *transform = Transform::from_translation(Vec3::new(
    translation.x + AnimationDirection::get_direction(&direction).x * player.movement.velocity.x, translation.y + AnimationDirection::get_direction(&direction).y * player.movement.velocity.y, 0.));
    
}