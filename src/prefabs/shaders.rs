

macro_rules! include_shader_prefab {
  ($name: literal, $vert: literal, $frag: literal, $geom: literal, $facade: expr) => {
    { 
      use ::glium::Program;
      log::info!("compiling shader {}", $name);
      Program::from_source(
        &*$facade,
        include_str!($vert),
        include_str!($frag),
        Some(include_str!($geom)),
      ).expect("Failed to compile gpu program")
    }
  };
  ($name: literal, $vert: literal, $frag: literal, $facade: expr) => {
    {
      use ::glium::Program;
      log::info!("compiling shader {}", $name);
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
