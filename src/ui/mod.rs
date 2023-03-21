use bevy::{prelude::*, winit::WinitSettings};

use crate::{GameState, NextState};

use self::{main_menu::*, pause_menu::*, game_menu::*};

mod main_menu;
mod pause_menu;
mod game_menu;
mod colors;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            // .insert_resource(WinitSettings::desktop_app())
            .add_system_set(SystemSet::on_enter(GameState::LoadingMainMenu)
                .with_system(main_menu_setup)
            )
            .add_system_set(SystemSet::on_update(GameState::MainMenu)
                .with_system(button_system)
            )
            .add_system_set(SystemSet::on_enter(GameState::LoadingGameMenu)
                .with_system(game_menu_setup)
            )
            .add_system_set(SystemSet::on_update(GameState::Game)
                .with_system(game_button_system)
            )
            .add_system_set(SystemSet::on_enter(GameState::Unload)
                .with_system(unload)
            )
            .add_system_set(SystemSet::on_update(GameState::Game)
                .with_system(check_pause_input)
            )
            .add_system_set(SystemSet::on_enter(GameState::LoadingPause)
                .with_system(setup_ui)
            )
            .add_system_set(SystemSet::on_update(GameState::Pause)
                .with_system(handle_pause_menu_input)
            )
        ;
    }
}

#[derive(Component)]
pub struct UIButton;

pub fn unload(
    query: Query<Entity, Without<Camera2d>>,
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    mut next_state: ResMut<NextState>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    state.overwrite_set(next_state.0.clone()).unwrap();
    next_state.0 = GameState::default();
}