use std::fmt::Debug;

use bytemuck::{TransparentWrapper, Pod, Zeroable};
use serde::{Deserialize, Serialize};



/// A Entity identifier used to uniquely identify a registered entity variant.
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize, Zeroable)]
pub struct EntityId {
    registry_id: u32,
    collision_box: (i32, i32, i32),
}

impl Debug for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EntityId").field("registry_id", &self.registry_id).field("collision_box", &self.collision_box).finish()
    }
}