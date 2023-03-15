use std::collections::HashMap;

use bevy::{prelude::*, utils::HashSet};
// use bevy_animations::*;
use crate::animations::*;
use bevy_ecs_ldtk::{prelude::*, ldtk::{LayerDefinition, Level}};

use bevy_rapier2d::prelude::*;

use crate::player::{WALKING_ANIMATION_FRAMES, RUNNUNG_ANIMATION_FRAMES, HARVESTING_ANIMATION_FRAMES};

pub const GATE_OPENING_FRAMES: [usize; 3] = [0, 1, 2];
pub const GATE_CLOSING_FRAMES: [usize; 3] = [2, 1, 0];
pub struct FarmWorldPlugin;

impl Plugin for FarmWorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_entity::<LdtkPlayer>("Player")
            .register_ldtk_entity::<LdtkGate>("Gate")
            .register_ldtk_int_cell::<FenceBundle>(1)
            .register_ldtk_int_cell_for_layer::<PathBundle>("Paths", 1)
            .register_ldtk_int_cell_for_layer::<PathBundle>("Paths", 2)
            .add_startup_system(spawn_world)
            .insert_resource(LevelSelection::Identifier("Main_Farm".to_string()))
            .insert_resource(CurrentLevel::default())
            .add_system(movement)
            .add_system(set_animations)
            .add_system(check_gate_collisions)
            .add_system(center_camera_around_player)
        ;
    }
}

#[derive(Debug, Resource, Clone, Default)]
pub struct CurrentLevel(Level);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
struct Player;

#[derive(Clone, Default, Bundle)]
struct LdtkPlayer {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub player: Player,
    #[bundle]
    pub collider_bundle: ColliderBundle,
    entity_instance: EntityInstance,
    pub direction: AnimationDirection
}

impl LdtkEntity for LdtkPlayer {
    fn bundle_entity(
            entity_instance: &EntityInstance,
            layer_instance: &LayerInstance,
            tileset: Option<&Handle<Image>>,
            tileset_definition: Option<&TilesetDefinition>,
            asset_server: &AssetServer,
            texture_atlases: &mut Assets<TextureAtlas>,
        ) -> LdtkPlayer {
        let texture = asset_server.load("farmer/char_a_p1_0bas_humn_v00.png");

        let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(64., 64.), 8, 8, None, None);

        let handle = texture_atlases.add(texture_atlas);

        let bundle = SpriteSheetBundle {
            texture_atlas: handle,
            ..Default::default()
        };

        let fields = &entity_instance.field_instances;

        let mut linvel = 0.;

