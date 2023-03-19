use bevy_ecs_ldtk::prelude::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::{ldtk::*, bevy_animations::*};

#[derive(Debug, Component, Clone, Default)]
pub struct Gate {
    pub in_collision: bool,
    pub open: bool
}

#[derive(Clone, Default, Bundle)]
pub struct LdtkGate {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub gate: Gate,
    #[bundle]
    pub sensor_bundle: SensorBundle,
    entity_instance: EntityInstance,
    pub direction: AnimationDirection,
    pub ldtk: Ldtk
}

impl LdtkEntity for LdtkGate {
    fn bundle_entity(
            entity_instance: &EntityInstance,
            _: &LayerInstance,
            _: Option<&Handle<Image>>,
            _: Option<&TilesetDefinition>,
            asset_server: &AssetServer,
            texture_atlases: &mut Assets<TextureAtlas>,
        ) -> Self {
        let texture = asset_server.load("buildings/fence_gate.png");

        let texture_atlas = TextureAtlas::from_grid(
            texture,
            Vec2::new(32., 50.),
            3,
            1,
            None,
            Some(Vec2::new(16., 16.))
        );

        let handle = texture_atlases.add(texture_atlas);

        Self { 
            gate: Gate::default(), 
            sensor_bundle: SensorBundle { 
                collider: Collider::cuboid(16., 16.), 
                sensor: Sensor,
                ..Default::default()
            },
            sprite_sheet_bundle: SpriteSheetBundle { 
                texture_atlas: handle,
                ..Default::default()
            },
            entity_instance: entity_instance.clone(),
            direction: AnimationDirection::default() ,
            ldtk: Ldtk
        }
    }
}