//! This module implements languages/grammars/words with rules of the form
///   Nonterminal -> Terminal Nonterminal*
/// Every Nonterminal should appear on the left hand side of no more than one rule.

use cgmath;
use std;

use prelude::*;
use vertex;
use vertices;

pub fn translate(v: &Vector) -> Matrix {
  let mut transform: Matrix = cgmath::SquareMatrix::from_value(1.0);
  transform.z.x = v.x;
  transform.z.y = v.y;
  transform
}

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
pub enum Terminal<TextureId> {
  Transform(Transform),
  AddBranch {
    texture_id : TextureId,
    width      : f32,
    length     : f32,
  },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Nonterminal(pub u32);

#[derive(Debug, Clone)]
pub struct RHS<TextureId> {
  pub actions : Vec<Terminal<TextureId>>,
  pub next    : Vec<Nonterminal>,
}

#[derive(Debug, Clone)]
pub struct T<TextureId> {
  pub rules: std::collections::HashMap<Nonterminal, RHS<TextureId>>,
}

fn render_void<TextureId: Clone + Eq + std::hash::Hash>(
  t         : &T<TextureId>,
  depth     : u32,
  nt        : Nonterminal,
  transform : &Matrix,
  vertices  : &mut vertices::T<TextureId>,
) {
  if depth == 0 {
    return
  }

  let rhs =
    match t.rules.get(&nt) {
      None => return,
      Some(rhs) => rhs,
    };

  let mut transform = transform.clone();

  for action in &rhs.actions {
    match action {
      &Terminal::Transform(ref t) => {
        transform = transform * t.to_matrix();
      },
      &Terminal::AddBranch { ref texture_id, width, length } => {
        let drop_z = |p: cgmath::Vector3<f32>| { [ p.x, p.y ] };

        let x1 = width / 2.0;
        let x0 = -x1;
        let y0 = 0.0;
        let y1 = length;

        let v1 = transform * cgmath::Vector3::new(x0, y0, 1.0);
        let v2 = transform * cgmath::Vector3::new(x1, y0, 1.0);
        let v3 = transform * cgmath::Vector3::new(x1, y1, 1.0);
        let v4 = transform * cgmath::Vector3::new(x0, y1, 1.0);

        vertices.push(texture_id.clone(), vertex::T { screen_posn: drop_z(v2), texture_posn: [ 1.0, -1.0] });
        vertices.push(texture_id.clone(), vertex::T { screen_posn: drop_z(v1), texture_posn: [-1.0, -1.0] });
        vertices.push(texture_id.clone(), vertex::T { screen_posn: drop_z(v3), texture_posn: [ 1.0,  1.0] });

        vertices.push(texture_id.clone(), vertex::T { screen_posn: drop_z(v3), texture_posn: [ 1.0,  1.0] });
        vertices.push(texture_id.clone(), vertex::T { screen_posn: drop_z(v1), texture_posn: [-1.0, -1.0] });
        vertices.push(texture_id.clone(), vertex::T { screen_posn: drop_z(v4), texture_posn: [-1.0,  1.0] });

        transform = transform * translate(&Vector::new(0.0, length));
      },
    }
  }

  for nt in &rhs.next {
    render_void(t, depth - 1, *nt, &transform, vertices);
  }
}

pub fn render<TextureId: Clone + Eq + std::hash::Hash>(
  t         : &T<TextureId>,
  depth     : u32,
  transform : &Matrix,
) -> vertices::T<TextureId> {
  let mut vertices = vertices::new();
  render_void(t, depth, Nonterminal(0), transform, &mut vertices);
  vertices
}