        for field in fields {
            if field.identifier == "velocity" {
                if let FieldValue::Float(Some(vel)) = field.value {
                    linvel = vel;
                }
            }
        }
        LdtkPlayer { 
            sprite_sheet_bundle: bundle,
            player: Player,
            collider_bundle: ColliderBundle { 
                collider: Collider::cuboid(8., 18.), 
                rigid_body: RigidBody::Dynamic, 
                velocity: Velocity { linvel: Vec2::new(linvel, linvel), angvel: 0. }, 
                rotation_constraints: LockedAxes::ROTATION_LOCKED,
                damping: Damping {
                    linear_damping: 100.,
                    angular_damping: 100.
                },
                ..Default::default()
            } ,
            entity_instance: entity_instance.clone(),
            direction: AnimationDirection::default()
        }
    }
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub damping: Damping,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderMassProperties,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct SensorBundle {
    pub collider: Collider,
    pub sensor: Sensor,
    pub active_events: ActiveEvents,
    pub rotation_constraints: LockedAxes,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Fence;

#[derive(Clone, Debug, Default, Bundle)]
pub struct FenceBundle {
    fence: Fence,
    #[bundle]
    pub collider_bundle: ColliderBundle,
}

impl LdtkIntCell for FenceBundle {
    fn bundle_int_cell(int_grid_cell: IntGridCell, layer_instance: &LayerInstance) -> Self {
        Self {
            fence: Fence, 
            collider_bundle: ColliderBundle { 
                collider: Collider::cuboid(8., 4.), 
                rigid_body: RigidBody::Fixed,
                ..Default::default()
            } 
        }
    }
}

#[derive(Debug, Component, Clone, Default)]
pub struct Gate {
    in_collision: bool,
    open: bool
}

#[derive(Clone, Default, Bundle)]
pub struct LdtkGate {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub gate: Gate,
    #[bundle]
    pub sensor_bundle: SensorBundle,
    entity_instance: EntityInstance,
    pub direction: AnimationDirection
}

impl LdtkEntity for LdtkGate {
    fn bundle_entity(
            entity_instance: &EntityInstance,
            layer_instance: &LayerInstance,
            tileset: Option<&Handle<Image>>,
            tileset_definition: Option<&TilesetDefinition>,
            asset_server: &AssetServer,
            texture_atlases: &mut Assets<TextureAtlas>,
        ) -> Self {
        let texture = asset_server.load("buildings/fence_gate.png");

        let texture_atlas = TextureAtlas::from_grid(
            texture,
            Vec2::new(32., 16.),
            3,
            1,
            None,
            Some(Vec2::new(16., 16.))
        );

        let handle = texture_atlases.add(texture_atlas);

        Self { 
            gate: Gate::default(), 
            sensor_bundle: SensorBundle { 
                collider: Collider::cuboid(16., 16.), 
                sensor: Sensor,
                ..Default::default()
            },
            sprite_sheet_bundle: SpriteSheetBundle { 
                texture_atlas: handle,
                ..Default::default()
            },
            entity_instance: entity_instance.clone(),
            direction: AnimationDirection::default() 
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Path;

#[derive(Clone, Debug, Default, Bundle)]
pub struct PathBundle {
    path: Path,
    #[bundle]
    pub sensor_bundle: SensorBundle,
}
impl LdtkIntCell for PathBundle {
    fn bundle_int_cell(int_grid_cell: IntGridCell, layer_instance: &LayerInstance) -> Self {
        Self {
            path: Path,
            sensor_bundle: SensorBundle { 
                collider: Collider::cuboid(12., 12.), 
                sensor: Sensor, 
                ..Default::default()
            }
        }
    }
}
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct SmallPath;

#[derive(Clone, Debug, Default, Bundle)]
pub struct SmallPathBundle {
    path: SmallPath,
    #[bundle]
    pub sensor_bundle: SensorBundle,
}

impl LdtkIntCell for SmallPathBundle {
    fn bundle_int_cell(int_grid_cell: IntGridCell, layer_instance: &LayerInstance) -> Self {
        Self {
            path: SmallPath,
            sensor_bundle: SensorBundle { 
                collider: Collider::cuboid(12., 12.), 
                sensor: Sensor, 
                ..Default::default()
            }
        }
    }
}

fn spawn_world(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>
) {
    let camera = Camera2dBundle::default();

    commands.spawn(camera);

    let ldtk_handle = asset_server.load("Rusty_Farm_World.ldtk");

    let ldtk_world = LdtkWorldBundle {
            ldtk_handle,
            ..Default::default()
    };

    commands.spawn(ldtk_world);
    
    let window = windows.get_primary_mut().unwrap();
    window.set_title("Rusty Farm".to_string());
}

fn set_animations(
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
                0.25, 
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
    for (i, query) in gate_query.iter().enumerate() {
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

fn check_gate_collisions(
    mut gate_query: Query<(
        Entity,
        &mut Gate
    )>,
    player_query: Query<(
        Entity,  
        &Player    
    )>,
    context: Res<RapierContext>,
    animations: Res<Animations>,
    mut animation_event: EventWriter<AnimationEvent>
) {
    // the player entity might not be loaded in yet
    let player_entity = match player_query.get_single() {
        Ok(entity) => entity.0,
        Err(_) => return
    };

    for gate_entity in gate_query.iter_mut() {
        let (gate_entity, mut gate) = gate_entity;

        if let Some(_) = context.intersection_pair(player_entity, gate_entity) {
            if let Some(in_animation) = animations.in_animation(gate_entity) {
                if !gate.open && !gate.in_collision && !in_animation {
                    animation_event.send(AnimationEvent("gate_opening", gate_entity));
                    gate.in_collision = true;
                    gate.open = true;
                }
            }
        }
        else if gate.in_collision && gate.open {
            animation_event.send(AnimationEvent("gate_closing", gate_entity));
            gate.in_collision = false;
            gate.open = false;
        }
    }
}

fn movement(
    input: Res<Input<KeyCode>>,
    mut player_query: Query<(
        Entity,
        &mut Transform,
        &mut Velocity,
        &mut AnimationDirection,
        &Player
    )>,
    level_query: Query<(&Handle<LdtkLevel>, &Transform), (Without<Player>, Without<Camera2d>)>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
    mut animation_event_writer: EventWriter<AnimationEvent>,
    animations: ResMut<Animations>,
    time: Res<Time>
) {
    let (player_entity, mut transform, mut vel, mut direction, _) = match player_query.get_single_mut(){
        Ok(p) => p,
        Err(_) => return
    };

    // if we haven't initialized our animations yet
    if !animations.is_inserted(&player_entity) {
        return;
    }

    let inputs = input.get_pressed();

    let mut dir = AnimationDirection::default();

    let mut running = false;

    for key in inputs {
        match *key {
            KeyCode::A => dir = AnimationDirection::Left,
            KeyCode::S => dir = AnimationDirection::Down,
            KeyCode::D => dir = AnimationDirection::Right,
            KeyCode::W => dir = AnimationDirection::Up,
            KeyCode::LShift => {
                running = true;
                vel.linvel.x = 3.15;
                vel.linvel.y = 3.15;
            }
            _ => {}
        }
    }

    *direction = dir;

    if *direction != AnimationDirection::default() {
        if !running {
            vel.linvel.x = 2.65;
            vel.linvel.y = 2.65;
            animation_event_writer.send(AnimationEvent("player_walking", player_entity));
        } 
        else {
            animation_event_writer.send(AnimationEvent("player_running", player_entity));
        }

        let translation = transform.translation;

        for (level_handle, level_transform) in level_query.iter() {
            let level = ldtk_levels.get(level_handle).unwrap();
            let level_bounds = Rect {
                min: Vec2::new(level_transform.translation.x, level_transform.translation.y),
                max: Vec2::new(
                    level_transform.translation.x + level.level.px_wid as f32,
                    level_transform.translation.y + level.level.px_hei as f32,
                ),
            };
            let new_transform = Transform::from_translation(
                Vec3::new(
                    translation.x + AnimationDirection::get_direction(&direction).x * vel.linvel.x, 
                    translation.y + AnimationDirection::get_direction(&direction).y * vel.linvel.y, 
                    translation.z
                )
            );
            if new_transform.translation.x < level_bounds.max.x &&
                new_transform.translation.x > level_bounds.min.x &&
                new_transform.translation.y < level_bounds.max.y &&
                new_transform.translation.y > level_bounds.min.y 
            {
                *transform = new_transform;
            }
        }

    }
}

fn center_camera_around_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>, Without<Handle<LdtkLevel>>)>,
    windows: Res<Windows>,
    level_query: Query<(&Handle<LdtkLevel>, &Transform), (Without<Player>, Without<Camera2d>)>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let mut camera_transform = camera_query.single_mut();
        for (level_handle, level_transform) in &level_query {
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                if let Some(window) = windows.get_primary() {
                    let window_width = window.width() / 2.0;
                    let window_height = window.height() / 2.0;

                    let level_height = level_transform.translation.y + ldtk_level.level.px_hei as f32;
                    let level_width = level_transform.translation.x + ldtk_level.level.px_wid as f32;
                    
                    let new_camera_x = player_transform.translation.x;
                    let new_camera_y = player_transform.translation.y;
                    
                    // Clamp the camera's position within the map boundaries
                     // Clamp the camera's position within the map boundaries
                    camera_transform.translation.x = new_camera_x
                        .max(level_transform.translation.x + window_width)
                        .min(level_transform.translation.x + level_width - window_width);
                    camera_transform.translation.y = new_camera_y
                        .max(level_transform.translation.y + window_height)
                        .min(level_transform.translation.y + level_height - window_height);
                }
            }
        }
    }
}