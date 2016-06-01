use cgmath;
use cgmath::{InnerSpace, SquareMatrix};

use prelude::*;
use alphabet::*;
use grammar;

pub struct Atom<Texture> {
  pub actions: Vec<Terminal<Texture>>,
  pub subword: T<Texture>,
}

fn empty<Texture>() -> Atom<Texture> {
  Atom {
    actions: vec!(),
    subword: vec!(),
  }
}

pub type T<Texture> = Vec<Atom<Texture>>;

fn generate_inner<Texture: Clone>(
  t                 : &grammar::T<Texture>,
  nt                : grammar::Nonterminal,
  max_recursion     : u32,
  max_iterations    : &mut u32,
  min_scale         : f32,
  mut transform     : Matrix,
  mut available_nutrients : f32,
  mut max_mass      : f32,
) -> (f32, f32, Atom<Texture>)
{
  let mut used_nutrients = 0.0;
  let mut mass = 0.0;

  if max_recursion == 0 || *max_iterations == 0 {
    return (0.0, 0.0, empty())
  }

  *max_iterations -= 1;

  let rhs = &t.rules[nt.0 as usize];

  // Interesting trick: similar to how we decrease max_recursion by 1 on every recursion,
  // we also divide min_scale by any scale actions we see.
  // This is equivalent to accumulating the scales and checking if it's less than min_scale.
  for action in &rhs.actions {
    match action {
      &Terminal::Transform(ref next_transform) => {
        transform = transform * next_transform.to_matrix();
      },
      &Terminal::AddBranch { mut width, mut length, .. } => {
        let x_scale = (transform * cgmath::Vector3::new(1.0, 0.0, 0.0)).magnitude();
        let y_scale = (transform * cgmath::Vector3::new(0.0, 1.0, 0.0)).magnitude();
        if x_scale <= min_scale || y_scale < min_scale {
          return (0.0, 0.0, empty())
        }
        width  *= x_scale;
        length *= y_scale;

        // the deductions are proportional to volume/surface area for 3D branches

        let nutrients_here = length * width * 0.001;
        used_nutrients += nutrients_here;
        available_nutrients = available_nutrients.min(width * width) - nutrients_here;
        if available_nutrients < 0.0 {
          return (0.0, 0.0, empty())
        }

        let mass_here = width * width * length * 0.02;
        mass += mass_here;
        max_mass = max_mass.min(width * width) - mass_here;
        if max_mass < 0.0 {
          return (0.0, 0.0, empty())
        }
      },
    }
  }

  let t =
    Atom {
      actions : rhs.actions.clone(),
      subword :
        rhs.next
        .iter()
        .map(|nt| {
          let (child_nutrients, child_mass, child) =
            generate_inner(
              t,
              *nt,
              max_recursion - 1,
              max_iterations,
              min_scale,
              transform,
              available_nutrients,
              max_mass,
            );
          if child_nutrients > available_nutrients || child_mass > max_mass {
            empty()
          } else {
            available_nutrients -= child_nutrients;
            max_mass -= child_mass;
            used_nutrients += child_nutrients;
            mass += child_mass;
            child
          }
        })
        .collect(),
    };
  (used_nutrients, mass, t)
}

pub fn generate<Texture: Clone>(
  t                   : &grammar::T<Texture>,
  max_recursion       : u32,
  mut max_iterations  : u32,
  min_scale           : f32,
  available_nutrients : f32,
  max_mass            : f32,
) -> T<Texture> {
  let transform = Matrix::from_value(1.0);
  vec!(
    generate_inner(
      t,
      grammar::Nonterminal(0),
      max_recursion,
      &mut
      max_iterations,
      min_scale,
      transform,
      available_nutrients,
      max_mass,
    ).2
  )
}
