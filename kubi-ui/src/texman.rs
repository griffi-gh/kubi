use rect_packer::{Packer, Config as PackerConfig};
use glam::{IVec2, ivec2};

#[derive(Clone, Copy)]
pub struct TextureAllocation {
  // pub index: usize,
  pub position: IVec2,
  pub size: IVec2,
  in_texture_size: IVec2,
}

impl TextureAllocation {
  pub fn uv(&self) -> (f32, f32, f32, f32) {
    let p0 = self.position.as_vec2() / self.in_texture_size.as_vec2();
    let p1 = (self.position + self.size).as_vec2() / self.in_texture_size.as_vec2();
    (p0.x, p0.y, p1.x, p1.y)
  }
}

pub struct TextureSpace {
  size: IVec2,
  packer: Packer,
  // allocations: Vec<TextureAllocation>,
}

impl TextureSpace {
  pub fn new(size: IVec2) -> Self {
    Self {
      size,
      packer: Packer::new(PackerConfig {
        width: size.x,
        height: size.y,
        border_padding: 1,
        rectangle_padding: 1,
      }),
      // allocations: Vec::new(),
    }
  }

  pub fn size(&self) -> IVec2 {
    self.size
  }

  pub fn allocate(&mut self, size: IVec2) -> Option<TextureAllocation> {
    let position = self.packer.pack(size.x, size.y, false)
      .map(|rect| ivec2(rect.x, rect.y))?;
    Some(TextureAllocation {
      position,
      size,
      in_texture_size: self.size
    })
  }

  // pub fn lookup(&self, index: usize) -> Option<TextureAllocation> {
  //   self.allocations.get(index).copied()
  // }
}
