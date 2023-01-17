use glam::IVec2;
use crate::game::world::chunk::ChunkData;
use crate::game::shaders::chunk::Vertex as ChunkVertex;

pub fn generate_mesh(position: IVec2, chunk_data: ChunkData, neighbors: [ChunkData; 4]) -> (Vec<ChunkVertex>, Vec<u16>) {
  let mut vertex = Vec::new();
  let mut index = Vec::new();
  vertex.push(ChunkVertex { position: [-0.5, -0.5, 0.], uv: [0., 0.], normal: [0., 1., 0.] });
  vertex.push(ChunkVertex { position: [ 0.0,  0.5, 0.], uv: [0., 1.], normal: [0., 1., 0.] });
  vertex.push(ChunkVertex { position: [ 0.5, -0.5, 0.], uv: [1., 1.], normal: [0., 1., 0.] });
  index.push(0);
  index.push(1);
  index.push(2);
  (vertex, index)
}
