use bevy::prelude::*;
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
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::Center,
                padding: UiRect::bottom(Val::Px(10.)),
                ..default()
            },
            ..default()
        })
        .insert(PauseMenu)
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(75.0), Val::Px(35.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
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
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                })
                .insert(PauseMenuItem::Resume)
            ;
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(75.0), Val::Px(35.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Exit",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                })
                .insert(PauseMenuItem::Exit)
            ;
        });
    println!("setting to paused");
    state.overwrite_set(GameState::Pause).unwrap();
}

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub enum PauseMenuItem {
    Resume,
    Exit,
}

// handle_pause_menu_input system - handles input on the Pause menu
pub fn handle_pause_menu_input(
    mut commands: Commands,
    mut pause_menu_items: Query<
        (&Interaction, &mut BackgroundColor, &PauseMenuItem),
        (Changed<Interaction>, With<Button>)>,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
    text_query: Query<&Text>,
    mut app_state: ResMut<State<GameState>>,
    mut next_state: ResMut<NextState>
) {
    let pause_menu = pause_menu_query.single();

    for (interaction, color, menu_item) in pause_menu_items.iter_mut() {
        // Handle Resume button click
        if menu_item == &PauseMenuItem::Resume && interaction == &Interaction::Clicked {
            app_state.overwrite_set(GameState::Game).unwrap();
            commands.entity(pause_menu).despawn_recursive();
        }
        
        // Handle Exit button click
        if menu_item == &PauseMenuItem::Exit && interaction == &Interaction::Clicked {
            app_state.overwrite_set(GameState::Unload).unwrap();
            next_state.0 = GameState::LoadingMainMenu;
            commands.entity(pause_menu).despawn_recursive();
        }
    }
}
