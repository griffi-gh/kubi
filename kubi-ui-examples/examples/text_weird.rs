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
    text::Text, rect::Rect
  },
  UiSize,
  elements,
};
use kubi_ui_glium::GliumUiRenderer;

fn main() {
  kubi_logging::init();

  let event_loop = EventLoopBuilder::new().build().unwrap();
  let (window, display) = SimpleWindowBuilder::new().build(&event_loop);
  window.set_title("Text rendering test");

  let mut kui = KubiUi::new();
  let mut backend = GliumUiRenderer::new(&display);

  let font_handle = kui.add_font_from_bytes(include_bytes!("../../assets/fonts/roboto/Roboto-Regular.ttf"));

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

        kui.add(Container {
          size: (UiSize::Percentage(1.), UiSize::Percentage(1.)),
          background: Some(vec4(0.1, 0.1, 0.1, 1.)),
          elements: elements(|elem| {
            elem.add(Text {
              text: "THIS LINE SHOULD BE SHARP!".into(),
              ..Default::default()
            });
            elem.add(Text {
              text: "THIS LINE SHOULD BE SHARP!".into(),
              text_size: 32,
              ..Default::default()
            });
            elem.add(Text {
              text: "All lines except 3 and 6 below will be blurry:".into(),
              ..Default::default()
            });
            for size in [9, 12, 16, 18, 24, 32] {
              elem.add(Text {
                text: "Testing default font, Proggy Tiny".into(),
                text_size: size,
                ..Default::default()
              });
            }
            elem.add(Rect {
              size: (UiSize::Percentage(1.), UiSize::Pixels(10.)),
              color: Some(vec4(0., 0., 1., 1.)),
            });
            elem.add(Rect {
              size: (UiSize::Percentage(1.), UiSize::Pixels(10.)),
              color: Some(vec4(1., 1., 0., 1.)),
            });
            elem.add(Text {
              text: "Hello, world!\nżółty liść. życie nie ma sensu i wszyscy zginemy;\nтест кирилиці їїїїїїїїїїї\njapanese text: テスト".into(),
              font: font_handle,
              text_size: 32,
              ..Default::default()
            });
            elem.add(Rect {
              size: (UiSize::Percentage(1.), UiSize::Pixels(10.)),
              color: Some(vec4(1., 0., 0., 1.)),
            });
            elem.add(Rect {
              size: (UiSize::Percentage(1.), UiSize::Pixels(10.)),
              color: Some(vec4(0., 0., 0., 1.)),
            });
            elem.add(Text {
              text: "OVERLAP TEST".into(),
              font: font_handle,
              text_size: 15,
              ..Default::default()
            });
          }),
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
