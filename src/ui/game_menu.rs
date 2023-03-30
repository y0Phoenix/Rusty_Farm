use bevy::prelude::*;
use crate::IconAssets;

use super::{*, colors::*};

pub const INVENTORY_COLS: usize = 18;
pub const INVENTORY_ROWS: usize = 9;
pub const INVENTORY_Y_SIZE: f32 = 350.;
pub const INVENTORY_X_SIZE: f32 = 600.;
pub const ICON_SIZE: f32 = 32.;

#[derive(Component)]
pub enum GameButton {
    Inventory
}
#[derive(Component)]
pub struct InventoryUi;

pub fn game_ui_interact_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &GameButton),
        Changed<Interaction>,
    >,
    mut inventory_ui: Query<&mut Visibility, With<InventoryUi>>,
    mut state: ResMut<State<GameState>>,
    mut next_state: ResMut<NextState>
) {
    for (interaction, mut color, button_type) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                match *button_type {
                    GameButton::Inventory => {
                        *color = PRESSED_BUTTON.into();
                        state.overwrite_set(GameState::Inventory).unwrap();
                        let mut inventory_ui = inventory_ui.single_mut();
                        *inventory_ui = Visibility::VISIBLE;
                    }
                }
            }
            Interaction::Hovered => {
                *color = Color::rgba(0.75, 0.75, 0.75, 1.).into();
            }
            Interaction::None => {
                *color = Color::rgba(1., 1., 1., 1.).into();
            }
        }
    }
}

pub fn game_menu_setup(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    icons: Res<IconAssets>
) {
    let main_inventory_node = NodeBundle {
        style: Style {
            size: Size::new(Val::Px(INVENTORY_X_SIZE), Val::Px(INVENTORY_Y_SIZE)),
            flex_wrap: FlexWrap::Wrap,
            position_type: PositionType::Absolute,
            position: UiRect::new(Val::Percent(27.), Val::Auto, Val::Percent(30.), Val::Auto),
            ..Default::default()
        },
        visibility: Visibility::INVISIBLE,
        background_color: Color::hex("CC6600").unwrap().into(),
        ..Default::default()
    };

    let inventory_col_node = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Px(34.)),
            margin: UiRect::vertical(Val::Px(1.)),
            ..Default::default()
        },
        ..Default::default()
    };

    let inventory_node = NodeBundle {
        style: Style {
            size: Size::new(Val::Px(ICON_SIZE), Val::Px(ICON_SIZE)),
            margin: UiRect::all(Val::Px(1.)),
            ..Default::default()
        },
        background_color: NORMAL_BUTTON.into(),
        ..Default::default()
    };

    let inventory_button_node = NodeBundle {
        style: Style {
            size: Size::new(Val::Px(ICON_SIZE), Val::Px(ICON_SIZE)),
            position_type: PositionType::Absolute,
            position: UiRect::new(Val::Percent(49.), Val::Auto, Val::Auto, Val::Percent(1.)),
            padding: UiRect::bottom(Val::Px(10.)),
            ..default()
        },
        background_color: NORMAL_BUTTON.into(),
        ..default()
    };

    let inventory_button = ImageBundle {
        image: UiImage(icons.backpack.clone()),
        calculated_size: CalculatedSize { 
            size: Size::new(Val::Px(ICON_SIZE), Val::Px(ICON_SIZE)) 
        },
        style: Style {
            size: Size::new(Val::Px(ICON_SIZE), Val::Px(ICON_SIZE)),
            ..Default::default()
        },
        ..Default::default()
    };

    // spawn buttons
    commands.spawn(inventory_button_node)
        .with_children(|parent| {
            parent.spawn(inventory_button)
                .insert(Interaction::default())
                .insert(GameButton::Inventory)
            ;
        })
    ;

    // spawn inventory
    commands.spawn(main_inventory_node)
        .insert(InventoryUi)
        .with_children(|parent| {
            for _ in 0..INVENTORY_ROWS {
                let mut col_node = parent.spawn(inventory_col_node.clone());
                for _ in 0..INVENTORY_COLS {
                    col_node.with_children(|parent| {
                        parent.spawn(inventory_node.clone())
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                image: UiImage(icons.corn.clone()),
                                ..Default::default()
                            });
                        });
                    });
                }
            }
        })
    ;
    state.overwrite_set(GameState::LoadingGame).unwrap();
}