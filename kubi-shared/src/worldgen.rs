use glam::{IVec3, ivec3};
use bracket_noise::prelude::*;
use crate::{
  chunk::{BlockData, CHUNK_SIZE},
  blocks::Block
};

fn local_height(height: i32, chunk_position: IVec3) -> usize {
  let offset = chunk_position * CHUNK_SIZE as i32;
  (height - offset.y).clamp(0, CHUNK_SIZE as i32) as usize
}
fn local_y_position(height: i32, chunk_position: IVec3) -> Option<usize> {
  let offset = chunk_position * CHUNK_SIZE as i32;
  let position = height - offset.y;
  (0..CHUNK_SIZE as i32).contains(&position).then_some(position as usize)
}

pub fn generate_world(chunk_position: IVec3, seed: u64) -> BlockData {
  let offset = chunk_position * CHUNK_SIZE as i32;
  let mut blocks = Box::new([[[Block::Air; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]);
  
  let mut height_noise = FastNoise::seeded(seed);
  height_noise.set_fractal_type(FractalType::FBM);
  height_noise.set_fractal_octaves(4);
  height_noise.set_frequency(0.003);

  let mut elevation_noise = FastNoise::seeded(seed);
  elevation_noise.set_fractal_type(FractalType::FBM);
  elevation_noise.set_fractal_octaves(1);
  elevation_noise.set_frequency(0.001);

  // let mut cave_noise = FastNoise::seeded(seed.rotate_left(1));
  // cave_noise.set_fractal_type(FractalType::FBM);
  // cave_noise.set_fractal_octaves(2);
  // cave_noise.set_frequency(0.001);

  //Generate height map
  let mut within_heightmap = false;
  for x in 0..CHUNK_SIZE {
    for z in 0..CHUNK_SIZE {
      let (noise_x, noise_y) = ((offset.x + x as i32) as f32, (offset.z + z as i32) as f32);
      //sample noises
      let raw_heightmap_value = height_noise.get_noise(noise_x, noise_y);
      let raw_elevation_value = elevation_noise.get_noise(noise_x, noise_y);
      //compute height
      let height = {
        let local_elevation = raw_elevation_value.powi(4).sqrt();
        (raw_heightmap_value.clamp(-1., 1.) * local_elevation * 100.) as i32
      };
      //place dirt
      for y in 0..local_height(height, chunk_position) {
        blocks[x][y][z] = Block::Dirt;
        within_heightmap = true;
      }
      //place stone
      for y in 0..local_height(height - 5 - (raw_heightmap_value * 5.) as i32, chunk_position) {
        blocks[x][y][z] = Block::Stone;
        within_heightmap = true;
      }
      //place grass
      if let Some(y) = local_y_position(height, chunk_position) {
        blocks[x][y][z] = Block::Grass;
      }
    }
  }
  
  //Carve out mountains
  if within_heightmap {
    // for z in 0..CHUNK_SIZE {
    //   for y in 0..CHUNK_SIZE {
    //     for x in 0..CHUNK_SIZE {
    //       if blocks[x][y][z] == Block::Air { continue }
    //       let position = ivec3(x as i32, y as i32, z as i32) + offset;
    //       let raw_cavemap_value = cave_noise.get_noise3d(position.x as f32, position.y as f32, position.z as f32);
    //       let is_cave = (-0.3..=-0.3).contains(&raw_cavemap_value);
    //       if is_cave {
    //         blocks[x][y][z] = Block::Air;
    //       }
    //     }
    //   }
    // }
  }

  blocks

  // let mut cave_noise = FastNoise::seeded(seed);
  // cave_noise.set_fractal_type(FractalType::FBM);
  // cave_noise.set_frequency(0.1);

  // let mut dirt_noise = FastNoise::seeded(seed.rotate_left(1));
  // dirt_noise.set_fractal_type(FractalType::FBM);
  // dirt_noise.set_frequency(0.1);

  // 

  // if chunk_position.y >= 0 {
  //   if chunk_position.y == 0 {
  //     for x in 0..CHUNK_SIZE {
  //       for z in 0..CHUNK_SIZE {
  //         blocks[x][0][z] = Block::Dirt;
  //         blocks[x][1][z] = Block::Grass;
  //       }
  //     }
  //   }
  // } else {
  //   for x in 0..CHUNK_SIZE {
  //     for y in 0..CHUNK_SIZE {
  //       for z in 0..CHUNK_SIZE {
  //         let position = ivec3(x as i32, y as i32, z as i32) + offset;
  //         let v_cave_noise = cave_noise.get_noise3d(position.x as f32, position.y as f32, position.z as f32) * (-position.y as f32 - 10.0).clamp(0., 1.);
  //         let v_dirt_noise = dirt_noise.get_noise3d(position.x as f32, position.y as f32, position.z as f32) * (-position.y as f32).clamp(0., 1.);
  //         if v_cave_noise > 0.5 {
  //           blocks[x][y][z] = Block::Stone;
  //         } else if v_dirt_noise > 0.5 {
  //           blocks[x][y][z] = Block::Dirt;
  //         }
  //       }
  //     }
  //   }
  // }
  // blocks

}
