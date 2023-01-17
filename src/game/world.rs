use glam::{Vec2, IVec2};
use glium::{
  Display, Frame, Surface, 
  DrawParameters, Depth, 
  DepthTest, uniform, 
  uniforms::{
    Sampler, SamplerBehavior, 
    MinifySamplerFilter, MagnifySamplerFilter,
  }
};
use hashbrown::HashMap;
use crate::game::{
  options::GameOptions,
  shaders::Programs,
  assets::Assets
};

mod chunk;
mod thread;

use chunk::{Chunk, ChunkState, CHUNK_SIZE};
use thread::WorldThreading;

const POSITIVE_X_NEIGHBOR: usize = 0;
const NEGATIVE_X_NEIGHBOR: usize = 1;
const POSITIVE_Z_NEIGHBOR: usize = 2;
const NEGATIVE_Z_NEIGHBOR: usize = 3;

const MAX_TASKS: usize = 6;

pub struct World {
  pub chunks: HashMap<IVec2, Chunk>,
  pub thread: WorldThreading,
}
impl World {
  pub fn chunk_neighbors(&self, position: IVec2) -> [Option<&Chunk>; 4] {
    [
      self.chunks.get(&(position + IVec2::new(1, 0))),
      self.chunks.get(&(position - IVec2::new(1, 0))),
      self.chunks.get(&(position + IVec2::new(0, 1))),
      self.chunks.get(&(position - IVec2::new(0, 1))),
    ]
  }

  pub fn new() -> Self {
    Self {
      chunks: HashMap::new(),
      thread: WorldThreading::new(),
    }
  }

  pub fn render(
    &self,
    target: &mut Frame, 
    programs: &Programs,
    assets: &Assets,
    perspective: [[f32; 4]; 4],
    view: [[f32; 4]; 4]
  ) {
    let sampler = SamplerBehavior {
      minify_filter: MinifySamplerFilter::Linear,
      magnify_filter: MagnifySamplerFilter::Nearest,
      max_anisotropy: 8,
      ..Default::default()
    };
    let draw_parameters = DrawParameters {
      depth: Depth {
        test: DepthTest::IfLess,
        write: true,
        ..Default::default()
      },
      #[cfg(feature = "polygon_rendering")] polygon_mode: glium::PolygonMode::Line,
      backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
      ..Default::default()
    };
    for (&position, chunk) in &self.chunks {
      if let Some(mesh) = &chunk.mesh {
        target.draw(
          &mesh.vertex_buffer,
          &mesh.index_buffer,
          &programs.chunk, 
          &uniform! {
            model: [
              [1., 0., 0., 0.],
              [0., 1., 0., 0.],
              [0., 0., 1., 0.],
              //[0., 0., 0., 1.0_f32]
              [(position.x * CHUNK_SIZE as i32) as f32, 0., (position.y * CHUNK_SIZE as i32) as f32, 1.0_f32]
            ],
            view: view,
            perspective: perspective,
            tex: Sampler(&assets.textures.block_atlas, sampler)
          }, 
          &draw_parameters
        ).unwrap();
      }
    }
  }

  pub fn update_loaded_chunks(&mut self, around_position: Vec2, options: &GameOptions, display: &Display) {
    let render_dist = options.render_distance as i32 + 1;
    let inside_chunk = (around_position / CHUNK_SIZE as f32).as_ivec2();

    //Mark all chunks for unload
    for (_, chunk) in &mut self.chunks {
      chunk.desired = ChunkState::Unload;
    }

    //Load new/update chunks in range
    for x in -render_dist..=render_dist {
      for z in -render_dist..=render_dist {
        let offset = IVec2::new(x, z);
        let position = inside_chunk + offset;
        if !self.chunks.contains_key(&position) {
          self.chunks.insert(position, Chunk::new(position));
        }
        {
          //we only need mutable reference here:
          let chunk = self.chunks.get_mut(&position).unwrap();
          if x == -render_dist || z == -render_dist || x == render_dist || z == render_dist {
            chunk.desired = ChunkState::Loaded;
          } else {
            chunk.desired = ChunkState::Rendered;
          } 
        }
        let chunk = self.chunks.get(&position).unwrap();
        if self.thread.task_amount() < MAX_TASKS {
          if matches!(chunk.state, ChunkState::Nothing) && matches!(chunk.desired, ChunkState::Loaded | ChunkState::Rendered) {
            self.thread.queue_load(position);
            self.chunks.get_mut(&position).unwrap().state = ChunkState::Loading;
          } else if matches!(chunk.state, ChunkState::Loaded) && matches!(chunk.desired, ChunkState::Rendered) {
            let mut state_changed = false;
            fn all_some<'a>(x: [Option<&'a Chunk>; 4]) -> Option<[&'a Chunk; 4]> {
              Some([x[0]?, x[1]?, x[2]?, x[3]?])
            }
            if let Some(neighbors) = all_some(self.chunk_neighbors(chunk.position)) {
              if {
                neighbors[0].block_data.is_some() &&
                neighbors[1].block_data.is_some() &&
                neighbors[2].block_data.is_some() &&
                neighbors[3].block_data.is_some()
              } {
                self.thread.queue_mesh(
                  position,
                  chunk.block_data.clone().unwrap(), 
                  [
                    neighbors[0].block_data.clone().unwrap(),
                    neighbors[1].block_data.clone().unwrap(),
                    neighbors[2].block_data.clone().unwrap(),
                    neighbors[3].block_data.clone().unwrap(),
                  ]
                );
                state_changed = true;
              }
            }
            if state_changed {
              self.chunks.get_mut(&position).unwrap().state = ChunkState::Rendering;
            }
          }
        }
      }
    }
    //Unloads and state downgrades
    self.chunks.retain(|_, chunk| {
      match chunk.desired {
        // Chunk unload
        ChunkState::Unload => false,
        // Any => Nothing downgrade
        ChunkState::Nothing => {
          chunk.block_data = None;
          chunk.mesh = None;
          chunk.state = ChunkState::Nothing;
          true
        },
        //Render => Loaded downgrade
        ChunkState::Loaded if matches!(chunk.state, ChunkState::Rendering | ChunkState::Rendered) => {
          chunk.mesh = None;
          chunk.state = ChunkState::Loaded;
          true
        },
        _ => true
      }
    });
    //Apply changes from threads
    self.thread.apply_tasks(&mut self.chunks, display);
  }
}
