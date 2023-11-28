use glam::{Vec2, vec2, Vec4};
use crate::{
  UiDirection,
  UiSize,
  LayoutInfo,
  draw::UiDrawCommand,
  measure::{Response, Hints},
  state::StateRepo,
  element::UiElement
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Alignment {
  Begin,
  Center,
  End,
}

pub struct Border {
  pub color: Vec4,
  pub width: f32,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Sides<T> {
  pub top: T,
  pub bottom: T,
  pub left: T,
  pub right: T,
}

impl<T: Clone> Sides<T> {
  #[inline]
  pub fn all(value: T) -> Self {
    Self {
      top: value.clone(),
      bottom: value.clone(),
      left: value.clone(),
      right: value,
    }
  }

  #[inline]
  pub fn horizontal_vertical(horizontal: T, vertical: T) -> Self {
    Self {
      top: vertical.clone(),
      bottom: vertical,
      left: horizontal.clone(),
      right: horizontal,
    }
  }
}

pub struct Container {
  // pub min_size: (UiSize, UiSize),
  // pub max_size: (UiSize, UiSize),
  pub size: (UiSize, UiSize),
  pub direction: UiDirection,
  //pub reverse: bool,
  pub gap: f32,
  pub padding: Sides<f32>,
  pub align: (Alignment, Alignment),
  pub background: Option<Vec4>,
  pub borders: Sides<Option<Border>>,
  pub clip: bool,
  pub elements: Vec<Box<dyn UiElement>>,
}

impl Default for Container {
  fn default() -> Self {
    Self {
      // min_size: (UiSize::Auto, UiSize::Auto),
      // max_size: (UiSize::Auto, UiSize::Auto),
      size: (UiSize::Auto, UiSize::Auto),
      direction: UiDirection::Vertical,
      //reverse: false,
      gap: 0.,
      padding: Sides::all(0.),
      ///Primary/secondary axis
      align: (Alignment::Begin, Alignment::Begin),
      background: Default::default(),
      borders: Default::default(),
      clip: Default::default(),
      elements: Vec::new(),
    }
  }
}

impl Container {
  pub fn measure_max_inner_size(&self, layout: &LayoutInfo) -> Vec2 {
    //TODO take explicit size into account
    layout.max_size - vec2(
      self.padding.left + self.padding.right,
      self.padding.top + self.padding.bottom,
    )
  }
}

impl UiElement for Container {
  fn measure(&self, state: &StateRepo, layout: &LayoutInfo) -> Response {
    let mut size = Vec2::ZERO;
    //if matches!(self.size.0, UiSize::Auto) || matches!(self.size.1, UiSize::Auto) {
    let mut leftover_gap = Vec2::ZERO;
    for element in &self.elements {
      let measure = element.measure(state, &LayoutInfo {
        position: layout.position + size,
        max_size: self.measure_max_inner_size(layout), // - size TODO
        direction: self.direction,
      });
      match self.direction {
        UiDirection::Horizontal => {
          size.x += measure.size.x + self.gap;
          size.y = size.y.max(measure.size.y);
          leftover_gap.x = self.gap;
        },
        UiDirection::Vertical => {
          size.x = size.x.max(measure.size.x);
          size.y += measure.size.y + self.gap;
          leftover_gap.y = self.gap;
        }
      }
    }
    size -= leftover_gap;

    let inner_content_size = Some(size);

    size += vec2(
      self.padding.left + self.padding.right,
      self.padding.top + self.padding.bottom,
    );

    match self.size.0 {
      UiSize::Auto => (),
      UiSize::Percentage(percentage) => size.x = layout.max_size.x * percentage,
      UiSize::Pixels(pixels) => size.x = pixels,
    }
    match self.size.1 {
      UiSize::Auto => (),
      UiSize::Percentage(percentage) => size.y = layout.max_size.y * percentage,
      UiSize::Pixels(pixels) => size.y = pixels,
    }

    Response {
      size,
      hints: Hints {
        inner_content_size,
        ..Default::default()
      },
      user_data: None
    }
  }

  fn process(&self, measure: &Response, state: &mut StateRepo, layout: &LayoutInfo, draw: &mut Vec<UiDrawCommand>) {
    let mut position = layout.position;

    //background
    if let Some(color) = self.background {
      draw.push(UiDrawCommand::Rectangle {
        position,
        size: measure.size,
        color
      });
    }

    //padding
    position += vec2(self.padding.left, self.padding.top);

    //alignment
    match (self.align.0, self.direction) {
      (Alignment::Begin, _) => (),
      (Alignment::Center, UiDirection::Horizontal) => {
        position.x += (measure.size.x - measure.hints.inner_content_size.unwrap().x) / 2.;
      },
      (Alignment::Center, UiDirection::Vertical) => {
        position.y += (measure.size.y - measure.hints.inner_content_size.unwrap().y) / 2.;
      },
      (Alignment::End, UiDirection::Horizontal) => {
        position.x += measure.size.x - measure.hints.inner_content_size.unwrap().x - self.padding.right - self.padding.left;
      },
      (Alignment::End, UiDirection::Vertical) => {
        position.y += measure.size.y - measure.hints.inner_content_size.unwrap().y - self.padding.bottom - self.padding.top;
      }
    }

    for element in &self.elements {
      //(passing max size from layout rather than actual bounds for the sake of consistency with measure() above)

      let mut el_layout = LayoutInfo {
        position,
        max_size: self.measure_max_inner_size(layout),
        direction: self.direction,
      };

      //measure
      let el_measure = element.measure(state, &el_layout);

      //align (on sec. axis)
      match (self.align.1, self.direction) {
        (Alignment::Begin, _) => (),
        (Alignment::Center, UiDirection::Horizontal) => {
          el_layout.position.y += (measure.size.y - self.padding.bottom - self.padding.top - el_measure.size.y) / 2.;
        },
        (Alignment::Center, UiDirection::Vertical) => {
          el_layout.position.x += (measure.size.x - self.padding.left - self.padding.right - el_measure.size.x) / 2.;
        },
        (Alignment::End, UiDirection::Horizontal) => {
          el_layout.position.y += measure.size.y - el_measure.size.y - self.padding.bottom;
        },
        (Alignment::End, UiDirection::Vertical) => {
          el_layout.position.x += measure.size.x - el_measure.size.x - self.padding.right;
        }
      }

      //process
      element.process(&el_measure, state, &el_layout, draw);

      //layout
      match self.direction {
        UiDirection::Horizontal => {
          position.x += el_measure.size.x + self.gap;
        },
        UiDirection::Vertical => {
          position.y += el_measure.size.y + self.gap;
        }
      }
    }
  }
}
