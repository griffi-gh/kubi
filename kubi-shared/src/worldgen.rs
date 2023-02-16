use bracket_noise::prelude::*;
use rand::prelude::*;
use glam::{IVec3, ivec3, Vec3Swizzles, IVec2};
use rand_xoshiro::Xoshiro256StarStar;
use crate::{
  chunk::{BlockData, CHUNK_SIZE},
  block::Block
};

fn mountain_ramp(mut x: f32) -> f32 {
  x = x * 2.0;
  if x < 0.4 {
    0.5 * x
  } else if x < 0.55 {
    4. * (x - 0.4) + 0.2
  } else {
    0.4444 * (x - 0.55) + 0.8
  }
}

fn local_height(height: i32, chunk_position: IVec3) -> usize {
  let offset = chunk_position * CHUNK_SIZE as i32;
  (height - offset.y).clamp(0, CHUNK_SIZE as i32) as usize
}

fn local_y_position(height: i32, chunk_position: IVec3) -> Option<usize> {
  let offset = chunk_position * CHUNK_SIZE as i32;
  let position = height - offset.y;
  (0..CHUNK_SIZE as i32).contains(&position).then_some(position as usize)
}

pub struct QueuedBlock {
  pub position: IVec3,
  pub block_type: Block,
}

pub fn generate_world(chunk_position: IVec3, seed: u64) -> (BlockData, Vec<QueuedBlock>) {
  let offset = chunk_position * CHUNK_SIZE as i32;
  let mut blocks = Box::new([[[Block::Air; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]);
  let mut queue = Vec::with_capacity(0);

  let mut smart_place = |blocks: &mut BlockData, position: IVec3, block: Block| {
    if position.to_array().iter().any(|&x| !(0..CHUNK_SIZE).contains(&(x as usize))) {
      let event_pos = offset + position;
      queue.retain(|block: &QueuedBlock| {
        block.position != event_pos
      });
      queue.push(QueuedBlock {
        position: event_pos, 
        block_type: block
      });
    } else {
      blocks[position.x as usize][position.y as usize][position.z as usize] = block;
    }
  };

  let mut height_noise = FastNoise::seeded(seed);
  height_noise.set_fractal_type(FractalType::FBM);
  height_noise.set_fractal_octaves(4);
  height_noise.set_frequency(0.003);

  let mut elevation_noise = FastNoise::seeded(seed.rotate_left(1));
  elevation_noise.set_fractal_type(FractalType::FBM);
  elevation_noise.set_fractal_octaves(1);
  elevation_noise.set_frequency(0.001);

  let mut rng = Xoshiro256StarStar::seed_from_u64(
    seed
    ^ ((chunk_position.x as u32 as u64) << 0)
    ^ ((chunk_position.z as u32 as u64) << 32)
  );
  let rng_map_a: [[f32; CHUNK_SIZE]; CHUNK_SIZE] = rng.gen();
  let rng_map_b: [[f32; CHUNK_SIZE]; CHUNK_SIZE] = rng.gen();

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
        let mut height = (mountain_ramp(raw_heightmap_value) * local_elevation * 100.) as i32;
        if height < 0 { height /= 2 }
        height
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
      //place tall grass
      if rng_map_a[x][z] < 0.03 {
        if let Some(y) = local_y_position(height + 1, chunk_position) {
          blocks[x][y][z] = Block::TallGrass;
        }
      }
      //place trees!
      if rng_map_a[x][z] < 0.001 {
        if let Some(y) = local_y_position(height + 1, chunk_position) {
          let tree_pos = ivec3(x as i32, y as i32, z as i32);
          let tree_height = 4 + (rng_map_b[x][z] * 3.).round() as i32;
          for tree_y in 0..tree_height {
            smart_place(&mut blocks, tree_pos + IVec3::Y * tree_y, Block::Wood);
          }
          // Part that wraps around the tree
          {
            let tree_leaf_height = tree_height - 3;
            let leaf_width = 2;
            for tree_y in tree_leaf_height..tree_height {
              for tree_x in (-leaf_width)..=leaf_width {
                for tree_z in (-leaf_width)..=leaf_width {
                  let tree_offset = ivec3(tree_x, tree_y, tree_z);
                  if tree_offset.xz() == IVec2::ZERO { continue }
                  smart_place(&mut blocks, tree_pos + tree_offset, Block::Leaf);
                }
              }
            }
          }
          //part above the tree
          {
            let leaf_above_height = 2;
            let leaf_width = 1;
            for tree_y in tree_height..(tree_height + leaf_above_height) {
              for tree_x in (-leaf_width)..=leaf_width {
                for tree_z in (-leaf_width)..=leaf_width {
                  let tree_offset = ivec3(tree_x, tree_y, tree_z);
                  smart_place(&mut blocks, tree_pos + tree_offset, Block::Leaf);
                }
              }
            }
          }
        }
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

  (blocks, queue)
  
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
