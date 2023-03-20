use bevy::{prelude::*, winit::WinitSettings};

use crate::GameState;

use self::systems::{main_menu_setup, button_system, unload, game_menu_setup, game_button_system};

mod systems;

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            // .insert_resource(WinitSettings::desktop_app())
            .add_system_set(SystemSet::on_enter(GameState::LoadinMainMenu)
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
        ;
    }
}

#[derive(Component)]
pub struct UIButton;