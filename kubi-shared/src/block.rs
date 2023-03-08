use serde::{Serialize, Deserialize};
use strum::EnumIter;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, EnumIter)]
#[repr(u8)]
pub enum BlockTexture {
  Stone,
  Dirt,
  GrassTop,
  GrassSide,
  Sand,
  Bedrock,
  Wood,
  WoodTop,
  Leaf,
  Torch,
  TallGrass,
  Snow,
  GrassSideSnow,
  Cobblestone,
  Planks,
  WaterSolid,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, EnumIter)]
#[repr(u8)]
pub enum Block {
  Air,
  Marker,
  Stone,
  Dirt,
  Grass,
  Sand,
  Cobblestone,
  TallGrass,
  Planks,
  Torch,
  Wood,
  Leaf,
  Water,
}

impl Block {
  #[inline]
  pub const fn descriptor(self) -> BlockDescriptor {
    match self {
      Self::Air => BlockDescriptor {
        name: "air",
        render: RenderType::None,
        collision: CollisionType::None,
        raycast_collision: false,
      },
      Self::Marker => BlockDescriptor {
        name: "marker",
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
      Self::Cobblestone => BlockDescriptor {
        name: "cobblestone",
        render: RenderType::SolidBlock(CubeTexture::all(BlockTexture::Cobblestone)),
        collision: CollisionType::Solid,
        raycast_collision: true,
      },
      Self::TallGrass => BlockDescriptor {
        name: "tall grass",
        render: RenderType::CrossShape(CrossTexture::all(BlockTexture::TallGrass)),
        collision: CollisionType::None,
        raycast_collision: true,
      },
      Self::Planks => BlockDescriptor { 
        name: "planks", 
        render: RenderType::SolidBlock(CubeTexture::all(BlockTexture::Planks)), 
        collision: CollisionType::Solid, 
        raycast_collision: true, 
      },
      Self::Torch => BlockDescriptor {
        name: "torch",
        render: RenderType::CrossShape(CrossTexture::all(BlockTexture::Torch)),
        collision: CollisionType::None,
        raycast_collision: true,
      },
      Self::Wood => BlockDescriptor {
        name: "leaf",
        render: RenderType::SolidBlock(CubeTexture::horizontal_vertical(BlockTexture::Wood, BlockTexture::WoodTop)),
        collision: CollisionType::Solid,
        raycast_collision: true,
      },
      Self::Leaf => BlockDescriptor {
        name: "leaf",
        render: RenderType::BinaryTransparency(CubeTexture::all(BlockTexture::Leaf)),
        collision: CollisionType::Solid,
        raycast_collision: true,
      },
      Self::Water => BlockDescriptor {
        name: "water",
        render: RenderType::BinaryTransparency(CubeTexture::all(BlockTexture::WaterSolid)),
        collision: CollisionType::None,
        raycast_collision: true,
      },
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
// impl BlockDescriptor {
//   pub fn of(block: Block) -> Self {
//     block.descriptor()
//   }
// }

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

#[derive(Clone, Copy, Debug)]
pub struct CrossTextureSides {
  pub front: BlockTexture,
  pub back: BlockTexture
}
impl CrossTextureSides {
  pub const fn all(texture: BlockTexture) -> Self {
    Self {
      front: texture,
      back: texture
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub struct CrossTexture(pub CrossTextureSides, pub CrossTextureSides);
impl CrossTexture {
  pub const fn all(texture: BlockTexture) -> Self {
    Self(
      CrossTextureSides::all(texture), 
      CrossTextureSides::all(texture)
    )
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
  SolidBlock(CubeTexture),
  BinaryTransparency(CubeTexture),
  CrossShape(CrossTexture),
}
