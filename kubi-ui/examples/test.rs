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
    rect::Rect
  },
  interaction::IntoInteractable,
  UiSize,
  UiDirection,
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

        let z = instant.elapsed().as_secs_f32().sin().powi(2);

        kui.add(Container {
          gap: 5.,
          padding: Sides::all(5.),
          align: (Alignment::Begin, Alignment::Center),
          size: (UiSize::Percentage(1.), UiSize::Percentage(1.)),
          elements: vec![
            Box::new(ProgressBar {
              value: 0.5,
              ..Default::default()
            }),
          ],
          ..Default::default()
        }, resolution);

        kui.add(Container {
          gap: 5.,
          padding: Sides::all(5.),
          align: (Alignment::End, Alignment::Center),
          size: (UiSize::Percentage(1.), UiSize::Percentage(1.)),
          elements: vec![
            Box::new(ProgressBar {
              value: z,
              ..Default::default()
            }),
            Box::new(Container {
              size: (UiSize::Percentage(1.), UiSize::Auto),
              align: (Alignment::Center, Alignment::End),
              padding: Sides::all(5.),
              gap: 10.,
              elements: vec![
                Box::new(Rect {
                  size: (UiSize::Percentage(0.5), UiSize::Pixels(30.)),
                  color: Some(vec4(0.75, 0., 0., 1.))
                }),
                Box::new(Rect {
                  size: (UiSize::Percentage(z / 2. + 0.5), UiSize::Pixels(30.)),
                  color: Some(vec4(0., 0.75, 0., 1.))
                }),
              ],
              ..Default::default()
            }),
            Box::new(Rect {
              size: (UiSize::Percentage(z / 2. + 0.5), UiSize::Pixels(30.)),
              color: Some(vec4(0., 0.75, 0., 1.))
            }),
            Box::new(Container {
              gap: 5.,
              padding: Sides::all(5.),
              background: Some(vec4(0., 0., 0., 0.5)),
              direction: UiDirection::Horizontal,
              elements: {
                let mut x: Vec<Box<dyn UiElement>> = vec![];
                for i in 0..10 {
                  x.push(Box::new(Rect {
                    size: (UiSize::Pixels(50.), UiSize::Pixels(50.)),
                    color: if i == 1 {
                      Some(vec4(0.75, 0.75, 0.75, 0.75))
                    } else {
                      Some(vec4(0.5, 0.5, 0.5, 0.75))
                    }
                  }));
                }
                x
              },
              ..Default::default()
            }),
            Box::new(Container {
              background: Some(vec4(1., 0., 0., 1.)),
              padding: Sides::horizontal_vertical(30., 5.),
              elements: vec![
                Box::new(Rect {
                  size: (UiSize::Pixels(50.), UiSize::Pixels(50.)),
                  color: Some(vec4(1., 1., 1., 0.75))
                }.into_interactable().on_click(|| {
                  println!("clicked");
                }))
              ],
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
