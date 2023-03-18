use bevy::prelude::*;
use crate::{bevy_animations::*, player::*, gate::*};

// gate frames
pub const GATE_OPENING_FRAMES: [usize; 3] = [0, 1, 2];
pub const GATE_CLOSING_FRAMES: [usize; 3] = [2, 1, 0];

// player frames
pub const WALKING_ANIMATION_FRAMES: [usize; 6] = [0, 1, 2, 3, 4, 5];
pub const RUNNUNG_ANIMATION_FRAMES: [usize; 6] = [0, 1, 6, 3, 4, 7];
pub const HARVESTING_ANIMATION_FRAMES: [usize; 4] = [0, 1, 2, 3];

pub fn set_animations(
    player_query: Query<(
        Entity,
        &Player       
    )>,
    gate_query: Query<(
        Entity,
        &Handle<TextureAtlas>,
        &Gate
    )>,
    mut animations: ResMut<Animations>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>
) {
    //  the player entity might not be loaded in yet
    let player_entity = match player_query.get_single() {
        Ok(entity) => entity.0,
        Err(_) => return
    };

    if !animations.is_inserted(&player_entity) {
        let farming = asset_server.load("farmer/farming_animation.png");
        let farmer = asset_server.load("farmer/char_a_p1_0bas_humn_v00.png");    
        
        let farmer_atlas = TextureAtlas::from_grid(farmer, Vec2::new(64., 64.), 8, 8, None, None);
        let farming_atlas = TextureAtlas::from_grid(
            farming,
            Vec2::new(64., 64.),
            4,
            4,
            None,
            None
        );
        
        let farmer_handle = texture_atlases.add(farmer_atlas);
        let farming_handle = texture_atlases.add(farming_atlas);
        
        animations
        .insert_animation(player_entity, AnimationType::Transform(
            TransformAnimation::new(
                Vec::from(WALKING_ANIMATION_FRAMES), 
                0.45, 
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
                0.35, 
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
                vec![0.001, 0.300, 0.350, 0.375], 
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
    for query in gate_query.iter() {
        let (gate_entity, handle, _) = query;
        if !animations.is_inserted(&gate_entity) {
            animations.insert_animation(
                gate_entity, 
                AnimationType::LinearTimed(
                    LinearTimedAnimation::new(
                        Vec::from(GATE_OPENING_FRAMES), 
                            vec![0.01, 0.1, 0.1], 
                            texture_atlases.get_handle(handle), 
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
                            vec![0.01, 0.1, 0.1], 
                            texture_atlases.get_handle(handle), 
                            false
                        ), 
                        "gate_closing"
                    )
                );
            }
        }
    }
}