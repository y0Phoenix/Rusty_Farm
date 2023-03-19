use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
// use bevy_animations::*;
use crate::{bevy_animations::*, player::*, crop::*};
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use ldtk::FarmWorldPlugin;

mod player;
mod path;
mod crop;
mod ldtk;
mod bevy_animations;
mod gate;
mod animations;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    LoadingAssets,
    LoadingLdtk,
    LoadingAnimations,
    Loaded
}

pub const EDGE_BUFFER: f32 = 25.;

/* A system that displays the events. */
fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}

#[derive(AssetCollection, Resource)]
pub struct LdtkAssets {
    #[asset(path = "Rusty_Farm_World.ldtk")]
    ldtk_world: Handle<LdtkAsset>
}

#[derive(AssetCollection, Resource)]
pub struct OtherAssets {
    #[asset(path = "farmer/farming_animation.png")]
    player_farming: Handle<Image>,
    #[asset(path = "farmer/char_a_p1_0bas_humn_v00.png")]
    player: Handle<Image>,
    #[asset(path = "buildings/fence_gate.png")]
    gate: Handle<Image>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_state::<GameState>(GameState::default())
        .add_loading_state(
            LoadingState::new(GameState::LoadingAssets)
                .continue_to_state(GameState::LoadingLdtk)
                .with_collection::<LdtkAssets>()
                .with_collection::<OtherAssets>()
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(20.0))
        .add_plugin(AnimationsPlugin {
            pixels_per_meter: 20.
        })
        .add_plugin(LdtkPlugin)
        .add_plugin(FarmWorldPlugin)
        // .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .add_plugin(PlayerPlugin)
        // .add_plugin(CropPlugin)
        // .add_system_set(SystemSet::on_enter(GameState::Loaded)
        //     .with_system(setup)
        // )
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        // .add_system(display_events)
        .run();
}

fn setup(mut commands: Commands, mut windows: ResMut<Windows>) {
    commands.spawn(Camera2dBundle::default());
    
    let window = windows.get_primary_mut().unwrap();
    window.set_title("Rusty Farm".to_string());
}
