#[macro_use]
extern crate glium;
extern crate glutin;
extern crate lsystem_renderer;

mod main;
mod prelude;

use prelude::*;
use lsystem_renderer::language::*;

pub fn new(depth: u32) -> T {
  if depth == 0 {
    All(vec!())
  } else {
    All(vec!(
      Line(Point::new(0.0, 0.0), Point::new(0.0, 1.0)),
      WithTransform(
        Translate(0.0, 1.0),
        vec!(
          WithTransform(
            Seq(vec!(Rotate( std::f32::consts::PI / 4.0), Scale(0.5))),
            vec!(new(depth - 1)),
          ),
          WithTransform(
            Seq(vec!(Rotate(-std::f32::consts::PI / 4.0), Scale(0.5))),
            vec!(new(depth - 1)),
          ),
        ),
      ),
    ))
  }
}

pub fn main() {
  let transform =
    Seq(vec!(
      Translate(0.0, -1.0),
    )).to_matrix();
  main::run(&transform, new(8))
}
