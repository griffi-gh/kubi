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
    progress_bar::ProgressBar,
    container::{Container, Sides, Alignment},
    text::Text
  },
  UiSize,
  elements,
};
use kubi_ui_glium::GliumUiRenderer;

fn main() {
  kubi_logging::init();

  let event_loop = EventLoopBuilder::new().build().unwrap();
  let (window, display) = SimpleWindowBuilder::new().build(&event_loop);
  window.set_title("Mom Downloader 2000");

  let mut kui = KubiUi::new();
  let mut backend = GliumUiRenderer::new(&display);

  let font_handle = kui.add_font_from_bytes(include_bytes!("../../assets/fonts/roboto/Roboto-Regular.ttf"));

  let instant = Instant::now();

  event_loop.run(|event, window_target| {
    window_target.set_control_flow(ControlFlow::Poll);
    match event {
      Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
        window_target.exit();
      },
      Event::AboutToWait => {
        let mut frame = display.draw();
        frame.clear_color_srgb(0., 0., 0., 1.);

        let resolution = UVec2::from(display.get_framebuffer_dimensions()).as_vec2();

        kui.begin();

        let mom_ratio = (instant.elapsed().as_secs_f32() / 60.).powf(0.5);

        kui.add(Container {
          align: (Alignment::Center, Alignment::Center),
          size: (UiSize::Percentage(1.), UiSize::Percentage(1.)),
          background: Some(vec4(0.1, 0.1, 0.1, 1.)),
          elements: vec![Box::new(Container {
            gap: 5.,
            padding: Sides::all(10.),
            align: (Alignment::Begin, Alignment::Begin),
            size: (UiSize::Pixels(450.), UiSize::Auto),
            background: Some(vec4(0.2, 0.2, 0.5, 1.)),
            elements: elements(|el| {
              if instant.elapsed().as_secs_f32() < 5. {
                el.add(Text {
                  text: "Downloading your mom...".into(),
                  font: font_handle,
                  text_size: 32,
                  ..Default::default()
                });
                el.add(ProgressBar {
                  value: mom_ratio,
                  ..Default::default()
                });
                el.add(Text {
                  text: format!("{:.2}% ({:.1} GB)", mom_ratio * 100., mom_ratio * 10000.).into(),
                  font: font_handle,
                  text_size: 24,
                  ..Default::default()
                });
              } else if instant.elapsed().as_secs() < 10 {
                el.add(Text {
                  text: "Error 413 Request Entity Too Large".into(),
                  font: font_handle,
                  color: vec4(1., 0.125, 0.125, 1.),
                  text_size: 26,
                  ..Default::default()
                });
                el.add(Text {
                  text: format!("Exiting in {}...", 10 - instant.elapsed().as_secs()).into(),
                  font: font_handle,
                  text_size: 24,
                  ..Default::default()
                })
              } else {
                window_target.exit();
              }
            }),
            ..Default::default()
          })],
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
