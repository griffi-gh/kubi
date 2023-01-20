use glium::{Program, backend::Facade};

macro_rules! include_shader_prefab {
  ($vert: literal, $frag: literal, $geom: literal, $facade: expr) => {
    { 
      log::info!("↓↓↓ compiling shader prefab ↓↓↓");
      log::info!("{} {} {}", $vert, $frag, $geom);
      Program::from_source(
        &*$facade,
        include_str!($vert),
        include_str!($frag),
        Some(include_str!($geom)),
      ).expect("Failed to compile gpu program")
    }
  };
  ($vert: literal, $frag: literal, $facade: expr) => {
    {
      log::info!("↓↓↓ compiling shader prefab ↓↓↓");
      log::info!("{} {}", $vert, $frag);
      Program::from_source(
        &*$facade,
        include_str!($vert),
        include_str!($frag),
        None,
      ).expect("Failed to compile gpu program")
    }
  };
}
pub(crate) use include_shader_prefab;
