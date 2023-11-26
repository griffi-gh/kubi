use std::num::NonZeroU8;
use num_enum::TryFromPrimitive;
use serde::{Serialize, Deserialize};
use strum::EnumIter;
use crate::block::Block;

#[derive(Clone, Copy)]
pub enum ItemUsage {
  AsBlock(Block)
}

pub struct ItemDescriptor {
  pub name: &'static str,
  pub usage: Option<ItemUsage>,
  pub stack_size: NonZeroU8,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug, EnumIter, TryFromPrimitive)]
#[repr(u8)]
pub enum Item {
  TestItem,
}

impl Item {
  #[inline]
  pub const fn descriptor(self) -> ItemDescriptor {
    match self {
      Self::TestItem => ItemDescriptor {
        name: "Test Item",
        usage: None,
        stack_size: nz::u8!(32),
      },
    }
  }
}

#[derive(Clone, Copy)]
pub struct ItemCollection(Option<(Item, NonZeroU8)>);

impl ItemCollection {
  pub const fn new(item: Item, amount: NonZeroU8) -> Self {
    Self(Some((item, amount)))
  }

  pub const fn new_single(item: Item) -> Self {
    Self(Some((item, nz::u8!(1))))
  }

  pub const fn new_empty() -> Self {
    Self(None)
  }

  pub const fn with_amount(&self, amount: NonZeroU8) -> Self {
    Self(match self.0 {
      Some((item, _)) => Some((item, amount)),
      None => None,
    })
  }

  /// Add items from another slot, copying them\
  /// Returns the leftover items
  pub fn add(&mut self, from: &Self) -> Self {
    let Some((item, count)) = from.0 else { return Self::new_empty() };
    todo!() //TODO finish item slot system
  }
}
