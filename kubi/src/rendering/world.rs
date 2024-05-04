// use glam::{ivec3, IVec3, Mat4, Quat, Vec3};
// use shipyard::{track, AllStoragesView, IntoIter, NonSendSync, Unique, UniqueView, UniqueViewMut, View};
// use glium::{
//   draw_parameters::{
//     BackfaceCullingMode, Depth, DepthTest, PolygonMode
//   }, implement_vertex, uniform, uniforms::{
//     MagnifySamplerFilter, MinifySamplerFilter, Sampler, SamplerBehavior, SamplerWrapFunction
//   }, Blend, DrawParameters, Smooth, Surface
// };
// use crate::{
//   camera::Camera,
//   player::MainPlayer,
//   transform::Transform,
//   prefabs::{
//     ChunkShaderPrefab,
//     BlockTexturesPrefab,
//     ColoredShaderPrefab,
//   },
//   world::{
//     ChunkStorage,
//     ChunkMeshStorage,
//     chunk::CHUNK_SIZE,
//   }, settings::GameSettings,
// };
// use super::{RenderTarget, primitives::cube::CubePrimitive};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ChunkVertex {
  pub position: [f32; 3],
  pub normal: [f32; 3],
  pub uv: [f32; 2],
  pub tex_index: u8,
}

// #[derive(Unique)]
// pub struct TransChunkQueue(pub Vec<IVec3>);

// pub fn init_trans_chunk_queue(storages: AllStoragesView) {
//   storages.add_unique(TransChunkQueue(Vec::with_capacity(512)));
// }

// fn draw_params(settings: &GameSettings) -> DrawParameters {
//   DrawParameters {
//     depth: Depth {
//       test: DepthTest::IfLess,
//       write: true,
//       ..Default::default()
//     },
//     multisampling: settings.msaa.is_some(),
//     polygon_mode: PolygonMode::Fill, //Change to Line for wireframe
//     backface_culling: BackfaceCullingMode::CullClockwise,
//     ..Default::default()
//   }
// }

// fn texture_sampler<'a, T>(texture: &'a T, settings: &GameSettings) -> Sampler<'a, T> {
//   Sampler(texture, SamplerBehavior {
//     minify_filter: MinifySamplerFilter::LinearMipmapLinear,
//     magnify_filter: MagnifySamplerFilter::Nearest,
//     max_anisotropy: settings.max_anisotropy.unwrap_or_default(),
//     wrap_function: (SamplerWrapFunction::Clamp, SamplerWrapFunction::Clamp, SamplerWrapFunction::Clamp),
//     depth_texture_comparison: None,
//   })
// }

// pub fn draw_world(
//   mut target: NonSendSync<UniqueViewMut<RenderTarget>>,
//   chunks: UniqueView<ChunkStorage>,
//   meshes: NonSendSync<UniqueView<ChunkMeshStorage>>,
//   program: NonSendSync<UniqueView<ChunkShaderPrefab>>,
//   texture: NonSendSync<UniqueView<BlockTexturesPrefab>>,
//   transform: View<Transform>,
//   camera: View<Camera>,
//   settings: UniqueView<GameSettings>,
//   mut trans_queue: UniqueViewMut<TransChunkQueue>,
// ) {
//   // let (camera, transform) = (&camera, &transform).iter().next().expect("No cameras in the scene");
//   // let camera_position = transform.0.to_scale_rotation_translation().2;

//   let camera = camera.iter().next().expect("No cameras in the scene");
//   let view = camera.view_matrix.to_cols_array_2d();
//   let perspective = camera.perspective_matrix.to_cols_array_2d();

//   let draw_parameters = draw_params(&settings);
//   let texture_sampler = texture_sampler(&texture.0, &settings);

//   for (&position, chunk) in &chunks.chunks {
//     if let Some(key) = chunk.mesh_index {
//       let mesh = meshes.get(key).expect("Mesh index pointing to nothing");
//       let world_position = position.as_vec3() * CHUNK_SIZE as f32;

//       //Skip mesh if its empty
//       if mesh.index_buffer.len() == 0 && mesh.trans_index_buffer.len() == 0 {
//         continue
//       }

//       //Frustum culling
//       {
//         let minp = world_position;
//         let maxp = world_position + Vec3::splat(CHUNK_SIZE as f32);
//         if !camera.frustum.is_box_visible(minp, maxp) {
//           continue
//         }
//       }

//       //Draw chunk mesh
//       if mesh.index_buffer.len() > 0 {
//         target.0.draw(
//           &mesh.vertex_buffer,
//           &mesh.index_buffer,
//           &program.0,
//           &uniform! {
//             position_offset: world_position.to_array(),
//             view: view,
//             perspective: perspective,
//             tex: texture_sampler,
//             discard_alpha: true,
//           },
//           &draw_parameters
//         ).unwrap();
//       }

