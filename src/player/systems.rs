use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::{gate::*, player::*};

pub fn check_gate_collisions(
    mut gate_query: Query<(
        Entity,
        &mut Gate
    )>,
    player_query: Query<(
        Entity,  
        &Player    
    )>,
    context: Res<RapierContext>,
    animations: Res<Animations>,
    mut animation_event: EventWriter<AnimationEvent>
) {
    // the player entity might not be loaded in yet
    let player_entity = match player_query.get_single() {
        Ok(entity) => entity.0,
        Err(_) => return
    };

    for gate_entity in gate_query.iter_mut() {
        let (gate_entity, mut gate) = gate_entity;

        if let Some(_) = context.intersection_pair(player_entity, gate_entity) {
            if let Some(in_animation) = animations.in_animation(gate_entity) {
                if !gate.open && !gate.in_collision && !in_animation {
                    animation_event.send(AnimationEvent("gate_opening", gate_entity));
                    gate.in_collision = true;
                    gate.open = true;
                }
            }
        }
        else if gate.in_collision && gate.open {
            animation_event.send(AnimationEvent("gate_closing", gate_entity));
            gate.in_collision = false;
            gate.open = false;
        }
    }
}

pub fn movement(
    input: Res<Input<KeyCode>>,
    mut player_query: Query<(
        Entity,
        &mut Transform,
        &mut Velocity,
        &mut AnimationDirection,
        &Player
    )>,
    level_query: Query<(&Handle<LdtkLevel>, &Transform), (Without<Player>, Without<Camera2d>)>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
    mut animation_event_writer: EventWriter<AnimationEvent>,
    mut reset_animation_event_writer: EventWriter<ResetAnimationEvent>,
    animations: ResMut<Animations>,
) {
    let (player_entity, mut transform, mut vel, mut direction, _) = match player_query.get_single_mut(){
        Ok(p) => p,
        Err(_) => return
    };

    // if we haven't initialized our animations yet
    if !animations.is_inserted(&player_entity) {
        return;
    }

    let inputs = input.get_pressed();

    let mut dir = AnimationDirection::default();

    let mut running = false;

    for key in inputs {
        match *key {
            KeyCode::A => dir = AnimationDirection::Left,
            KeyCode::S => dir = AnimationDirection::Down,
            KeyCode::D => dir = AnimationDirection::Right,
            KeyCode::W => dir = AnimationDirection::Up,
            KeyCode::LShift => {
                running = true;
                vel.linvel.x = PLAYER_RUNNUNG_VEL;
                vel.linvel.y = PLAYER_RUNNUNG_VEL;
            }
            _ => {}
        }
    }

    *direction = dir;

    if *direction != AnimationDirection::default() {
        if !running {
            vel.linvel.x = PLAYER_WALKING_VEL;
            vel.linvel.y = PLAYER_WALKING_VEL;
            animation_event_writer.send(AnimationEvent("player_walking", player_entity));
        } 
        else {
            animation_event_writer.send(AnimationEvent("player_running", player_entity));
        }

        let translation = transform.translation;

        for (level_handle, level_transform) in level_query.iter() {
            let level = ldtk_levels.get(level_handle).unwrap();
            let level_bounds = Rect {
                min: Vec2::new(level_transform.translation.x, level_transform.translation.y),
                max: Vec2::new(
                    level_transform.translation.x + level.level.px_wid as f32,
                    level_transform.translation.y + level.level.px_hei as f32,
                ),
            };
            let new_transform = Transform::from_translation(
                Vec3::new(
                    translation.x + AnimationDirection::get_direction(&direction).x * vel.linvel.x, 
                    translation.y + AnimationDirection::get_direction(&direction).y * vel.linvel.y, 
                    translation.z
                )
            );
            if new_transform.translation.x < level_bounds.max.x &&
                new_transform.translation.x > level_bounds.min.x &&
                new_transform.translation.y < level_bounds.max.y &&
                new_transform.translation.y > level_bounds.min.y 
            {
                *transform = new_transform;
            }
        }

    }
}

pub fn center_camera_around_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>, Without<Handle<LdtkLevel>>)>,
    windows: Res<Windows>,
    level_query: Query<(&Handle<LdtkLevel>, &Transform), (Without<Player>, Without<Camera2d>)>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let mut camera_transform = camera_query.single_mut();
        for (level_handle, level_transform) in &level_query {
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                if let Some(window) = windows.get_primary() {
                    let window_width = window.width() / 2.0;
                    let window_height = window.height() / 2.0;

                    let level_height = level_transform.translation.y + ldtk_level.level.px_hei as f32;
                    let level_width = level_transform.translation.x + ldtk_level.level.px_wid as f32;
                    
                    let new_camera_x = player_transform.translation.x;
                    let new_camera_y = player_transform.translation.y;
                    
                    // Clamp the camera's position within the map boundaries
                     // Clamp the camera's position within the map boundaries
                    camera_transform.translation.x = new_camera_x
                        .max(level_transform.translation.x + window_width)
                        .min(level_transform.translation.x + level_width - window_width);
                    camera_transform.translation.y = new_camera_y
                        .max(level_transform.translation.y + window_height)
                        .min(level_transform.translation.y + level_height - window_height);
                }
            }
        }
    }
}

pub fn spawn_extra_colliders(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>
) {
    let player_entity = player_query.single();
    commands.entity(player_entity)
        .with_children(|children| {
            // first sensor collider is for the feet
            children.spawn(SensorBundle {
                collider: Collider::cuboid(4., 1.),
                sensor: Sensor,
                ..Default::default()
            })
                .insert(PlayerFootCollider)
                .insert(TransformBundle::from(Transform::from_xyz(0., -8., 0.)))
            ;
            // second sensor collider is just a larger one mainly for crop collision detection
            children.spawn(SensorBundle {
                collider: Collider::cuboid(4., 24.),
                sensor: Sensor,
                ..Default::default()
            })
                .insert(PlayerLargeCollider)
            ;
        })
    ;
}