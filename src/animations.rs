use bevy::prelude::*;
use crate::{bevy_animations::*, player::*, gate::*, OtherAssets, GameState};

// gate configs
pub const GATE_OPENING_FRAMES: [usize; 3] = [0, 1, 2];
pub const GATE_CLOSING_FRAMES: [usize; 3] = [2, 1, 0];
pub const GATE_FRAME_TIMINGS: [f32; 3] = [0.001, 0.1, 0.1];

// player configs
pub const WALKING_ANIMATION_FRAMES: [usize; 6] = [0, 1, 2, 3, 4, 5];
pub const RUNNUNG_ANIMATION_FRAMES: [usize; 6] = [0, 1, 6, 3, 4, 7];
pub const HARVESTING_ANIMATION_FRAMES: [usize; 4] = [0, 1, 2, 3];
pub const PLAYER_HARVESTING_TIMINGS: [f32; 4] = [0.001, 0.300, 0.350, 0.375];

pub fn set_animations(
    mut player_query: Query<(
        Entity,
        &mut Handle<TextureAtlas>,
        &Player       
    ), Without<Gate>>,
    mut gate_query: Query<(
        Entity,
        &mut Handle<TextureAtlas>,
        &Gate
    ), Without<Player>>,
    mut animations: ResMut<Animations>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    textures: Res<OtherAssets>,
    mut state: ResMut<State<GameState>>
) {
    //  the player entity might not be loaded in yet
    let (player_entity, mut player_texture, _) = player_query.single_mut();

    
    let gate = textures.gate.clone();
    let farming = textures.player_farming.clone();
    let farmer = textures.player.clone();    
    
    let gate_atlas = TextureAtlas::from_grid(gate, Vec2::new(32., 50.), 3, 1, None, Some(Vec2::new(16., 16.)));
    let farmer_atlas = TextureAtlas::from_grid(farmer, Vec2::new(64., 64.), 8, 8, None, None);
    let farming_atlas = TextureAtlas::from_grid(farming, Vec2::new(64., 64.), 4, 4,None, None);
    
    let farmer_handle = texture_atlases.add(farmer_atlas);
    let farming_handle = texture_atlases.add(farming_atlas);
    let gate_handle = texture_atlases.add(gate_atlas);

    *player_texture = farmer_handle.clone();
    
    animations
        .insert_animation(player_entity, AnimationType::Transform(
            TransformAnimation::new(
                Vec::from(WALKING_ANIMATION_FRAMES), 
                0.65, 
                farmer_handle.clone(), 
                Vec2::new(8., 8.), 
                AnimationDirectionIndexes::new(8, 7, 6, 5), 
                true
            ), 
            "player_walking"
            )
        )
        .insert_animation(player_entity, AnimationType::Transform(
            TransformAnimation::new(
                Vec::from(RUNNUNG_ANIMATION_FRAMES), 
                0.75, 
                farmer_handle, 
                Vec2::new(8., 8.), 
                AnimationDirectionIndexes::new(8, 7, 6, 5), 
                true
            ), 
            "player_running"
            )
        )
        .insert_animation(player_entity, AnimationType::Timed(
            TimedAnimation::new(
                Vec::from(HARVESTING_ANIMATION_FRAMES), 
                Vec::from(PLAYER_HARVESTING_TIMINGS), 
                farming_handle, 
                Vec2::new(4., 4.), 
                AnimationDirectionIndexes::new(4, 3, 2, 1), 
                false, 
                true, 
                1
            ), 
            "player_harvesting"
            )
        )
    ;
    for query in gate_query.iter_mut() {
        let (gate_entity, mut texture, _) = query;
        *texture = gate_handle.clone();
        if !animations.is_inserted(&gate_entity) {
            animations.insert_animation(
            gate_entity, 
            AnimationType::LinearTimed(
                LinearTimedAnimation::new(
                    Vec::from(GATE_OPENING_FRAMES), 
                        Vec::from(GATE_FRAME_TIMINGS), 
                        texture_atlases.get_handle(gate_handle.clone()), 
                        false
                    ), 
                    "gate_opening"
                )
            );
            animations.insert_animation(
                gate_entity, 
                AnimationType::LinearTimed(
                    LinearTimedAnimation::new(
                        Vec::from(GATE_CLOSING_FRAMES), 
                        Vec::from(GATE_FRAME_TIMINGS), 
                        texture_atlases.get_handle(gate_handle.clone()), 
                        false
                    ), 
                    "gate_closing"
                )
            );
        }
    }
    match state.overwrite_replace(GameState::Loaded) {
        Ok(_) => {},
        Err(_) => {}
    }
}