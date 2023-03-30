use std::{fs::{File, read_to_string}, io::BufReader};

use bevy::{prelude::*, app::AppExit, ecs::schedule::ShouldRun};
use crate::{NextState, save::{SaveGames, SaveName}};

use super::{*, colors::*};

pub const MAIN_MENU_BUTTON_WIDTH: f32 = 250.;
pub const MAIN_MENU_BUTTON_HEIGHT: f32 = 65.;
pub const LOAD_MENU_BUTTON_WIDTH: f32 = 350.;
pub const LOAD_MENU_BUTTON_HEIGHT: f32 = 65.;

#[derive(Debug, Component, PartialEq, Eq)]
pub enum MainMenuButton {
    NewGame,
    LoadGame,
    Exit
}

#[derive(Debug, Component, PartialEq, Eq)]
pub struct LoadGameButton(pub String);

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct LoadGameMenu;

pub fn landing_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MainMenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<State<GameState>>,
    mut next_state: ResMut<NextState>,
    mut app_exit: ResMut<Events<AppExit>>,
    mut main_menu_state: ResMut<State<MainMenuState>>
) {
    for (interaction, mut color, main_menu_button) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                match *main_menu_button {
                    MainMenuButton::NewGame => {
                        state.overwrite_set(GameState::Unload).unwrap();
                        next_state.0 = GameState::LoadingLdtk;
                        main_menu_state.overwrite_set(MainMenuState::NotActive).unwrap();
                    },
                    MainMenuButton::Exit => {
                        app_exit.send(AppExit);
                    },
                    MainMenuButton::LoadGame => {
                        state.overwrite_set(GameState::Unload).unwrap();
                        next_state.0 = GameState::MainMenu;
                        main_menu_state.overwrite_set(MainMenuState::LoadingLoadGame).unwrap();
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
    mut game_state: ResMut<State<GameState>>,
    mut main_menu_state: ResMut<State<MainMenuState>>
) {
    let main_menu_button = ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(MAIN_MENU_BUTTON_WIDTH), Val::Px(MAIN_MENU_BUTTON_HEIGHT)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: NORMAL_BUTTON.into(),
        ..default()
    };

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
                .spawn(main_menu_button.clone())
                .insert(MainMenuButton::NewGame)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play New Game",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: PLAY_GAME_BUTTON.into(),
                        },
                    ));
                })
            ;
            parent
                .spawn(main_menu_button.clone())
                .insert(MainMenuButton::LoadGame)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Load Game",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: PLAY_GAME_BUTTON.into(),
                        },
                    ));
                })
            ;
            parent
                .spawn(main_menu_button.clone())
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
    game_state.overwrite_set(GameState::MainMenu).unwrap();
    main_menu_state.overwrite_set(MainMenuState::Landing).unwrap();
}

pub fn load_game_menu_setup(
    mut commands: Commands,
    mut main_menu_state: ResMut<State<MainMenuState>>,
    asset_server: Res<AssetServer>
) {
    let save_games = read_to_string("saves/save_games.rson").expect("Save Games File Not Found");

    let save_games = ron::from_str::<SaveGames>(&save_games.as_str()).unwrap();

    let main_menu_button = ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(LOAD_MENU_BUTTON_WIDTH), Val::Px(LOAD_MENU_BUTTON_HEIGHT)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: NORMAL_BUTTON.into(),
        ..default()
    };

    commands.spawn(
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
        .with_children(|parent| {
            for save_game in save_games.0.iter() {
            parent.spawn(main_menu_button.clone())
                .insert(LoadGameButton(save_game.name.clone()))
                .with_children(|parent| {
                        parent.spawn(
                            TextBundle::from_section(
                                save_game.date.as_str(),
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.,
                                    color: Color::WHITE
                                }
                            )
                        );
                    })
                ;
            }
            if save_games.0.len() == 0 {
                let mut new_button = main_menu_button.clone();
                new_button.style.size = Size::new(Val::Px(LOAD_MENU_BUTTON_WIDTH + 100.), Val::Px(LOAD_MENU_BUTTON_HEIGHT));
                parent.spawn(new_button)
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "No Save Games Found",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.,
                                color: Color::WHITE
                            }
                        ));
                    })
                ;
            }
    });
    main_menu_state.overwrite_set(MainMenuState::LoadGame).unwrap();
}

pub fn load_game_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &LoadGameButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<State<GameState>>,
    mut next_state: ResMut<NextState>,
    mut main_menu_state: ResMut<State<MainMenuState>>,
    mut save_name: ResMut<SaveName>,
    inputs: Res<Input<KeyCode>>
) {
    if inputs.pressed(KeyCode::Escape) {
        state.overwrite_set(GameState::Unload).unwrap();
        next_state.0 = GameState::LoadingMainMenu;
        main_menu_state.overwrite_set(MainMenuState::NotActive).unwrap();
        return;
    }

    for (interaction, mut color, button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::GREEN.into();
                *save_name = SaveName::Load(button.0.clone());
                state.overwrite_set(GameState::Unload).unwrap();
                next_state.0 = GameState::LoadingLdtk;
                main_menu_state.overwrite_set(MainMenuState::NotActive).unwrap();
            },
            Interaction::Hovered => {
                *color = NORMAL_BUTTON_HOVER.into();
            },
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}