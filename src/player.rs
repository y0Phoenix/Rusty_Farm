use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody, ExternalForce, Damping, ActiveEvents};

use crate::{AnimationTimer, EntityVelocity, MoveDirection};


pub struct Movement {
    running: bool,
    velocity: EntityVelocity
}

impl Movement {
    fn default() -> Self {
        Self { running: false, velocity: EntityVelocity(Vec2::new(1., 1.)) }
    }
    fn set_running(&mut self) {
        self.running = true;
        self.velocity = EntityVelocity(Vec2::new(1.25, 1.25))
    }
    fn set_walking(&mut self) {
        self.running = false;
        self.velocity = EntityVelocity(Vec2::new(1., 1.))
    } 
    fn is_running(&self) -> bool {
        self.running
    }
}

#[derive(Component)]
struct Player {
    in_animation: bool,
    movement: Movement,
    animation_tick: usize,
    dir: MoveDirection
}

impl Player {
    fn new() -> Self {
        Self { 
            in_animation: false,
            movement: Movement::default(), 
            animation_tick: 0, 
            dir: MoveDirection::Still 
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(player_setup)
            .add_system(move_player)
        ;
    }
}

fn player_setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let farmer = asset_server.load("farmer/char_a_p1_0bas_humn_v00.png");
    let texture_atlas = TextureAtlas::from_grid(farmer, Vec2::new(64., 64.), 8, 8, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
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
        ActiveEvents::COLLISION_EVENTS
    ));
}

fn move_player(
    time: Res<Time>,
    mut query: Query<(
        &mut TextureAtlasSprite, 
        &mut Transform,
        &mut AnimationTimer,
        &mut Player
    )>, 
    keyboard_input: Res<Input<KeyCode>>
) {
    let (mut sprite, mut transform, mut timer, mut player) = query.single_mut();
    timer.tick(time.delta());
    if player.in_animation {
        return;
    }
    let inputs = keyboard_input.get_pressed();

    let mut dir = MoveDirection::Still;

    inputs.for_each(|key| {
        match *key {
            KeyCode::A => dir = MoveDirection::Left,
            KeyCode::D => dir = MoveDirection::Right,
            KeyCode::W => dir = MoveDirection::Up,
            KeyCode::S => dir = MoveDirection::Down,
            KeyCode::LShift => player.movement.set_running(),
            _ => {}
        }
    });

    if dir == MoveDirection::Still {
        let index = match player.dir {
            MoveDirection::Left => 3,
            MoveDirection::Right => 2,
            MoveDirection::Up => 1,
            MoveDirection::Down => 0,
            MoveDirection::Still => 0
        };
        sprite.index = index * 8;
        return;
    }

    player.dir = dir;

    let x_index = 
        if player.movement.is_running() {
            // if the player is starting to run 
            if player.animation_tick < 6 {
                6
            }
            // if the player is running cycle to second running sprite
            else if player.animation_tick == 6 {
                7
            }
            // if the player is running cycle to first sprite
            else {
                6
            }
        }
        else {
            // if reached the end of walking sprites
            if player.animation_tick == 5 || player.animation_tick > 5 {
                0
            }
            // if haven't reached end of walking sprites
            else {
                player.animation_tick + 1
            }
        };

        let y_index = 
            // if the player is moving left
            if dir == MoveDirection::Left {
                8
            }
            // if the player is moveing right
            else if dir == MoveDirection::Right {
                7
            }
            // if the player is moving up
            else if dir == MoveDirection::Up {
                6
            }
            // if the player is still or down 
            else {
                5
            } as usize;

    let translation = transform.translation;
    *transform = Transform::from_translation(Vec3::new(
        translation.x + MoveDirection::get_direction(&dir).x * player.movement.velocity.x, translation.y + MoveDirection::get_direction(&dir).y * player.movement.velocity.y, 0.));
    if timer.finished() {
        player.movement.set_walking();
        player.animation_tick = x_index;
        timer.reset();
        // println!("x: {}, y: {}, total: {}, dir: {:?}", x_index, y_index, y_index as i32 - x_index as i32, dir);
        sprite.index = (y_index * 8) - (8 - x_index);
    }
    else if player.dir != dir {
        sprite.index = y_index * 7;
    }
}