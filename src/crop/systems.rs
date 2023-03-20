use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::{bevy_animations::*, ldtk::*, OtherAssets, player::{Player, PlayerFootCollider, PlayerLargeCollider}, load_atlases::Atlases};
use rand::Rng;

use super::*;

pub fn spawn_crops(
    mut commands: Commands,
    crop_field_query: Query<(&CropField, &Transform)>,
    atlases: Res<Atlases>,
) {

    let handle = atlases.handles.get("corn_growth").unwrap().clone();

    for (_, transform) in crop_field_query.iter() {
        let new_transform = Transform::from_translation(Vec3::new(transform.translation.x + 10., transform.translation.y + 20., transform.translation.z + 3.));
        commands.spawn(SpriteSheetBundle {
            texture_atlas: handle.clone(),
            transform: new_transform.clone(),
            ..Default::default()
        })
            .insert(Crop::new(CropType::Corn))
            .insert(RigidBody::KinematicPositionBased)
            .insert(AnimationTimer(Timer::from_seconds(CropType::duration_from(&CropType::Corn), TimerMode::Repeating)))
            .with_children(|children| {
                children.spawn(SensorBundle {
                    collider: Collider::cuboid(1.5, 2.5),
                    sensor: Sensor,
                    ..Default::default()
                })
                    .insert(TransformBundle::from(Transform::from_xyz(0., -12., 0.)))
                    .insert(CropCollider)
                ;
            })
        ;
    }
}

/// check collisions for killing the crop
pub fn check_crop_foot_collisions(
    mut commands: Commands,
    player_query: Query<Entity, With<PlayerFootCollider>>,
    crop_collider_query: Query<(Entity, &Parent), With<CropCollider>>,
    mut crop_query: Query<(&mut TextureAtlasSprite, &mut Crop, &mut Handle<TextureAtlas>)>,
    context: Res<RapierContext>,
    atlases: Res<Atlases>
) {
    let player_foot_entity = player_query.single();

    let mut rng = rand::thread_rng();

    for (crop_collider_entity, crop_parent) in crop_collider_query.iter() {
        let (mut sprite, mut crop, mut texture) = match crop_query.get_mut(crop_parent.get()) {
            Ok(q) => q,
            Err(_) => continue
        };
        if let Some(_) = context.intersection_pair(player_foot_entity, crop_collider_entity) {
            let random_number = rng.gen_range(0..100);
            if random_number <= CROP_KILL_CHANCE && !crop.in_collision {
                sprite.index = 0;
                crop.crop_type = CropType::Dead;
                *texture = atlases.handles.get("dead_crop").unwrap().clone();
            } 
            crop.in_collision = true;
            continue;
        }
        crop.in_collision = false;
    }
}

/// check collisions for highlighing the crop
pub fn check_crop_collisions_to_highlight(
    mut player_query: Query<&mut Player>,
    player_collider: Query<Entity, With<PlayerLargeCollider>>,
    crop_collider_query: Query<(Entity, &Parent), With<CropCollider>>,
    mut crop_query: Query<(&mut Handle<TextureAtlas>, &Crop, Entity)>,
    context: Res<RapierContext>,
    atlases: Res<Atlases>,
) {

    let mut player = player_query.single_mut();
    // this entity is the large collider for the whole player
    let player_entity = player_collider.single();


    let mut collision = false;

    // first loop is for checking collisions and setting textures to highlighted
    for (crop_collider_entity, crop_parent) in crop_collider_query.iter() {
        let (mut texture, crop, texture_entity) = match crop_query.get_mut(crop_parent.get()) {
            Ok(q) => q,
            Err(_) => continue
        };

        if let Some(_) = context.intersection_pair(player_entity, crop_collider_entity) {
            if let Some(colliding_entity) = player.crop_colliding {
                if colliding_entity.index() != texture_entity.index() {
                    player.crop_colliding = Some(texture_entity);
                    player.previous_crop_colliding = Some(colliding_entity);
                    *texture = atlases.handles.get(crop.crop_type.to_highlighted().atlas_name()).unwrap().clone();
                }
            }
            else {
                player.crop_colliding = Some(texture_entity);
                *texture = atlases.handles.get(crop.crop_type.to_highlighted().atlas_name()).unwrap().clone();
            }
            collision = true;
            break;  
        }
    }

    // set the texture of the previous colliding entity back to normal
    if collision {
        if let Some(previous_colliding_entity) = player.previous_crop_colliding {
            let (mut texture, crop, _) = match crop_query.get_mut(previous_colliding_entity) {
                Ok(q) => q,
                Err(e) => {
                    println!("{}", e);
                    return;
                }
            };
            player.previous_crop_colliding = None;
            *texture = atlases.handles.get(crop.crop_type.to_normal().atlas_name()).unwrap().clone();
        }
    }
    else {
        let colliding_entity = match player.crop_colliding {
            Some(entity) => entity,
            None => return
        };
        let (mut texture, crop, _) = match crop_query.get_mut(colliding_entity) {
            Ok(q) => q,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };
        player.previous_crop_colliding = None;
        player.crop_colliding = None;
        *texture = atlases.handles.get(crop.crop_type.to_normal().atlas_name()).unwrap().clone();    
    }
}

/// system for cycling the lifetime of the crop
pub fn crop_liftime (
    mut crop_query: Query<(&mut AnimationTimer, &mut Crop, &mut TextureAtlasSprite,)>,
    time: Res<Time>
) {
    for (mut timer, mut crop, mut sprite) in crop_query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() && crop.crop_type != CropType::Dead && crop.crop_type != CropType::DeadHighlighted {
            if crop.stage + 1 > 5 {
                return;
            }
            sprite.index = crop.stage - 1;
            crop.stage += 1;
            timer.reset();
        }
    }
}