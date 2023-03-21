use bevy::prelude::*;
use super::{*, colors::*};

pub fn game_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<State<GameState>>,
    mut next_state: ResMut<NextState>
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                state.overwrite_set(GameState::Unload).unwrap();
                next_state.0 = GameState::LoadingMainMenu;
            }
            Interaction::Hovered => {
                *color = NORMAL_BUTTON_HOVER.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn game_menu_setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<GameState>>
) {
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
        .insert(UIButton)
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
                        "Exit",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
    state.overwrite_set(GameState::LoadingGame).unwrap();
}