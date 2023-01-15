use glium::{Surface, uniform};
use glium::glutin::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ControlFlow},
};

mod assets;
mod display;
mod shaders;
mod camera;

use assets::Assets;
use display::init_display;
use shaders::{Programs, chunk::Vertex as ChunkVertex};
use camera::Camera;

pub fn run() {
    log::info!("starting up");
    let event_loop = EventLoop::new();
    log::info!("initializing display");
    let display = init_display(&event_loop);
    log::info!("compiling shaders");
    let programs = Programs::compile_all(&display);
    log::info!("loading assets");
    let assets = Assets::load_all_sync(&display);
    log::info!("init camera");
    let mut camera = Camera::default();
    camera.position = [0., 0., -1.];
    camera.direction = [0., 0., 1.];
    log::info!("game loaded");

    //=======================
    let vertex1 = ChunkVertex { position: [-0.5, -0.5, 1.], uv: [0., 0.], normal: [0., 1., 0.] };
    let vertex2 = ChunkVertex { position: [ 0.0,  0.5, 1.], uv: [0., 1.], normal: [0., 1., 0.] };
    let vertex3 = ChunkVertex { position: [ 0.5, -0.25, 1.], uv: [1., 1.], normal: [0., 1., 0.] };
    let shape = vec![vertex1, vertex2, vertex3];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    //=======================

    event_loop.run(move |ev, _, control_flow| {
        match ev {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    log::info!("exit requested");
                    *control_flow = ControlFlow::Exit;
                    return
                },
                _ => ()
            },
            _ => ()
        }
        let mut target = display.draw();
        let target_dimensions = target.get_dimensions();
        let perspective = camera.perspective_matrix(target_dimensions);
        let view = camera.view_matrix();
        target.clear_color_and_depth((0.5, 0.5, 1., 1.), 1.);
        target.draw(
            &vertex_buffer,
            &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList), 
            &programs.chunk, 
            &uniform! { 
                model: [[0.0f32; 4]; 4],
                view: view,
                perspective: perspective,
                tex: &assets.textures.block_atlas
            }, 
            &Default::default()
        ).unwrap();
        target.finish().unwrap();
    });
}
