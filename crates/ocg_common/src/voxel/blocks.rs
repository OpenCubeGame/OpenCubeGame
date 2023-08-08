//! The builtin block types.
//! Most of this will be moved to a "base" mod at some point in the future.

use ocg_schemas::dependencies::rgb::RGBA8;
use ocg_schemas::registry::RegistryName;
use ocg_schemas::voxel::voxeltypes::BlockShapeSet::StandardShapedMaterial;
use ocg_schemas::voxel::voxeltypes::{BlockDefinition, BlockRegistry, EMPTY_BLOCK};

/// Installs the base set of blocks into the given block registry.
pub fn setup_basic_blocks(registry: &mut BlockRegistry) {
    registry.push_object(EMPTY_BLOCK.clone()).unwrap();
    registry
        .push_object(BlockDefinition {
            name: RegistryName::ocg_const("stone"),
            shape_set: StandardShapedMaterial,
            representative_color: RGBA8::new(128, 128, 128, 255),
            has_collision_box: true,
            has_drawable_mesh: true,
        })
        .unwrap();
    registry
        .push_object(BlockDefinition {
            name: RegistryName::ocg_const("dirt"),
            shape_set: StandardShapedMaterial,
            representative_color: RGBA8::new(110, 81, 0, 255),
            has_collision_box: true,
            has_drawable_mesh: true,
        })
        .unwrap();
    registry
        .push_object(BlockDefinition {
            name: RegistryName::ocg_const("grass"),
            shape_set: StandardShapedMaterial,
            representative_color: RGBA8::new(30, 230, 30, 255),
            has_collision_box: true,
            has_drawable_mesh: true,
        })
        .unwrap();
}