use cgmath;
use std;

use prelude::*;
use alphabet::*;
use vertices;
use word;
use vertex;

fn translate(v: &Vector) -> Matrix {
  let mut transform: Matrix = cgmath::SquareMatrix::from_value(1.0);
  transform.z.x = v.x;
  transform.z.y = v.y;
  transform
}

pub fn render<Texture: Clone + Eq + std::hash::Hash>(word: &word::T<Texture>) -> vertices::T<Texture> {
  let mut vertices = vertices::new();
  let transform = cgmath::SquareMatrix::from_value(1.0);
  render_inner(word, &transform, &mut vertices);
  vertices
}

fn render_inner<Texture: Clone + Eq + std::hash::Hash>(
  word      : &word::T<Texture>,
  transform : &Matrix,
  vertices  : &mut vertices::T<Texture>,
) {
  for atom in word {
    let mut transform = transform.clone();

    for action in &atom.actions {
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

    render_inner(&atom.subword, &transform, vertices);
  }
}
