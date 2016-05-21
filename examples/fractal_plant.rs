/// https://en.wikipedia.org/wiki/L-system#Example_7:_Fractal_plant

#[macro_use]
extern crate glium;
extern crate glutin;
extern crate lsystem_renderer;

mod main;
mod prelude;

use prelude::*;
use lsystem_renderer::language::*;

fn line() -> T {
  Line(Point::new(0.0, 0.0), Point::new(0.0, 1.0))
}

fn forward() -> Transform {
  Translate(0.0, 1.0)
}

fn turn_left() -> Transform {
  Rotate( std::f32::consts::PI * 25.0 / 180.0)
}

fn turn_right() -> Transform {
  Rotate(-std::f32::consts::PI * 25.0 / 180.0)
}

fn x(depth: u32) -> T {
  if depth == 0 {
    return All(vec!())
  }

  let recurse = || {
    WithTransform(
      Scale(0.5),
      vec!(x(depth - 1)),
    )
  };

  All(vec!(
    line(),
    WithTransform(
      forward(),
      vec!(
        WithTransform(
          turn_left(),
          vec!(recurse()),
        ),
        recurse(),
        line(),
        WithTransform(
          forward(),
          vec!(
            WithTransform(
              turn_right(),
              vec!(
                line(),
                WithTransform(
                  forward(),
                  vec!(recurse()),
                ),
              ),
            ),
            WithTransform(
              turn_left(),
              vec!(recurse()),
            ),
          ),
        ),
      ),
    ),
  ))
}

pub fn new(depth: u32) -> T {
  x(depth)
}

pub fn main() {
  let transform =
    Seq(vec!(
      Translate(-0.8, -0.8),
      Scale(0.3),
      Rotate(-std::f32::consts::PI * 25.0 / 180.0),
    )).to_matrix();
  main::run(&transform, new(8))
}
