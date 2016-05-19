use std;

use prelude::*;
use language::*;

pub fn new(depth: u32) -> T {
  if depth == 0 {
    All(vec!())
  } else {
    All(vec!(
      Line(Point::new(0.0, 0.0), Point::new(0.0, 1.0)),
      WithTransforms(
        vec!(Translate(0.0, 1.0)),
        vec!(
          WithTransforms(
            vec!(Rotate( std::f32::consts::PI / 4.0), Scale(0.5)),
            vec!(new(depth - 1)),
          ),
          WithTransforms(
            vec!(Rotate(-std::f32::consts::PI / 4.0), Scale(0.5)),
            vec!(new(depth - 1)),
          ),
        ),
      ),
    ))
  }
}

