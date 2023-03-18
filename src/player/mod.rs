use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::{bevy_animations::*, ldtk::*};

use self::systems::*;

pub mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(movement)
            .add_system(check_gate_collisions)
            .add_system(center_camera_around_player)
        ;
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Clone, Default, Bundle)]
pub struct LdtkPlayer {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub player: Player,
    #[bundle]
    pub collider_bundle: ColliderBundle,
    entity_instance: EntityInstance,
    pub direction: AnimationDirection
}

impl LdtkEntity for LdtkPlayer {
    fn bundle_entity(
            entity_instance: &EntityInstance,
            _: &LayerInstance,
            _: Option<&Handle<Image>>,
            _: Option<&TilesetDefinition>,
            asset_server: &AssetServer,
            texture_atlases: &mut Assets<TextureAtlas>,
        ) -> LdtkPlayer {
        let texture = asset_server.load("farmer/char_a_p1_0bas_humn_v00.png");

        let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(64., 64.), 8, 8, None, None);

        let handle = texture_atlases.add(texture_atlas);

        let bundle = SpriteSheetBundle {
            texture_atlas: handle,
            ..Default::default()
        };

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
            sprite_sheet_bundle: bundle,
            player: Player,
            collider_bundle: ColliderBundle { 
                collider: Collider::cuboid(8., 18.), 
                rigid_body: RigidBody::Dynamic, 
                velocity: Velocity { linvel: Vec2::new(linvel, linvel), angvel: 0. }, 
                rotation_constraints: LockedAxes::ROTATION_LOCKED,
                damping: Damping {
                    linear_damping: 100.,
                    angular_damping: 100.
                },
                ..Default::default()
            } ,
            entity_instance: entity_instance.clone(),
            direction: AnimationDirection::default()
        }
    }
}
