use glium::Surface;
use glutin::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ControlFlow},
};

mod assets;
mod display;

use assets::Assets;
use display::init_display;

pub fn run() {
    log::info!("starting up");
    let event_loop = EventLoop::new();
    log::info!("initializing display");
    let display = init_display(&event_loop);
    log::info!("loading assets");
    let assets = Assets::load_all_sync(&display);
    log::info!("game loaded");

    event_loop.run(move |ev, _, control_flow| {
        match ev {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return
                },
                _ => ()
            },
            _ => ()
        }
        let mut target = display.draw();
        target.clear_color_and_depth((0.5, 0.5, 1., 1.), 1.);
        target.finish().unwrap();
    });
}
