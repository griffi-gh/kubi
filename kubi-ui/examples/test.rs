use std::time::Instant;
use glam::{Vec2, IVec2, UVec2};
use glium::{backend::glutin::SimpleWindowBuilder, Surface};
use winit::{
  event::{Event, WindowEvent},
  event_loop::{EventLoopBuilder, ControlFlow}
};
use kubi_ui::{
  KubiUi,
  backend::glium::GliumUiRenderer, element::{progress_bar::ProgressBar, container::{Container, Sides}, UiElement}, UiSize
};

fn main() {
  kubi_logging::init();

  let event_loop = EventLoopBuilder::new().build().unwrap();
  let (window, display) = SimpleWindowBuilder::new().build(&event_loop);

  let mut kui = KubiUi::new();
  let mut backend = GliumUiRenderer::new(&display);

  let instant = Instant::now();
  event_loop.run(|event, window_target| {
    window_target.set_control_flow(ControlFlow::Poll);
    match event {
      Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
        window_target.exit();
      },
      Event::AboutToWait => {
        let mut frame = display.draw();
        frame.clear_color_srgb(0.5, 0.5, 0.5, 0.);

        let resolution = UVec2::from(display.get_framebuffer_dimensions()).as_vec2();

        kui.begin();

        kui.add(Container {
          gap: 5.,
          padding: Sides::all(5.),
          elements: vec![
            Box::new(ProgressBar {
              value: 0.5,
              ..Default::default()
            }),
            Box::new(ProgressBar {
              value: instant.elapsed().as_secs_f32().sin().powi(2),
              ..Default::default()
            }),
          ],
          ..Default::default()
        }, resolution);

        kui.end();

        let plan = kui.draw_plan();
        if plan.0 {
          backend.update(plan.1);
        }
        backend.draw(&mut frame, resolution);

        frame.finish().unwrap();
      }
      _ => (),
    }
  }).unwrap();
}