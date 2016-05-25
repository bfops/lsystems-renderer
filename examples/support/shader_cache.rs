use glium;
use std;

pub struct T<Id> {
  cache: std::collections::HashMap<Id, glium::Program>,
}

pub fn new<Id: Eq + std::hash::Hash>() -> T<Id> {
  T {
    cache: std::collections::HashMap::new(),
  }
}

impl<Id: ::support::Texture> T<Id> {
  pub fn get<Facade: glium::backend::Facade>(&mut self, window: &Facade, id: Id) -> &glium::Program {
    self.cache
      .entry(id.clone())
      .or_insert_with(|| {
        program!(
          window,
          330 => {
            vertex: "
              #version 330

              uniform mat4 transform;

              in vec2 screen_posn;
              in vec2 texture_posn;

              out vec2 f_texture_posn;

              void main() {
                f_texture_posn = texture_posn;
                gl_Position = transform * vec4(screen_posn, 0.0, 1.0);
              }
            ",

            fragment: &id.to_fragment_shader(),
          },
        ).unwrap()
      })
  }
}
