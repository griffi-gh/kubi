use glium::{Surface, uniform};
use glutin::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ControlFlow},
};

mod assets;
mod display;
mod shaders;
mod camera;

use assets::Assets;
use display::init_display;
use shaders::{Programs, Colored2dVertex};

pub fn run() {
    log::info!("starting up");
    let event_loop = EventLoop::new();
    log::info!("initializing display");
    let display = init_display(&event_loop);
    log::info!("compiling shaders");
    let programs = Programs::compile_all(&display);
    log::info!("loading assets");
    let assets = Assets::load_all_sync(&display);
    log::info!("game loaded");

    //=======================
    let vertex1 = Colored2dVertex { position: [-0.5, -0.5] };
    let vertex2 = Colored2dVertex { position: [ 0.0,  0.5] };
    let vertex3 = Colored2dVertex { position: [ 0.5, -0.25] };
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
        target.clear_color_and_depth((0.5, 0.5, 1., 1.), 1.);
        target.draw(&vertex_buffer, &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList), &programs.colored_2d, &uniform! { u_color: [1.0f32, 0.0f32, 0.0f32, 1.0f32] }, &Default::default()).unwrap();
        target.finish().unwrap();
    });
}
