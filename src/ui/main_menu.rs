use bevy::{prelude::*, app::AppExit};
use crate::NextState;

use super::{*, colors::*};

#[derive(Debug, Component, PartialEq, Eq)]
pub enum MainMenuButton {
    PlayGame,
    Exit
}

#[derive(Component)]
pub struct MainMenu;

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MainMenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<State<GameState>>,
    mut next_state: ResMut<NextState>,
    mut app_exit: ResMut<Events<AppExit>>
) {
    for (interaction, mut color, main_menu_button) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                match *main_menu_button {
                    MainMenuButton::PlayGame => {
                        state.overwrite_set(GameState::Unload).unwrap();
                        next_state.0 = GameState::LoadingLdtk;
                    },
                    MainMenuButton::Exit => {
                        app_exit.send(AppExit);
                    }
                }
            }
            Interaction::Hovered => {
                // match *main_menu_button {
                //     MainMenuButton::PlayGame => *color = PLAY_GAME_BUTTON_HOVER.into(),
                //     MainMenuButton::Exit => *color = NORMAL_BUTTON_HOVER.into()
                // }
                *color = NORMAL_BUTTON_HOVER.into();
            }
            Interaction::None => {
                // match *main_menu_button {
                //     MainMenuButton::PlayGame => *color = PLAY_GAME_BUTTON.into(),
                //     MainMenuButton::Exit => *color = NORMAL_BUTTON.into()
                // }
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn main_menu_setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<GameState>>
) {
    commands
        .spawn(
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(650.)),
                    padding: UiRect::vertical(Val::Px(100.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            }
        )
        .insert(MainMenu)
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Px(65.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(MainMenuButton::PlayGame)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play Game",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: PLAY_GAME_BUTTON.into(),
                        },
                    ));
                })
            ;
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Px(65.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(MainMenuButton::Exit)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Exit",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::RED,
                        },
                    ));
                })
            ;
        });
    state.overwrite_set(GameState::MainMenu).unwrap();
}