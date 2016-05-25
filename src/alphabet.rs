use cgmath;

use prelude::*;

#[derive(Debug, Clone)]
pub struct Transform {
  pub rotation : f32,
  pub scale    : Vector,
}

impl Transform {
  pub fn to_matrix(&self) -> Matrix {
    let mut scale: Matrix = cgmath::SquareMatrix::from_value(1.0);
    scale.x.x = self.scale.x;
    scale.y.y = self.scale.y;

    let mut rotate: Matrix = cgmath::SquareMatrix::from_value(1.0);
    let (s, c) = self.rotation.sin_cos();
    rotate.x.x = c;
    rotate.x.y = s;
    rotate.y.x = -s;
    rotate.y.y = c;

    rotate * scale
  }
}

/// The terminals in this alphabet. These represent actions with possible side effects!
#[derive(Debug, Clone)]
pub enum Terminal<Texture> {
  Transform(Transform),
  AddBranch {
    texture_id : Texture,
    width      : f32,
    length     : f32,
  },
}
