use cgmath;
use std;

use prelude::*;
use vertex;
use vertices;

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

pub type Many<TextureId> = Vec<T<TextureId>>;

#[derive(Debug, Clone)]
pub enum T<TextureId> {
  WithTransform(Transform, Many<TextureId>),
  DrawTexture {
    texture_id     : TextureId,
    texture_bounds : (Point, Vector),
    screen_bounds  : (Point, Vector),
  },
  All(Many<TextureId>),
}

pub use self::T::*;

impl<TextureId: Clone + Eq + std::hash::Hash> T<TextureId> {
  pub fn render(&self, transform: &Matrix, vertices: &mut vertices::T<TextureId>) {
    match self {
      &WithTransform(ref new_transform, ref t) => {
        let transform = transform * new_transform.to_matrix();
        for inner in t {
          inner.render(&transform, vertices);
        }
      },
      &DrawTexture { ref texture_id, texture_bounds: (tex_p, tex_l), screen_bounds: (screen_p, screen_l) } => {
        let drop_z = |p: cgmath::Vector3<f32>| { [ p.x, p.y ] };

        let v1 = transform * cgmath::Vector3::new(screen_p.x             , screen_p.y             , 1.0);
        let v2 = transform * cgmath::Vector3::new(screen_p.x + screen_l.x, screen_p.y             , 1.0);
        let v3 = transform * cgmath::Vector3::new(screen_p.x + screen_l.x, screen_p.y + screen_l.y, 1.0);
        let v4 = transform * cgmath::Vector3::new(screen_p.x             , screen_p.y + screen_l.y, 1.0);

        let t1 = [ tex_p.x          , tex_p.y ];
        let t2 = [ tex_p.x + tex_l.x, tex_p.y ];
        let t3 = [ tex_p.x + tex_l.x, tex_p.y + tex_l.y ];
        let t4 = [ tex_p.x          , tex_p.y + tex_l.y ];

        vertices.push(texture_id.clone(), vertex::T { screen_posn: drop_z(v2), texture_posn: t2 });
        vertices.push(texture_id.clone(), vertex::T { screen_posn: drop_z(v1), texture_posn: t1 });
        vertices.push(texture_id.clone(), vertex::T { screen_posn: drop_z(v3), texture_posn: t3 });

        vertices.push(texture_id.clone(), vertex::T { screen_posn: drop_z(v3), texture_posn: t3 });
        vertices.push(texture_id.clone(), vertex::T { screen_posn: drop_z(v1), texture_posn: t1 });
        vertices.push(texture_id.clone(), vertex::T { screen_posn: drop_z(v4), texture_posn: t4 });
      },
      &All(ref many) => {
        for t in many {
          t.render(transform, vertices);
        }
      },
    }
  }
}
