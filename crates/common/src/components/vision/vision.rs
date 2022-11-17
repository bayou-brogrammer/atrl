use crate::prelude::*;

#[derive(Reflect, Component, Debug, Default, Deref, DerefMut)]
#[reflect(Component)]
pub struct Vision(pub u8); // This must match with vision_type
