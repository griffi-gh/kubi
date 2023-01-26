use strum::EnumIter;
use crate::prefabs::BlockTexture;

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter)]
#[repr(u8)]
pub enum Block {
  Air,
  Stone,
  Dirt,
  Grass,
  Sand,
}
impl Block {
  pub const fn descriptor(self) -> BlockDescriptor {
    match self {
      Self::Air => BlockDescriptor {
        name: "air",
        render: RenderType::None,
        collision: CollisionType::None,
        raycast_collision: false,
      },
      Self::Stone => BlockDescriptor { 
        name: "stone", 
        render: RenderType::SolidBlock(CubeTexture::all(BlockTexture::Stone)), 
        collision: CollisionType::Solid, 
        raycast_collision: true, 
      },
      Self::Dirt => BlockDescriptor { 
        name: "dirt", 
        render: RenderType::SolidBlock(CubeTexture::all(BlockTexture::Dirt)), 
        collision: CollisionType::Solid, 
        raycast_collision: true, 
      },
      Self::Grass => BlockDescriptor { 
        name: "grass", 
        render: RenderType::SolidBlock(CubeTexture::top_sides_bottom(
          BlockTexture::GrassTop, 
          BlockTexture::GrassSide, 
          BlockTexture::Dirt
        )), 
        collision: CollisionType::Solid, 
        raycast_collision: true, 
      },
      Self::Sand => BlockDescriptor { 
        name: "sand", 
        render: RenderType::SolidBlock(CubeTexture::all(BlockTexture::Sand)), 
        collision: CollisionType::Solid, 
        raycast_collision: true, 
      },
      _ => todo!()
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub struct BlockDescriptor {
  pub name: &'static str,
  pub render: RenderType,
  pub collision: CollisionType,
  pub raycast_collision: bool,
}
impl BlockDescriptor {
  pub fn of(block: Block) -> Self {
    block.descriptor()
  }
}

#[derive(Clone, Copy, Debug)]
pub struct CubeTexture {
  pub top: BlockTexture,
  pub bottom: BlockTexture,
  pub left: BlockTexture,
  pub right: BlockTexture,
  pub front: BlockTexture,
  pub back: BlockTexture,
}
impl CubeTexture {
  pub const fn top_sides_bottom(top: BlockTexture, sides: BlockTexture, bottom: BlockTexture) -> Self {
    Self {
      top,
      bottom,
      left: sides,
      right: sides,
      front: sides,
      back: sides,
    }
  }
  pub const fn horizontal_vertical(horizontal: BlockTexture, vertical: BlockTexture) -> Self {
    Self::top_sides_bottom(vertical, horizontal, vertical)
  }
  pub const fn all(texture: BlockTexture) -> Self {
    Self::horizontal_vertical(texture, texture)
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollisionType {
  None,
  Solid,
}

#[derive(Clone, Copy, Debug)]
pub enum RenderType {
  None,
  SolidBlock(CubeTexture)
}
