
use ocg_schemas::voxel::chunk_storage::{PaletteStorage, ChunkStorage};
use ocg_schemas::coordinates::{InChunkRange, InChunkPos, AbsBlockPos, CHUNK_DIM, AbsChunkPos};
use noise::{NoiseFn, Perlin};
use std::collections::HashMap;
use std::ops::{Add, Sub, Mul, Div};

pub mod newgen;

const MOUNTAIN_START_GLOBAL: i32 = 48;

pub fn generate(count_x: i32, count_y: i32, count_z: i32) -> HashMap<AbsChunkPos, PaletteStorage<u64>> {
    let simplex = Perlin::new(0);

    let mut chunks: HashMap<AbsChunkPos, PaletteStorage<u64>> = HashMap::with_capacity((count_x * count_z) as usize);
    let generator = newgen::StdGenerator::new(0);
    for chunk in chunks.iter_mut() {
        generator.generate_chunk(chunk.1);
    }

//    for chunk_x in 0..count_x {
//        for chunk_z in 0..count_z {
//            for chunk_y in 0..count_y {
//                chunks.insert(AbsBlockPos::new(chunk_x, chunk_y, chunk_z), {
//                    let mut chunk: PaletteStorage<u64> = PaletteStorage::default();
//            
//                    chunk.fill(InChunkRange::WHOLE_CHUNK, 0);
//            
//                    let chunk_y_pos = chunk_y * CHUNK_DIM;
//
//                    // 0 == air
//                    // 1 == stone
//                    // 2 == grass
//                    // 3 == dirt
//                    // 4 == snow
//                    for x in 0..CHUNK_DIM {
//                        for z in 0..CHUNK_DIM {
//                            let point: [f64; 2] = [(x as f64 / CHUNK_DIM as f64) + chunk_x as f64, (z as f64 / CHUNK_DIM as f64) + chunk_z as f64];
//                            let mut value: f64 = map_range((-1.0, 1.0), (0.0, (count_y * CHUNK_DIM) as f64), simplex.get(point));
//                            value -= chunk_y_pos as f64;
//                            let mut y = 0;
//                            while value > 0.0 && y < CHUNK_DIM {
//                                let pos = InChunkPos::try_new(x, y, z).unwrap();
//                                let global_y = y + chunk_y_pos;
//                                if global_y > (value - 1.0) as i32 && global_y <= (value + 1.0) as i32 {
//                                    if global_y >= MOUNTAIN_START_GLOBAL {
//                                        chunk.put(pos, 4); // snow
//                                    } else {
//                                        chunk.put(pos, 2); // grass
//                                    }
//                                } else if global_y < (value - 5.0) as i32 {
//                                    chunk.put(pos, 1); // stone
//                                } else if global_y <= value as i32 {
//                                    chunk.put(pos, 3); // dirt
//                                }
//                                value -= 1.0;
//                                y += 1;
//                            }
//                        }
//                    }
//                    chunk
//                });
//            }
//        }
//    }

    return chunks;
}

fn clamp<T: Copy + PartialOrd>(num: T, min: T, max: T) -> T {
    if num > max {
        return max;
    } else if num < min {
        return min;
    }
    return num;
}

fn map_range<T: Copy>(from_range: (T, T), to_range: (T, T), s: T) -> T 
    where T: Add<T, Output=T> +
             Sub<T, Output=T> +
             Mul<T, Output=T> +
             Div<T, Output=T>
{
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}