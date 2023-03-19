use bevy::prelude::*;
// use bevy_animations::*;
use crate::{gate::*, animations::*, player::*, path::*, crop::*, LdtkAssets, GameState};
use bevy_ecs_ldtk::{prelude::*, ldtk::Level};

use bevy_rapier2d::prelude::*;

#[derive(Debug, Clone, Component, Default)]
pub struct Ldtk;
pub struct FarmWorldPlugin;

impl Plugin for FarmWorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_entity::<LdtkPlayer>("Player")
            .register_ldtk_entity::<LdtkGate>("Gate")
            .register_ldtk_int_cell::<FenceBundle>(1)
            .register_ldtk_int_cell_for_layer::<PathBundle>("Paths", 1)
            .register_ldtk_int_cell_for_layer::<PathBundle>("Paths", 2)
            .register_ldtk_int_cell_for_layer::<CropFieldBundle>("Paths", 3)
            .add_system_set(SystemSet::on_enter(GameState::LoadingLdtk)
                .with_system(spawn_world)
            )
            .add_system_set(SystemSet::on_update(GameState::LoadingLdtk)
                .with_system(check_ldtk_entities)
            )
            .add_system_set(SystemSet::on_enter(GameState::LoadingAnimations)
                .with_system(set_animations)
            )
            .insert_resource(LevelSelection::Identifier("Main_Farm".to_string()))
            .insert_resource(CurrentLevel::default())
            // .add_system(set_animations)
        ;
    }
}

#[derive(Debug, Resource, Clone, Default)]
pub struct CurrentLevel(Level);

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub damping: Damping,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderMassProperties,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct SensorBundle {
    pub collider: Collider,
    pub sensor: Sensor,
    pub active_events: ActiveEvents,
    pub rotation_constraints: LockedAxes,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Fence;

#[derive(Clone, Debug, Default, Bundle)]
pub struct FenceBundle {
    fence: Fence,
    #[bundle]
    pub collider_bundle: ColliderBundle,
    pub ldtk: Ldtk
}

impl LdtkIntCell for FenceBundle {
    fn bundle_int_cell(_: IntGridCell, _: &LayerInstance) -> Self {
        Self {
            fence: Fence, 
            collider_bundle: ColliderBundle { 
                collider: Collider::cuboid(8., 4.), 
                rigid_body: RigidBody::Fixed,
                ..Default::default()
            } ,
            ldtk: Ldtk
        }
    }
}

fn spawn_world(
    mut commands: Commands, 
    ldtk_assets: Res<LdtkAssets>,
) {
    let ldtk_handle = ldtk_assets.ldtk_world.clone();

    let ldtk_world = LdtkWorldBundle {
            ldtk_handle,
            ..Default::default()
    };

    commands.spawn(ldtk_world);
}

fn check_ldtk_entities(
    query: Query<&Ldtk>,
    mut state: ResMut<State<GameState>>
) {
    if !query.is_empty() {
        state.set(GameState::LoadingAnimations).expect("Something Went Wrong Changing State To Loaded");
        return;
    }
}