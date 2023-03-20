use std::collections::HashMap;

use bevy::{prelude::*};

use crate::{OtherAssets, GameState};

#[derive(Resource)]
pub struct Atlases {
    pub handles: HashMap<&'static str, Handle<TextureAtlas>>
}

/// system that loads texture atlases and inserts them into the `Atlases` resources so the are easily accessable anywhere
pub fn load_altases(
    mut commands: Commands,
    textures: Res<OtherAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut state: ResMut<State<GameState>>
) {
    let crop_growth_highlighted_atlas = TextureAtlas::from_grid(textures.corn_growth_highlighted.clone(), Vec2::new(16., 29.), 5, 1, None, None);
    let crop_growth_atlas = TextureAtlas::from_grid(textures.corn_growth.clone(), Vec2::new(16., 29.), 5, 1, None, None);
    let dead_crop_atlas = TextureAtlas::from_grid(textures.dead_crop.clone(), Vec2::new(16., 29.), 3, 1, None, None);
    let dead_crop_highlighted_atlas = TextureAtlas::from_grid(textures.dead_crop_highlighted.clone(), Vec2::new(16., 29.), 3, 1, None, None);
    let gate_atlas = TextureAtlas::from_grid(textures.gate.clone(), Vec2::new(32., 50.), 3, 1, None, Some(Vec2::new(16., 16.)));
    let player_atlas = TextureAtlas::from_grid(textures.player.clone(), Vec2::new(64., 64.), 8, 8, None, None);
    let player_farming_atlas = TextureAtlas::from_grid(textures.player_farming.clone(), Vec2::new(64., 64.), 4, 4,None, None);
    
    let crop_growth_highlighted_handle = texture_atlases.add(crop_growth_highlighted_atlas);
    let crop_growth_handle = texture_atlases.add(crop_growth_atlas);
    let dead_crop_handle = texture_atlases.add(dead_crop_atlas);
    let dead_crop_highlighted_handle = texture_atlases.add(dead_crop_highlighted_atlas);
    let player_handle = texture_atlases.add(player_atlas); 
    let player_farming_handle = texture_atlases.add(player_farming_atlas);
    let gate_handle = texture_atlases.add(gate_atlas);

    let mut map = HashMap::new();
    map.insert("corn_growth_highlighted", crop_growth_highlighted_handle);
    map.insert("corn_growth", crop_growth_handle);
    map.insert("dead_crop", dead_crop_handle);
    map.insert("dead_crop_highlighted", dead_crop_highlighted_handle);
    map.insert("player", player_handle);
    map.insert("player_farming", player_farming_handle);
    map.insert("gate", gate_handle);

    commands.insert_resource(Atlases {
        handles: map
    });

    state.overwrite_set(GameState::LoadinMainMenu).unwrap();
}