
use ocg_schemas::voxel::chunk_storage::{PaletteStorage, ChunkStorage};
use ocg_schemas::coordinates::{InChunkRange, InChunkPos, AbsBlockPos, CHUNK_DIM};
use noise::{
    Fbm,
    Simplex, 
};
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
use std::collections::HashMap;


pub fn generate(count_x: i32, count_y: i32, count_z: i32) -> HashMap<AbsBlockPos, PaletteStorage<u64>> {
    let fbm = Fbm::<Simplex>::new(0);
    let simplex = Simplex::new(0);

    let count_y_f = (count_y * CHUNK_DIM) as f64;

    let noise_map = PlaneMapBuilder::<_, 2>::new(fbm)
        .set_size((count_x * CHUNK_DIM * 10) as usize, (count_z * CHUNK_DIM * 10) as usize)
        .set_x_bounds(0.0, count_y_f)
        .set_y_bounds(0.0, count_y_f)
        .build();

    let mut chunks: HashMap<AbsBlockPos, PaletteStorage<u64>> = HashMap::new();

    for chunk_x in 0..count_x {
        for chunk_z in 0..count_z {
            for chunk_y in 0..count_y {
                chunks.insert(AbsBlockPos::new(chunk_x, chunk_y, chunk_z), {
                    let mut chunk: PaletteStorage<u64> = PaletteStorage::default();
            
                    chunk.fill(InChunkRange::WHOLE_CHUNK, 0);
            
                    for x in 0..CHUNK_DIM - 1 {
                        for z in 0..CHUNK_DIM - 1 {
                            for y in 0..(noise_map.get_value((x + chunk_x * CHUNK_DIM) as usize, (z + chunk_z * CHUNK_DIM) as usize) as i32 * CHUNK_DIM - (chunk_y * CHUNK_DIM)) {
                                chunk.put(InChunkPos::try_new(x, y, z).unwrap(), 1);
                            }
                        }
                    }
                    chunk
                });
            }
        }
    }

    noise_map.write_to_file("noise_map.png");

    return chunks;
}