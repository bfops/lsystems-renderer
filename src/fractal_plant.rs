/// https://en.wikipedia.org/wiki/L-system#Example_7:_Fractal_plant

use std;

use prelude::*;
use language::*;

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

  let len = (1 << depth) as f32;
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

