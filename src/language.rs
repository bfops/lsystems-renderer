use cgmath;
use prelude::*;
use vertex;

#[derive(Debug, Clone)]
pub enum Transform {
  Translate(f32, f32),
  Rotate(f32),
  Scale(f32),
  Seq(Vec<Transform>),
}

impl Transform {
  pub fn to_matrix(&self) -> Matrix {
    match self {
      &Translate(x, y) => {
        let mut transform: Matrix = cgmath::SquareMatrix::from_value(1.0);
        transform.z.x = x;
        transform.z.y = y;
        transform
      },
      &Rotate(angle) => {
        let mut transform: Matrix = cgmath::SquareMatrix::from_value(1.0);
        let (s, c) = angle.sin_cos();
        transform.x.x = c;
        transform.x.y = s;
        transform.y.x = -s;
        transform.y.y = c;
        transform
      },
      &Scale(s) => {
        let mut transform: Matrix = cgmath::SquareMatrix::from_value(s);
        transform.z.z = 1.0;
        transform
      },
      &Seq(ref ts) => {
        let mut transform = cgmath::SquareMatrix::from_value(1.0);
        for t in ts {
          transform = transform * t.to_matrix();
        }
        transform
      },
    }
  }
}

pub use self::Transform::*;

pub type Many = Vec<T>;

#[derive(Debug, Clone)]
pub enum T {
  WithTransform(Transform, Many),
  Line(Point, Point),
  All(Many),
}

pub use self::T::*;

impl T {
  pub fn render(&self, transform: &Matrix, vertices: &mut Vec<vertex::T>) {
    match self {
      &WithTransform(ref new_transform, ref t) => {
        let transform = transform * new_transform.to_matrix();
        for inner in t {
          inner.render(&transform, vertices);
        }
      },
      &Line(ref p1, ref p2) => {
        let p1 = transform * cgmath::Vector3::new(p1.x, p1.y, 1.0);
        let p2 = transform * cgmath::Vector3::new(p2.x, p2.y, 1.0);
        vertices.push(vertex::T { position: [p1.x, p1.y] });
        vertices.push(vertex::T { position: [p2.x, p2.y] });
      },
      &All(ref many) => {
        for t in many {
          t.render(transform, vertices);
        }
      },
    }
  }
}
