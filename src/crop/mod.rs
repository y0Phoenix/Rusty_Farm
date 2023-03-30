use std::time::Duration;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;
use serde::{Serialize, Deserialize};
use crate::{bevy_animations::*, ldtk::*, crop::systems::*, GameState, save::Savable, mechanics::perspective::SecondaryPerspectiveBody};

// the crops chance to die from the player stepping on it
pub const CROP_KILL_CHANCE: i32 = 30;

pub mod systems;
pub struct CropPlugin;

impl Plugin for CropPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(GameState::Game)
                .with_system(check_crop_foot_collisions.label("foot"))
                .with_system(check_crop_collisions_to_highlight.after("foot").label("highlight"))
                .with_system(crop_liftime.after("highlight"))
            )
            .add_system_set(SystemSet::on_enter(GameState::LoadingNewGame)
                .with_system(spawn_crops)
            )
        ;
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct CropField;

#[derive(Clone, Default, Bundle)]
pub struct CropFieldBundle {
    #[bundle]
    pub crop_field: CropField,
    pub ldtk: Ldtk
}

impl LdtkIntCell for CropFieldBundle {
    fn bundle_int_cell(_: IntGridCell, _: &LayerInstance) -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Component, Default)]
pub enum CropType {
    #[default]
    Potato,
    PotatoHighlighted,
    CarrotHighlighted,
    Carrot,
    CornHighlighted,
    Corn,
    CabbageHighlighted,
    Cabbage,
    Dead,
    DeadHighlighted
}

impl CropType {
    pub fn atlas_name(&self) -> &'static str {
        match self {
            CropType::Potato => "potato_growth",
            CropType::PotatoHighlighted => "potato_growth_highlighted",
            CropType::Carrot => "carrot_growth",
            CropType::CarrotHighlighted => "carrot_growth_highlighted",
            CropType::Corn => "corn_growth",
            CropType::CornHighlighted => "corn_growth_highlighted",
            CropType::Cabbage => "cabbage_growth",
            CropType::CabbageHighlighted => "cabbage_growth_highlighted",
            CropType::Dead => "dead_crop",
            CropType::DeadHighlighted => "dead_crop_highlighted"
        }
    }
    pub fn to_highlighted(&self) -> Self {
        match self {
            CropType::Potato => CropType::PotatoHighlighted,
            CropType::PotatoHighlighted => CropType::PotatoHighlighted,
            CropType::Carrot => CropType::CarrotHighlighted,
            CropType::CarrotHighlighted => CropType::CarrotHighlighted,
            CropType::Corn => CropType::CornHighlighted,
            CropType::CornHighlighted => CropType::CornHighlighted,
            CropType::Cabbage => CropType::CabbageHighlighted,
            CropType::CabbageHighlighted => CropType::CabbageHighlighted,
            CropType::Dead => CropType::DeadHighlighted,
            CropType::DeadHighlighted => CropType::DeadHighlighted
        }
    }
    pub fn to_normal(&self) -> Self {
        match self {
            CropType::Potato => CropType::Potato,
            CropType::PotatoHighlighted => CropType::Potato,
            CropType::Carrot => CropType::Carrot,
            CropType::CarrotHighlighted => CropType::Carrot,
            CropType::Corn => CropType::Corn,
            CropType::CornHighlighted => CropType::Corn,
            CropType::Cabbage => CropType::Cabbage,
            CropType::CabbageHighlighted => CropType::Cabbage,
            CropType::Dead => CropType::Dead,
            CropType::DeadHighlighted => CropType::Dead
        }
    }
    pub fn duration(&self) -> f32 {
        let mut rng = rand::thread_rng();
        match self {
            CropType::Potato => rng.gen_range(50..75) as f32,
            CropType::PotatoHighlighted => rng.gen_range(50..75) as f32,
            CropType::Carrot => rng.gen_range(80..100) as f32,
            CropType::CarrotHighlighted => rng.gen_range(80..100) as f32,
            CropType::Corn => rng.gen_range(45..65) as f32,
            CropType::CornHighlighted => rng.gen_range(45..65) as f32,
            CropType::Cabbage => rng.gen_range(100..125) as f32,
            CropType::CabbageHighlighted => rng.gen_range(100..125) as f32,
            CropType::Dead => 1.,
            CropType::DeadHighlighted => 1.,
        }
    }
    pub fn duration_from(value: &CropType) -> f32 {
        let mut rng = rand::thread_rng();
        match value {
            CropType::Potato => rng.gen_range(50..75) as f32,
            CropType::PotatoHighlighted => rng.gen_range(50..75) as f32,
            CropType::Carrot => rng.gen_range(80..100) as f32,
            CropType::CarrotHighlighted => rng.gen_range(80..100) as f32,
            CropType::Corn => rng.gen_range(45..65) as f32,
            CropType::CornHighlighted => rng.gen_range(45..65) as f32,
            CropType::Cabbage => rng.gen_range(100..125) as f32,
            CropType::CabbageHighlighted => rng.gen_range(100..125) as f32,
            CropType::Dead => 1.,
            CropType::DeadHighlighted => 1.,
        }
    }
}

#[derive(Component, Serialize, Deserialize, Debug, Clone, Default)]
pub struct Crop {
    pub stage: usize,
    pub crop_type: CropType,
    pub in_collision: bool,
}

impl Crop {
    fn new(crop_type: CropType) -> Self {
        Self { 
            stage: 1, 
            crop_type,
            in_collision: false
        }
    }
}

#[derive(Bundle, Default)]
pub struct CropBundle {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    #[bundle]
    pub sensor_bundle: SensorBundle,
    pub savable: Savable,
    pub secondary_perpective_body: SecondaryPerspectiveBody,
    pub rigid_body: RigidBody,
    pub animation_timer: AnimationTimer,
    pub crop: Crop
}

#[derive(Bundle)]
pub struct SmallCropColliderBundle {
    sensor_bundle: SensorBundle,
    transform_bundle: TransformBundle,
    crop_collider: CropCollider
}

impl Default for SmallCropColliderBundle {
    fn default() -> Self {
        Self { 
            sensor_bundle: SensorBundle {
                collider: Collider::cuboid(1.5, 2.5),
                sensor: Sensor,
                ..Default::default()
            }, 
            transform_bundle: TransformBundle::from(Transform::from_xyz(0., -12., 0.)), 
            crop_collider: CropCollider 
        }
    }
}

#[derive(Component)]
pub struct CropCollider;

#[derive(Component)]
pub struct CropTexture;

