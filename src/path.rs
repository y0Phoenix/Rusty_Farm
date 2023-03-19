use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::{ldtk::*, };

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Path;

#[derive(Clone, Debug, Default, Bundle)]
pub struct PathBundle {
    path: Path,
    #[bundle]
    pub sensor_bundle: SensorBundle,
    pub ldtk: Ldtk
}
impl LdtkIntCell for PathBundle {
    fn bundle_int_cell(_: IntGridCell, _: &LayerInstance) -> Self {
        Self {
            path: Path,
            sensor_bundle: SensorBundle { 
                collider: Collider::cuboid(12., 12.), 
                sensor: Sensor, 
                ..Default::default()
            },
            ldtk: Ldtk
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
    pub ldtk: Ldtk
}

impl LdtkIntCell for SmallPathBundle {
    fn bundle_int_cell(_: IntGridCell, _: &LayerInstance) -> Self {
        Self {
            path: SmallPath,
            sensor_bundle: SensorBundle { 
                collider: Collider::cuboid(12., 12.), 
                sensor: Sensor, 
                ..Default::default()
            },
            ldtk: Ldtk
        }
    }
}
