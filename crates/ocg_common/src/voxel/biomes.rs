//! The builtin biome types.
//! Most of this will be moved to a "base" mod at some point in the future.

use noise::NoiseFn;
use ocg_schemas::{voxel::{biome::{BiomeRegistry, BiomeDefinition, PLAINS_BIOME_NAME, biome_map::GLOBAL_SCALE_MOD}, generation::Context, voxeltypes::{BlockRegistry, BlockEntry}}, registry::RegistryName, dependencies::rgb::RGBA8, range::range};

use super::blocks::{SNOWY_GRASS_BLOCK_NAME, DIRT_BLOCK_NAME, GRASS_BLOCK_NAME, STONE_BLOCK_NAME, WATER_BLOCK_NAME};


/// Registry name for ocean.
pub const OCEAN_BIOME_NAME: RegistryName = RegistryName::ocg_const("ocean");
/// Registry name for hills.
pub const HILLS_BIOME_NAME: RegistryName = RegistryName::ocg_const("hills");
/// Registry name for mountains.
pub const MOUNTAINS_BIOME_NAME: RegistryName = RegistryName::ocg_const("mountains");

pub fn setup_basic_biomes(biome_registry: &mut BiomeRegistry) {
    biome_registry
        .push_object(BiomeDefinition {
            name: PLAINS_BIOME_NAME,
            representative_color: RGBA8::new(20, 180, 10, 255),
            elevation: range(1.0..2.5),
            temperature: range(..),
            moisture: range(..2.5),
            rule_source: |pos: &bevy_math::IVec3, context: &Context, block_registry: &BlockRegistry| {
                let (i_grass, _) = block_registry.lookup_name_to_object(GRASS_BLOCK_NAME.as_ref()).unwrap();
                let (i_dirt, _) = block_registry.lookup_name_to_object(DIRT_BLOCK_NAME.as_ref()).unwrap();
                let (i_stone, _) = block_registry.lookup_name_to_object(STONE_BLOCK_NAME.as_ref()).unwrap();
                let (i_snow_grass, _) = block_registry.lookup_name_to_object(SNOWY_GRASS_BLOCK_NAME.as_ref()).unwrap();

                if context.ground_y == pos.y {
                    if pos.y >= 80 {
                        return Some(BlockEntry::new(i_snow_grass, 0));
                    } else {
                        return Some(BlockEntry::new(i_grass, 0));
                    }
                } else if pos.y <= context.ground_y && pos.y > context.ground_y - 5 {
                    return Some(BlockEntry::new(i_dirt, 0));
                } else if context.ground_y > pos.y {
                    return Some(BlockEntry::new(i_stone, 0));
                }
                return None;
            },
            surface_noise: |point: [f64; 2], seed: u32, noise: &mut Box<dyn NoiseFn<f64, 2>>| {
                let new_point = [point[0] / GLOBAL_SCALE_MOD * 2.0, point[1] / GLOBAL_SCALE_MOD * 2.0];

                let mut value = noise.get(new_point) * 0.75;
                value += noise.get([new_point[0] * 2.0, new_point[1] * 2.0]) * 0.25;
                value *= 5.0;
                return value;
            },
            blend_influence: 0.5,
            block_influence: 1.0,
            can_generate: true,
        })
        .unwrap();

    biome_registry
        .push_object(BiomeDefinition {
            name: HILLS_BIOME_NAME,
            representative_color: RGBA8::new(15, 110, 10, 255),
            elevation: range(2.5..3.5),
            temperature: range(..),
            moisture: range(..2.5),
            rule_source: |pos: &bevy_math::IVec3, context: &Context, block_registry: &BlockRegistry| {
                let (i_grass, _) = block_registry.lookup_name_to_object(GRASS_BLOCK_NAME.as_ref()).unwrap();
                let (i_dirt, _) = block_registry.lookup_name_to_object(DIRT_BLOCK_NAME.as_ref()).unwrap();
                let (i_stone, _) = block_registry.lookup_name_to_object(STONE_BLOCK_NAME.as_ref()).unwrap();
                let (i_snow_grass, _) = block_registry.lookup_name_to_object(SNOWY_GRASS_BLOCK_NAME.as_ref()).unwrap();

                if context.ground_y == pos.y {
                    if pos.y >= 80 {
                        return Some(BlockEntry::new(i_snow_grass, 0));
                    } else {
                        return Some(BlockEntry::new(i_grass, 0));
                    }
                } else if pos.y <= context.ground_y && pos.y > context.ground_y - 5 {
                    return Some(BlockEntry::new(i_dirt, 0));
                } else if context.ground_y > pos.y {
                    return Some(BlockEntry::new(i_stone, 0));
                }
                return None;
            },
            surface_noise: |point: [f64; 2], seed: u32, noise: &mut Box<dyn NoiseFn<f64, 2>>| {
                let new_point = [point[0] / GLOBAL_SCALE_MOD * 40.0, point[1] / GLOBAL_SCALE_MOD * 40.0];

                let mut value = noise.get(new_point) * 0.6;
                value += noise.get([new_point[0] * 1.5, new_point[1] * 1.5]) * 0.25;
                value += noise.get([new_point[0] * 3.0, new_point[1] * 3.0]) * 0.15;
                value *= 0.05;
                return value;
            },
            blend_influence: 1.0,
            block_influence: 1.0,
            can_generate: true,
        })
        .unwrap();

    biome_registry
        .push_object(BiomeDefinition {
            name: MOUNTAINS_BIOME_NAME,
            representative_color: RGBA8::new(220, 220, 220, 255),
            elevation: range(3.5..),
            temperature: range(../*=3.0*/),
            moisture: range(..2.5),
            rule_source: |pos: &bevy_math::IVec3, context: &Context, block_registry: &BlockRegistry| {
                let (i_grass, _) = block_registry.lookup_name_to_object(GRASS_BLOCK_NAME.as_ref()).unwrap();
                let (i_dirt, _) = block_registry.lookup_name_to_object(DIRT_BLOCK_NAME.as_ref()).unwrap();
                let (i_stone, _) = block_registry.lookup_name_to_object(STONE_BLOCK_NAME.as_ref()).unwrap();
                let (i_snow_grass, _) = block_registry.lookup_name_to_object(SNOWY_GRASS_BLOCK_NAME.as_ref()).unwrap();

                if context.ground_y == pos.y {
                    if pos.y >= 80 {
                        return Some(BlockEntry::new(i_snow_grass, 0));
                    } else {
                        return Some(BlockEntry::new(i_grass, 0));
                    }
                } else if pos.y <= context.ground_y && pos.y > context.ground_y - 5 {
                    return Some(BlockEntry::new(i_dirt, 0));
                } else if context.ground_y > pos.y {
                    return Some(BlockEntry::new(i_stone, 0));
                }
                return None;
            },
            surface_noise: |point: [f64; 2], seed: u32, noise: &mut Box<dyn NoiseFn<f64, 2>>| {
                let new_point = [point[0] / GLOBAL_SCALE_MOD * 16.0, point[1] / GLOBAL_SCALE_MOD * 16.0];
                let ridge = (0.5 - (0.5 - noise.get([new_point[0] * 5.0, new_point[1] * 5.0])).abs()) * 2.0;
                let ridge2 = (noise.get([new_point[0] * 2.0, new_point[1] * 2.0]) * 0.25) + (noise.get(new_point) * 0.5);

                let intermediate = ridge2 / 0.75;
                let mut value = intermediate * 0.15 * ridge;
                value += ridge2;
                value += intermediate + 0.05 * (0.5 - (0.5 - noise.get([new_point[0] * 9.0, new_point[1] * 9.0])).abs()) * 2.0;
                value *= 15.0;
                return value;
            },
            blend_influence: 1.0,
            block_influence: 1.0,
            can_generate: true,
        })
        .unwrap();

    biome_registry
        .push_object(BiomeDefinition {
            name: OCEAN_BIOME_NAME,
            representative_color: RGBA8::new(10, 120, 180, 255),
            elevation: range(..1.0),
            temperature: range(..),
            moisture: range(2.5..),
            rule_source: |pos: &bevy_math::IVec3, context: &Context, block_registry: &BlockRegistry| {
                let (i_stone, _) = block_registry.lookup_name_to_object(STONE_BLOCK_NAME.as_ref()).unwrap();
                let (i_water, _) = block_registry.lookup_name_to_object(WATER_BLOCK_NAME.as_ref()).unwrap();

                if context.sea_level > pos.y {
                    if context.ground_y > pos.y {
                        return Some(BlockEntry::new(i_stone, 0));
                    } else {
                        return Some(BlockEntry::new(i_water, 0));
                    }
                }
                return None;
            },
            surface_noise: |point: [f64; 2], seed: u32, noise: &mut Box<dyn NoiseFn<f64, 2>>| {
                let new_point = [point[0] / GLOBAL_SCALE_MOD * 1.0, point[1] / GLOBAL_SCALE_MOD * 1.0];

                let mut value = noise.get(new_point) * -7.5;
                value += 1.0;
                return value;
            },
            blend_influence: 1.0,
            block_influence: 1.0,
            can_generate: true,
        })
        .unwrap();
}