use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::{bevy_animations::*, ldtk::*, GameState};

use self::systems::*;

pub mod systems;

pub const PLAYER_WALKING_VEL: f32 = 0.90;
pub const PLAYER_RUNNUNG_VEL: f32 = 1.25;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::LoadingGame)
                .with_system(spawn_extra_colliders)
            )
            .add_system_set(SystemSet::on_update(GameState::Game)
                .with_system(movement)
                .with_system(check_gate_collisions)
                .with_system(center_camera_around_player)
            )
        ;
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player {
    pub crop_colliding: Option<Entity>,
    pub previous_crop_colliding: Option<Entity>,
}

#[derive(Component)]
pub struct PlayerFootCollider;

#[derive(Component)]
pub struct PlayerLargeCollider;

#[derive(Clone, Default, Bundle)]
pub struct LdtkPlayer {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub player: Player,
    #[bundle]
    pub collider_bundle: ColliderBundle,
    entity_instance: EntityInstance,
    pub direction: AnimationDirection,
    pub ldtk: Ldtk
}

impl LdtkEntity for LdtkPlayer {
    fn bundle_entity(
            entity_instance: &EntityInstance,
            _: &LayerInstance,
            _: Option<&Handle<Image>>,
            _: Option<&TilesetDefinition>,
            _: &AssetServer,
            _: &mut Assets<TextureAtlas>,
        ) -> LdtkPlayer {
        let fields = &entity_instance.field_instances;

        let mut linvel = 0.;

        for field in fields {
            if field.identifier == "velocity" {
                if let FieldValue::Float(Some(vel)) = field.value {
                    linvel = vel;
                }
            }
        }
        LdtkPlayer { 
            sprite_sheet_bundle: SpriteSheetBundle::default(),
            player: Player::default(),
            collider_bundle: ColliderBundle { 
                collider: Collider::cuboid(4., 18.), 
                rigid_body: RigidBody::Dynamic, 
                velocity: Velocity { linvel: Vec2::new(linvel, linvel), angvel: 0. }, 
                rotation_constraints: LockedAxes::ROTATION_LOCKED,
                damping: Damping {
                    linear_damping: 100.,
                    angular_damping: 100.
                },
                ..Default::default()
            },
            entity_instance: entity_instance.clone(),
            direction: AnimationDirection::default(),
            ldtk: Ldtk
        }
    }
}
