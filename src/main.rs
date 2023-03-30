use bevy::{prelude::*, log};
use bevy_asset_loader::prelude::*;
use load_atlases::load_altases;
use mechanics::perspective::PerspectiveMechanicsPlugin;
use save::SavePlugin;
use ui::UIPlugin;
// use bevy_animations::*;
use crate::{bevy_animations::*, player::*, crop::*, mechanics::*};
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
mod load_atlases;
mod ui;
mod mechanics;
mod save;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    LoadingAssets,
    LoadingAtlases,
    LoadingLdtk,
    LoadingSave,
    LoadingNewGame,
    LoadingAnimations,
    LoadingGameMenu,
    LoadingGame,
    Game,
    LoadingInventory,
    Inventory,
    LoadingPause,
    Pause,
    LoadingMainMenu,
    MainMenu,
    Unload,
    Saving,
}

#[derive(Resource, Default, PartialEq, Eq)]
pub struct NextState(GameState);

#[derive(Debug, Resource, PartialEq, Eq)]
pub enum OldState {
    GameState(GameState),
    NextState(GameState)
}

impl OldState {
    fn default() -> Self {
        Self::GameState(GameState::default())
    }
    fn game_state(&self) -> Option<&GameState> {
        match self {
            OldState::GameState(game_state) => Some(game_state),
            _ => None
        }
    }
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
pub struct IconAssets {
    #[asset(path = "icons/potato.png")]
    potato: Handle<Image>,
    #[asset(path = "icons/cabbage.png")]
    cabbage: Handle<Image>,
    #[asset(path = "icons/corn.png")]
    corn: Handle<Image>,
    #[asset(path = "icons/carrot.png")]
    carrot: Handle<Image>,
    #[asset(path = "icons/backpack.png")]
    backpack: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct OtherAssets {
    #[asset(path = "farmer/farming_animation.png")]
    player_farming: Handle<Image>,
    #[asset(path = "farmer/char_a_p1_0bas_humn_v00.png")]
    player: Handle<Image>,
    #[asset(path = "buildings/fence_gate.png")]
    gate: Handle<Image>,
    #[asset(path = "crops/corn_growth.png")]
    corn_growth: Handle<Image>,
    #[asset(path = "crops/corn_growth_highlighted.png")]
    corn_growth_highlighted: Handle<Image>,
    #[asset(path = "crops/dead_crop.png")]
    dead_crop: Handle<Image>,
    #[asset(path = "crops/dead_crop_highlighted.png")]
    dead_crop_highlighted: Handle<Image>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_plugin(CropPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(LdtkPlugin)
        .add_plugin(FarmWorldPlugin)
        .add_plugin(PerspectiveMechanicsPlugin)
        .add_plugin(SavePlugin)
        .add_state::<GameState>(GameState::default())
        .add_loading_state(
            LoadingState::new(GameState::LoadingAssets)
                .continue_to_state(GameState::LoadingAtlases)
                .with_collection::<LdtkAssets>()
                .with_collection::<OtherAssets>()
                .with_collection::<IconAssets>()
        )
        .add_system_set(SystemSet::on_enter(GameState::LoadingAtlases)
            .with_system(load_altases)
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(20.0))
        .add_plugin(AnimationsPlugin {
            pixels_per_meter: 20.
        })
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .insert_resource(NextState::default())
        .insert_resource(OldState::default())
        .add_startup_system(setup)
        .add_system(debug_state)
        // .add_system(display_events)
        .run();
}

fn setup(mut commands: Commands, mut windows: ResMut<Windows>) {
    commands.spawn(Camera2dBundle::default());
    
    let window = windows.get_primary_mut().unwrap();
    window.set_title("Rusty Farm".to_string());
}

fn debug_state(
    game_state: Res<State<GameState>>,
    mut old_state: ResMut<OldState>
) {
    if *game_state.current() != *old_state.game_state().unwrap() {
        log::info!("state changed to {:?}", game_state.current());
        *old_state = OldState::GameState(game_state.current().clone()); 
    }
}