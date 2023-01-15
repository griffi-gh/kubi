use glium::{Surface, uniform};
use glium::glutin::{
  event::{Event, WindowEvent, DeviceEvent, VirtualKeyCode},
  event_loop::{EventLoop, ControlFlow},
};
use std::time::Instant;

mod assets;
mod display;
mod shaders;
mod camera;
mod controller;

use assets::Assets;
use display::init_display;
use shaders::{Programs, chunk::Vertex as ChunkVertex};
use camera::Camera;
use controller::Controls;

struct State {
  pub camera: Camera,
  pub controls: Controls, 
}
impl State {
  pub fn init() -> Self {
    Self {
      camera: Camera::default(),
      controls: Controls::default(),
    }
  }
}

pub fn run() {
  log::info!("starting up");
  let event_loop = EventLoop::new();
  log::info!("initializing display");
  let display = init_display(&event_loop);
  log::info!("compiling shaders");
  let programs = Programs::compile_all(&display);
  log::info!("loading assets");
  let assets = Assets::load_all_sync(&display);
  log::info!("init game state");
  let mut state = State::init();
  log::info!("game loaded");

  //=======================
  let vertex1 = ChunkVertex { position: [-0.5, -0.5, 0.], uv: [0., 0.], normal: [0., 1., 0.] };
  let vertex2 = ChunkVertex { position: [ 0.0,  0.5, 0.], uv: [0., 1.], normal: [0., 1., 0.] };
  let vertex3 = ChunkVertex { position: [ 0.5, -0.25, 0.], uv: [1., 1.], normal: [0., 1., 0.] };
  let shape = vec![vertex1, vertex2, vertex3];
  let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
  //=======================

  let mut last_render = Instant::now();

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Poll;
    match event {
      Event::MainEventsCleared => (),
      Event::DeviceEvent {
        event: DeviceEvent::MouseMotion{ delta, },
        ..
      } => {
        state.controls.process_mouse_input(delta.0, delta.1);
      }
      Event::WindowEvent { event, .. } => {
        match event {
          WindowEvent::CloseRequested => {
            log::info!("exit requested");
            *control_flow = ControlFlow::Exit;
            return
          },
          _ => return
        }
      },
      _ => return
    }
    
    let now = Instant::now();
    let dt = (now - last_render).as_secs_f32();
    last_render = now;

    let actions = state.controls.calculate(dt);
    actions.apply_to_camera(&mut state.camera);

    let mut target = display.draw();
    let target_dimensions = target.get_dimensions();
    let perspective = state.camera.perspective_matrix(target_dimensions);
    let view = state.camera.view_matrix();
    target.clear_color_and_depth((0.5, 0.5, 1., 1.), 1.);
    target.draw(
      &vertex_buffer,
      glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList), 
      &programs.chunk, 
      &uniform! { 
        model: [
          [1., 0., 0., 0.],
          [0., 1., 0., 0.],
          [0., 0., 1., 0.],
          [0., 0., 0., 1.0_f32]
        ],
        view: view,
        perspective: perspective,
        tex: &assets.textures.block_atlas
      }, 
      &Default::default()
    ).unwrap();
    target.finish().unwrap();
  });
}
