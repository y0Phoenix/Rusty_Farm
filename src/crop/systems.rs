use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::{bevy_animations::*, ldtk::*};

use super::CropField;

pub fn spawn_crops(
    crop_field_query: Query<(&CropField, &Transform)>
) {

}