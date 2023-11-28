use std::time::Instant;
use glam::{Vec2, IVec2, UVec2};
use glium::{backend::glutin::SimpleWindowBuilder, Surface};
use winit::{
  event::{Event, WindowEvent},
  event_loop::{EventLoopBuilder, ControlFlow}
};
use kubi_ui::{
  KubiUi,
  element::{
    UiElement,
    progress_bar::ProgressBar,
    container::{Container, Sides}
  },
  UiSize
};
use kubi_ui_glium::GliumUiRenderer;

fn main() {
  kubi_logging::init();

  let event_loop = EventLoopBuilder::new().build().unwrap();
  let (window, display) = SimpleWindowBuilder::new().build(&event_loop);

  let mut kui = KubiUi::new();
  let mut backend = GliumUiRenderer::new(&display);

  let instant = Instant::now();
  let mut pcnt = 0;
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
            Box::new(Container {
              gap: 1.,
              elements: {
                let mut elements: Vec<Box<dyn UiElement>> = vec![];
                let cnt = instant.elapsed().as_secs() * 10000;
                if pcnt != cnt {
                  log::info!("{cnt}");
                  pcnt = cnt;
                }
                for i in 0..cnt {
                  elements.push(Box::new(ProgressBar {
                    value: (instant.elapsed().as_secs_f32() + (i as f32 / 10.)).sin().powi(2),
                    size: (UiSize::Auto, UiSize::Pixels(5.)),
                    ..Default::default()
                  }));
                }
                elements
              },
              ..Default::default()
            })
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
