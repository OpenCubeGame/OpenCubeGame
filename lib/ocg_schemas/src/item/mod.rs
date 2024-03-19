//! All item related types.

use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

use crate::dependencies::serde::{Deserialize, Serialize};
use crate::registry::{Registry, RegistryId, RegistryName, RegistryNameRef, RegistryObject};

/// An Item type reference (id + metadata + count + others) stored in an inventory, used to uniquely identify a registered item variant.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[repr(C)]
pub struct ItemEntry<'a> {
    /// The item ID in the registry
    pub id: RegistryId,
    /// The count of this item.
    pub count: u32,
    /// Extra data for this item, can be used for various things, unpack with MessagePack.
    pub extra_data: &'a [u8],
}

/// A named registry of item definitions.
pub type ItemRegistry = Registry<ItemDefinition>;

impl<'a> ItemEntry<'a> {
    /// Helper to construct a new item entry.
    pub fn new(id: RegistryId, count: u32) -> Self {
        Self { id, count, extra_data: &[] }
    }

    /// Helper to construct a new item entry.
    pub fn new_data(id: RegistryId, count: u32, extra_data: &'a [u8]) -> Self {
        Self { id, count, extra_data }
    }

    /// Helper to look up the block definition corresponding to this ID
    pub fn lookup(self, registry: &ItemRegistry) -> Option<&ItemDefinition> {
        registry.lookup_id_to_object(self.id)
    }
}

impl Debug for ItemEntry<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ItemEntry{{id={}, count={}, extra_data=0x{:#?}}}", self.id, self.count, self.extra_data)
    }
}

/// A definition of an item type, specifying properties such as registry name, shape, textures.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemDefinition {
    /// The unique registry name
    pub name: RegistryName,
    /// The maximum stack size of this item.
    pub max_size: u32,
}

/// The registry name of [`EMPTY_ITEM`]
pub const EMPTY_ITEM_NAME: RegistryName = RegistryName::ocg_const("empty");

/// The empty item definition, used when no specific items have been generated
pub static EMPTY_ITEM: ItemDefinition = ItemDefinition {
    name: EMPTY_ITEM_NAME,
    max_size: 0,
};

impl RegistryObject for ItemDefinition {
    fn registry_name(&self) -> RegistryNameRef {
        self.name.as_ref()
    }
}

impl PartialEq for ItemDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Hash for ItemDefinition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl ItemDefinition {}