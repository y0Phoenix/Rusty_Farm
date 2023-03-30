use std::{fs::{File, read_to_string, OpenOptions}, io::Write};

use bevy::{prelude::{Component, Vec3, Plugin, App, Query, With, Transform, ResMut, State, SystemSet, Resource, Commands, Res, BuildChildren}, time::Timer, sprite::SpriteSheetBundle, log};
use bevy_rapier2d::prelude::*;
use serde::*;
use time::{OffsetDateTime};

use crate::{GameState, player::Player, crop::{Crop, CropBundle, SmallCropColliderBundle}, NextState, load_atlases::Atlases, ldtk::SensorBundle, bevy_animations::AnimationTimer, mechanics::perspective::SecondaryPerspectiveBody};

#[derive(Component, Default, Debug, Clone)]
pub struct Savable;

#[derive(Debug, Default, Resource)]
pub enum SaveName {
    #[default]
    New,
    Load(String)
}

impl SaveName {
    pub fn name(&self) -> Option<String> {
        match self {
            SaveName::Load(name) => Some(name.clone()),
            _ => None
        }
    }

    pub fn path(&self) -> Option<String> {
        match self {
            SaveName::Load(name) => Some(format!("saves/{}", name.clone())),
            _ => None
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveGameMetaData {
    pub date: String,
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveGames(pub Vec<SaveGameMetaData>);

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerData {
    translation: Vec3,
    player: Player
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CropData {
    translation: Vec3,
    crop: Crop
}

#[derive(Debug, Serialize, Deserialize, Resource)]
pub struct SaveData {
    date: String,
    player_data: PlayerData,
    crop_data: Vec<CropData>
}

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SaveName::default())
            .add_system_set(SystemSet::on_enter(GameState::Saving)
                .with_system(save_game)
            )
            .add_system_set(SystemSet::on_enter(GameState::LoadingSave)
                .with_system(load_save)
            )
        ;
    }
}

pub fn save_game(
    player_query: Query<(&Transform, &Player), With<Savable>>,
    crops_query: Query<(&Transform, &Crop), With<Savable>>,
    mut app_state: ResMut<State<GameState>>,
    mut next_state: ResMut<NextState>,
    mut save_name: ResMut<SaveName>
) {
    let (player_transform, player) = player_query.single();

    let mut crop_data: Vec<CropData> = Vec::new();

    for (crop_transform, crop) in crops_query.iter() {
        crop_data.push(CropData { 
            translation: crop_transform.translation, 
            crop: crop.clone() 
        });
    }

    log::info!("saving {} crops", crop_data.len());

    let sys_time = OffsetDateTime::now_local().unwrap();

    let formatted_time = format!("{} {} {} at {}:{}", sys_time.month(), sys_time.day(), sys_time.year(), sys_time.hour(), sys_time.minute());

    let rson_data = ron::ser::to_string_pretty(&SaveData {
        date: formatted_time.clone(),
        player_data: PlayerData { 
            translation: player_transform.translation, 
            player: player.clone() 
        },
        crop_data
    }, ron::ser::PrettyConfig::default()).unwrap();

    let save_games_data = read_to_string("saves/save_games.rson").unwrap();
    let mut save_games_file = File::create("saves/save_games.rson").unwrap();
    let mut save_games_data = ron::from_str::<SaveGames>(&save_games_data.as_str()).unwrap();
    
    // TODO implement an error screen UI that can be used to display an error when the user tries to save more than five games
    let save_game_path = match save_name.name() {
        Some(_) => Some(save_name.path().unwrap()),
        None => {
            if save_games_data.0.len() < 6 {
                let new_name = format!("save{}.rson", save_games_data.0.len() + 1);
                save_games_data.0.push(SaveGameMetaData { 
                    date: formatted_time.clone(), 
                    name: new_name.clone() 
                });
                *save_name = SaveName::Load(new_name.clone());
                Some(format!("saves/{}", new_name))
            }
            else {
                None
            }
        }
    }.expect("Max Save Game Count Reached");
    log::info!("attempting to open save file {}", save_game_path);
    let mut save_file = File::create(save_game_path).unwrap();
    save_file.write_all(rson_data.as_bytes()).unwrap();
    
    for save_game in save_games_data.0.iter_mut() {
        if save_game.name == save_name.name().unwrap() {
            save_game.date = formatted_time;
            break;
        }
    }
    save_games_file.write_all(ron::ser::to_string_pretty(&save_games_data, ron::ser::PrettyConfig::default()).unwrap().as_bytes()).unwrap();

    app_state.overwrite_set(next_state.0.clone()).unwrap();
    next_state.0 = GameState::default();
}

fn load_save(
    mut commands: Commands,
    mut player_query: Query<(&mut Transform, &mut Player)>,
    save_name: Res<SaveName>,
    mut game_state: ResMut<State<GameState>>,
    atlases: Res<Atlases>
) {
    let (mut player_transform, mut player) = player_query.single_mut();

    let save_data = read_to_string(save_name.path().unwrap()).unwrap();

    let save_data = ron::from_str::<SaveData>(&save_data.as_str()).unwrap();

    *player_transform = Transform::from_xyz(save_data.player_data.translation.x, save_data.player_data.translation.y, save_data.player_data.translation.z + 50.);
    *player = save_data.player_data.player;

    log::info!("setting player position and data from save");

    for crop_data in save_data.crop_data {
        commands.spawn(CropBundle {
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: atlases.handles.get(crop_data.crop.crop_type.atlas_name()).unwrap().clone(),
                transform: Transform::from_translation(crop_data.translation),
                ..Default::default()
            },
            sensor_bundle: SensorBundle {
                collider: Collider::cuboid(8., 3.),
                sensor: Sensor,
                ..Default::default()
            },
            animation_timer: AnimationTimer(Timer::from_seconds(crop_data.crop.crop_type.duration(), bevy::time::TimerMode::Repeating)),
            rigid_body: RigidBody::KinematicPositionBased,
            crop: crop_data.crop,
            ..Default::default()
        })
            .with_children(|parent| {
                parent.spawn(SmallCropColliderBundle::default());
            })
        ;
    }
    game_state.overwrite_set(GameState::LoadingAnimations).unwrap();
}