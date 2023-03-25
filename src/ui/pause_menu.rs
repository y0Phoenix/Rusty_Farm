use bevy::{prelude::*, app::AppExit};
use crate::NextState;

use super::{*, colors::*};

// UI component for Pause menu
#[derive(Component, Default)]
pub struct PauseMenu;

pub fn check_pause_input(
    inputs: Res<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>
) {
    for key in inputs.get_pressed() {
        if key == &KeyCode::Escape {
            state.overwrite_set(GameState::LoadingPause).unwrap();
        }
    }
}

// setup_ui system - creates a new Pause menu entity and adds the PauseMenu component to it
pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<GameState>>
) {

    // Create pause menu entity
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Auto),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                position: UiRect::new(Val::Auto, Val::Auto, Val::Percent(25.), Val::Auto),
                ..default()
            },
            ..default()
        })
        .insert(PauseMenu)
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    size: Size::AUTO,
                    justify_content: JustifyContent::SpaceBetween,
                    flex_direction: FlexDirection::Column,
                    align_self: AlignSelf::Center,
                    ..Default::default()
                },
                ..Default::default()
            }).with_children(|parent| {

                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(400.0), Val::Px(65.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            margin: UiRect::vertical(Val::Px(25.)),
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Resume",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ));
                    })
                    .insert(PauseMenuItem::Resume)
                ;
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(400.0), Val::Px(65.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            margin: UiRect::vertical(Val::Px(25.)),
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Exit To Main Menu",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ));
                    })
                    .insert(PauseMenuItem::ExitToMain)
                ;
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(400.0), Val::Px(65.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            margin: UiRect::vertical(Val::Px(25.)),
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Exit To Desktop",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ));
                    })
                    .insert(PauseMenuItem::Exit)
                ;
            });
        });
    state.overwrite_set(GameState::Pause).unwrap();
}

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub enum PauseMenuItem {
    Resume,
    ExitToMain,
    Exit,
}

// handle_pause_menu_input system - handles input on the Pause menu
pub fn handle_pause_menu_input(
    mut commands: Commands,
    mut pause_menu_items: Query<
        (&Interaction, &mut BackgroundColor, &PauseMenuItem),
        (Changed<Interaction>, With<Button>)>,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
    mut app_state: ResMut<State<GameState>>,
    mut next_state: ResMut<NextState>,
    mut app_exit: ResMut<Events<AppExit>>
) {
    let pause_menu = pause_menu_query.single();

    for (interaction, mut color, menu_item) in pause_menu_items.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                match *menu_item {
                    // Handle Resume button click
                    PauseMenuItem::Resume => {
                        app_state.overwrite_set(GameState::Game).unwrap();
                        commands.entity(pause_menu).despawn_recursive();
                    },
                    // Handle Exit button click
                    PauseMenuItem::ExitToMain => {
                        app_state.overwrite_set(GameState::Unload).unwrap();
                        next_state.0 = GameState::LoadingMainMenu;
                        commands.entity(pause_menu).despawn_recursive();
                    },
                    // Handle Exit App
                    PauseMenuItem::Exit => {
                        app_exit.send(AppExit);
                    }
                }
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