//       if mesh.trans_index_buffer.len() > 0 {
//         trans_queue.0.push(position);
//       }
//     }
//   }

//   // const HALF_CHUNK_SIZE: IVec3 = IVec3::splat((CHUNK_SIZE >> 1) as i32);
//   // trans_queue.0.sort_by_cached_key(|&pos| -(
//   //   (pos + HALF_CHUNK_SIZE).distance_squared(camera_position.as_ivec3())
//   // ));
// }

// pub fn draw_world_trans(
//   mut target: NonSendSync<UniqueViewMut<RenderTarget>>,
//   chunks: UniqueView<ChunkStorage>,
//   meshes: NonSendSync<UniqueView<ChunkMeshStorage>>,
//   program: NonSendSync<UniqueView<ChunkShaderPrefab>>,
//   texture: NonSendSync<UniqueView<BlockTexturesPrefab>>,
//   camera: View<Camera>,
//   settings: UniqueView<GameSettings>,
//   mut trans_queue: UniqueViewMut<TransChunkQueue>,
// ) {
//   let camera = camera.iter().next().expect("No cameras in the scene");
//   let view = camera.view_matrix.to_cols_array_2d();
//   let perspective = camera.perspective_matrix.to_cols_array_2d();

//   let mut draw_parameters = draw_params(&settings);
//   draw_parameters.blend = Blend::alpha_blending();
//   draw_parameters.backface_culling = BackfaceCullingMode::CullingDisabled;
//   draw_parameters.smooth = Some(Smooth::Fastest);

//   let texture_sampler = texture_sampler(&texture.0, &settings);

//   for position in trans_queue.0.drain(..).rev() {
//     let world_position = position.as_vec3() * CHUNK_SIZE as f32;
//     let mesh_idx = chunks.chunks[&position].mesh_index.expect("No mesh index");
//     let mesh = meshes.get(mesh_idx).expect("Mesh index pointing to nothing");
//     target.0.draw(
//       &mesh.trans_vertex_buffer,
//       &mesh.trans_index_buffer,
//       &program.0,
//       &uniform! {
//         position_offset: world_position.to_array(),
//         view: view,
//         perspective: perspective,
//         tex: texture_sampler,
//         discard_alpha: false,
//       },
//       &draw_parameters
//     ).unwrap();
//   }
// }

// pub fn draw_current_chunk_border(
//   mut target: NonSendSync<UniqueViewMut<RenderTarget>>,
//   player: View<MainPlayer>,
//   transforms: View<Transform, track::All>,
//   buffers: NonSendSync<UniqueView<CubePrimitive>>,
//   program: NonSendSync<UniqueView<ColoredShaderPrefab>>,
//   camera: View<Camera>,
//   settings: UniqueView<GameSettings>,
// ) {
//   if cfg!(target_os = "android") {
//     return
//   }
//   if !settings.debug_draw_current_chunk_border {
//     return
//   }
//   let camera = camera.iter().next().expect("No cameras in the scene");
//   let view = camera.view_matrix.to_cols_array_2d();
//   let perspective = camera.perspective_matrix.to_cols_array_2d();
//   let (_, &player_transform) = (&player, &transforms).iter().next().expect("No player");
//   let (_, _, player_position) = player_transform.0.to_scale_rotation_translation();
//   let player_in_chunk = ivec3(
//     (player_position.x as i32).div_euclid(CHUNK_SIZE as i32),
//     (player_position.y as i32).div_euclid(CHUNK_SIZE as i32),
//     (player_position.z as i32).div_euclid(CHUNK_SIZE as i32),
//   );
//   let world_position = player_in_chunk.as_vec3() * CHUNK_SIZE as f32;
//   target.0.draw(
//     &buffers.0,
//     &buffers.1,
//     &program.0,
//     &uniform! {
//       model: Mat4::from_scale_rotation_translation(
//         Vec3::splat(CHUNK_SIZE as f32),
//         Quat::default(),
//         world_position
//       ).to_cols_array_2d(),
//       color: [0.25f32; 4],
//       view: view,
//       perspective: perspective,
//     },
//     &DrawParameters {
//       depth: Depth {
//         test: DepthTest::IfLess,
//         ..Default::default()
//       },
//       blend: Blend::alpha_blending(),
//       ..Default::default()
//     }
//   ).unwrap();
//   target.0.draw(
//     &buffers.0,
//     &buffers.1,
//     &program.0,
//     &uniform! {
//       model: Mat4::from_scale_rotation_translation(
//         Vec3::splat(CHUNK_SIZE as f32),
//         Quat::default(),
//         world_position
//       ).to_cols_array_2d(),
//       color: [0.0f32; 4],
//       view: view,
//       perspective: perspective,
//     },
//     &DrawParameters {
//       polygon_mode: PolygonMode::Point,
//       line_width: Some(2.),
//       point_size: Some(5.),
//       ..Default::default()
//     }
//   ).unwrap();
// }
