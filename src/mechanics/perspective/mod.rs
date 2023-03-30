use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// **INFO** mechanics plugin for 2d that will change the secondary bodies z position based on where the primary body is to the secondary body
/// 
/// This is useful for determining if the player should for example be positioned behind some tall grass when they walk behind it, and be positioned
/// in front when they walk in front of it 
/// 
/// **NOTE** For this mechanic to work properly you need to attach the `PrimaryPerspectiveBody` or `SecondaryPerspectiveBody` to the same entity as the sprite
/// 
/// # Example
/// ```rust
/// commands.spawn(SpriteBundle {
///     /* your sprite */
/// })
///     .insert(PrimaryPerspectiveBody)
/// ;
/// ```
/// **NOTE** if the primary perspective body isn't attached to the same entity that has the sprite the translation will change the incorrect entity and you won't
/// get the proper functionality
/// 
/// **NOTE** This mechanic uses bevy_rapier2d to determine whether or not to maipulate the translation. Knowing this you can edit the way the perspective changes via 
/// tweaking the collider size
pub struct PerspectiveMechanicsPlugin;

impl Plugin for PerspectiveMechanicsPlugin {
    fn build(&self, app: &mut App) {
        app.
            add_system(change_perpective)
        ;
    }
}

/// The primary body out of a pair
/// 
/// Should mainly be used for bodies that move a lot like players or NPCs
#[derive(Component, Debug, Default, Clone)]
pub struct PrimaryPerspectiveBody;

/// The secondary body out of a par
/// 
/// Should mainly be used for bodies that don't move like walls or trees
#[derive(Component, Debug, Default, Clone)]
pub struct SecondaryPerspectiveBody;

pub fn change_perpective(
    primary_query: Query<(Entity, &Transform), (With<PrimaryPerspectiveBody>, Without<SecondaryPerspectiveBody>)>,
    mut secondary_query: Query<(Entity, &mut Transform), (With<SecondaryPerspectiveBody>, Without<PrimaryPerspectiveBody>)>,
    rapier_context: Res<RapierContext>
) {
    for (second_entity, mut second_transform) in secondary_query.iter_mut() {
        for (primary_entity, primary_transform) in primary_query.iter() {
            if let Some(collision) = rapier_context.intersection_pair(second_entity, primary_entity) {
                if !collision {
                    continue;
                }
                if second_transform.translation.y > primary_transform.translation.y {
                    second_transform.translation.z = primary_transform.translation.z - 0.5;
                }
                else {
                    second_transform.translation.z = primary_transform.translation.z + 0.5;
                }
            }
        }   
    }
}