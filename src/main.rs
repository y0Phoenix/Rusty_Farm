use bevy::prelude::*;

const EDGE_BUFFER: f32 = 25.;

const PLANT_SPACING: f32 = 32.;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Player;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .insert_resource(ClearColor(Color::hex("005500").unwrap()))
        .add_system(bevy::window::close_on_esc)
        .add_system(move_player)
        .run();
}

fn move_player(mut query: Query<(&mut TextureAtlasSprite, &mut Transform), With<Player>>, keyboard_input: Res<Input<KeyCode>>) {
    let (x_dir, y_dir, up) = if keyboard_input.pressed(KeyCode::A) {
        (-1., 0., false)
    }
    else if keyboard_input.pressed(KeyCode::D) {
        (1., 0., false)
    }
    else if keyboard_input.pressed(KeyCode::S) {
        (0., -1., false)
    }
    else if keyboard_input.pressed(KeyCode::W) {
        (0., 1., true)
    }
    else {
        (0., 0., false)
    };
    let (mut sprite, mut transform) = query.single_mut();
    let translation = transform.translation;
    *transform = Transform::from_translation(Vec3::new(translation.x + x_dir, translation.y + y_dir, 0.));
    if up {
        sprite.index = 1;
    }
    else {
        sprite.index = 0;
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: ResMut<Windows>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    commands.spawn(Camera2dBundle::default());

    let texture = asset_server.load("fence.png");

    commands.spawn(SpriteBundle {
        texture,
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..Default::default()
    });

    let window = windows.get_primary_mut().unwrap();
    window.set_title("Rusty Farm".to_string());

    let window_width = window.width();
    let window_height = window.height();

    let tomato_texture = asset_server.load("tomato_1.png");
    
    for row in 0..9 {
        for col in 0..9 {
            let x = EDGE_BUFFER + -(window_width / 2.) + (col as f32 * PLANT_SPACING) as f32;
            let y = (window_height / 2.) - (row as f32 * PLANT_SPACING) as f32 - EDGE_BUFFER;

            println!("x: {} y: {}", x, y);

            commands.spawn(SpriteBundle {
                texture: tomato_texture.clone(),
                transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                ..Default::default()
            });
        }
    }

    let farmer = asset_server.load("farmer/farmer.png");
    let texture_atlas = TextureAtlas::from_grid(farmer, Vec2::new(20., 24.), 4, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..Default::default()
        },
        Player
    ));

}