use std::time::Instant;
use glam::{UVec2, vec4};
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
    container::{Container, Sides, Alignment},
    rect::Rect, text::Text
  },
  interaction::IntoInteractable,
  UiSize,
  UiDirection, IfModified,
};
use kubi_ui_glium::GliumUiRenderer;

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
          align: (Alignment::Begin, Alignment::Begin),
          size: (UiSize::Percentage(1.), UiSize::Percentage(1.)),
          elements: vec![
            Box::new(Text {
              text: "Hello_world".into(),
              ..Default::default()
            }),
          ],
          ..Default::default()
        }, resolution);

        kui.end();

        backend.update(&kui);
        backend.draw(&mut frame, resolution);

        frame.finish().unwrap();
      }
      _ => (),
    }
  }).unwrap();
}
