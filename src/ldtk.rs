use bevy::prelude::*;
// use bevy_animations::*;
use crate::{gate::*, animations::*, player::*, path::*};
use bevy_ecs_ldtk::{prelude::*, ldtk::Level};

use bevy_rapier2d::prelude::*;
pub struct FarmWorldPlugin;

impl Plugin for FarmWorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_entity::<LdtkPlayer>("Player")
            .register_ldtk_entity::<LdtkGate>("Gate")
            .register_ldtk_int_cell::<FenceBundle>(1)
            .register_ldtk_int_cell_for_layer::<PathBundle>("Paths", 1)
            .register_ldtk_int_cell_for_layer::<PathBundle>("Paths", 2)
            .add_startup_system(spawn_world)
            .insert_resource(LevelSelection::Identifier("Main_Farm".to_string()))
            .insert_resource(CurrentLevel::default())
            .add_system(set_animations)
            
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
}

impl LdtkIntCell for FenceBundle {
    fn bundle_int_cell(_: IntGridCell, _: &LayerInstance) -> Self {
        Self {
            fence: Fence, 
            collider_bundle: ColliderBundle { 
                collider: Collider::cuboid(8., 4.), 
                rigid_body: RigidBody::Fixed,
                ..Default::default()
            } 
        }
    }
}

fn spawn_world(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    let ldtk_handle = asset_server.load("Rusty_Farm_World.ldtk");

    let ldtk_world = LdtkWorldBundle {
            ldtk_handle,
            ..Default::default()
    };

    commands.spawn(ldtk_world);
}