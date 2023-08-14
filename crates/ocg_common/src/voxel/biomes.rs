//! The builtin biome types.
//! Most of this will be moved to a "base" mod at some point in the future.

use noise::SuperSimplex;
use ocg_schemas::{voxel::{biome::{BiomeRegistry, BiomeDefinition, VPElevation, VPTemperature, VPMoisture, EMPTY_BIOME_NAME}, generation::{rule_sources::{EmptyRuleSource, ChainRuleSource, ConditionRuleSource, BlockRuleSource}, fbm_noise::Fbm, condition_sources::{YLevelCondition, OffsetGroundLevelCondition, ChainCondition, GroundLevelCondition}}, voxeltypes::{BlockRegistry, BlockEntry, EMPTY_BLOCK_NAME}}, registry::RegistryName, dependencies::rgb::RGBA8};

use super::blocks::{SNOWY_GRASS_BLOCK_NAME, DIRT_BLOCK_NAME, GRASS_BLOCK_NAME, STONE_BLOCK_NAME};


/// Registry name for plains.
pub const PLAINS_BIOME_NAME: RegistryName = RegistryName::ocg_const("plains");
/// Registry name for ocean.
pub const OCEAN_BIOME_NAME: RegistryName = RegistryName::ocg_const("ocean");
/// Registry name for hills.
pub const HILLS_BIOME_NAME: RegistryName = RegistryName::ocg_const("hills");
/// Registry name for mountains.
pub const MOUNTAINS_BIOME_NAME: RegistryName = RegistryName::ocg_const("mountains");

pub fn setup_basic_biomes(block_registry: &BlockRegistry, biome_registry: &mut BiomeRegistry) {
    let (i_air, _) = block_registry.lookup_name_to_object(EMPTY_BLOCK_NAME.as_ref()).unwrap();
    let (i_grass, _) = block_registry.lookup_name_to_object(GRASS_BLOCK_NAME.as_ref()).unwrap();
    let (i_dirt, _) = block_registry.lookup_name_to_object(DIRT_BLOCK_NAME.as_ref()).unwrap();
    let (i_stone, _) = block_registry.lookup_name_to_object(STONE_BLOCK_NAME.as_ref()).unwrap();
    let (i_snow_grass, _) = block_registry.lookup_name_to_object(SNOWY_GRASS_BLOCK_NAME.as_ref()).unwrap();

    let under_surface_5 = Box::new(OffsetGroundLevelCondition::new(-5));
    let on_surface = Box::new(GroundLevelCondition());

    let noise_func = Fbm::<SuperSimplex>::new(0);
    let noise_func = noise_func.set_octaves(vec![1.0, 1.0, 1.0, 1.0]);

    biome_registry.push_object(BiomeDefinition {
        name: EMPTY_BIOME_NAME,
        representative_color: RGBA8::new(0, 0, 0, 0),
        size_chunks: 0,
        elevation: VPElevation::LowLand,
        temperature: VPTemperature::MedTemp,
        moisture: VPMoisture::MedMoist,
        rule_source: Box::new(EmptyRuleSource()),
        surface_noise: Box::new(noise::Constant {value: 0.0}),
    }).unwrap();

    let plains_rule_source = Box::new(ChainRuleSource::new(vec![
        Box::new(ConditionRuleSource::new(on_surface, 
            Box::new(ChainRuleSource::new(vec![
                    Box::new(ConditionRuleSource::new(Box::new(YLevelCondition::new(80)), Box::new(BlockRuleSource::new(BlockEntry::new(i_snow_grass, 0))))),
                    Box::new(BlockRuleSource::new(BlockEntry::new(i_grass, 0))),
                ])
            ))
        ),
        Box::new(ConditionRuleSource::new(under_surface_5, Box::new(BlockRuleSource::new(BlockEntry::new(i_dirt, 0))))),
        Box::new(BlockRuleSource::new(BlockEntry::new(i_stone, 0)))
    ]));

    biome_registry
        .push_object(BiomeDefinition {
            name: PLAINS_BIOME_NAME,
            representative_color: RGBA8::new(20, 180, 10, 255),
            size_chunks: 4,
            elevation: VPElevation::LowLand,
            temperature: VPTemperature::MedTemp,
            moisture: VPMoisture::MedMoist,
            rule_source: plains_rule_source.clone(),
            surface_noise: Box::new(noise_func),
        })
        .unwrap();

    let noise_func = Fbm::<SuperSimplex>::new(0);
    let noise_func = noise_func.set_octaves(vec![0.1, 0.5, 0.0, 1.5]);
    biome_registry
        .push_object(BiomeDefinition {
            name: OCEAN_BIOME_NAME,
            representative_color: RGBA8::new(10, 120, 180, 255),
            size_chunks: 6,
            elevation: VPElevation::Ocean,
            temperature: VPTemperature::MedTemp,
            moisture: VPMoisture::HiMoist,
            rule_source: Box::new(BlockRuleSource::new(BlockEntry::new(i_stone, 0))),
            surface_noise: Box::new(noise_func),
        })
        .unwrap();

    let noise_func = Fbm::<SuperSimplex>::new(0);
    let noise_func = noise_func.set_octaves(vec![0.1, 0.5, 0.0, 1.5]);
    let noise_func = noise_func.set_persistence(0.75);
    biome_registry
        .push_object(BiomeDefinition {
            name: HILLS_BIOME_NAME,
            representative_color: RGBA8::new(15, 110, 10, 255),
            size_chunks: 3,
            elevation: VPElevation::Hill,
            temperature: VPTemperature::MedTemp,
            moisture: VPMoisture::MedMoist,
            rule_source: plains_rule_source.clone(),
            surface_noise: Box::new(noise_func),
        })
        .unwrap();

    let noise_func = Fbm::<SuperSimplex>::new(0);
    let noise_func = noise_func.set_octaves(vec![1.0, 1.5, 1.0, 1.5]);
    let noise_func = noise_func.set_persistence(1.0);
    biome_registry
        .push_object(BiomeDefinition {
            name: MOUNTAINS_BIOME_NAME,
            representative_color: RGBA8::new(220, 220, 220, 255),
            size_chunks: 2,
            elevation: VPElevation::Mountain,
            temperature: VPTemperature::Freezing,
            moisture: VPMoisture::LowMoist,
            rule_source: plains_rule_source.clone(),
            surface_noise: Box::new(noise_func),
        })
        .unwrap();
}