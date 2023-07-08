macro_rules! include_shader_prefab {
  ($name: literal, $vert: literal, $frag: literal, $facade: expr) => {
    {
      use ::glium::{Program, program::ProgramCreationInput};
      log::info!("compiling shader {}", $name);
      Program::new(&*$facade, ProgramCreationInput::SourceCode {
        vertex_shader: include_str!($vert),
        fragment_shader: include_str!($frag),
        geometry_shader: None,
        tessellation_control_shader: None,
        tessellation_evaluation_shader: None,
        transform_feedback_varyings: None,
        outputs_srgb: false,
        uses_point_size: false,
      }).expect("Failed to compile gpu program")
    }
  };
}
pub(crate) use include_shader_prefab;
