use cgmath;
use prelude::*;
use vertex;

#[derive(Debug, Clone, Copy)]
pub enum Transform {
  Translate(f32, f32),
  Rotate(f32),
  Scale(f32),
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
    }
  }
}

pub use self::Transform::*;

#[derive(Debug, Clone)]
pub enum Inner {
  WithTransforms(Vec<Transform>, T),
  Line(Point, Point),
}

pub use self::Inner::*;

pub type T = Vec<Inner>;

impl Inner {
  fn render(&self, transform: &Matrix, vertices: &mut Vec<vertex::T>) {
    match self {
      &WithTransforms(ref transforms, ref t) => {
        let mut transform = transform.clone();
        for t in transforms {
          transform = transform * t.to_matrix();
        }
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
    }
  }
}

pub fn render(t: &T, transform: &Matrix) -> Vec<vertex::T> {
  let mut vertices = Vec::new();
  for t in t {
    t.render(transform, &mut vertices);
  }
  vertices
}
