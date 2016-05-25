use prelude::*;
use alphabet::*;
use grammar;

pub struct Atom<Texture> {
  pub actions: Vec<Terminal<Texture>>,
  pub subword: T<Texture>,
}

pub type T<Texture> = Vec<Atom<Texture>>;

fn generate_inner<Texture: Clone>(
  t              : &grammar::T<Texture>,
  nt             : grammar::Nonterminal,
  max_recursion  : u32,
  max_iterations : &mut u32,
  mut min_scale  : Vector,
) -> Atom<Texture>
{
  if max_recursion == 0 || *max_iterations == 0 || min_scale.x >= 1.0 || min_scale.y >= 1.0 {
    return
      Atom {
        actions: vec!(),
        subword: vec!(),
      }
  }

  *max_iterations -= 1;

  let rhs = &t.rules[nt.0 as usize];

  // Interesting trick: similar to how we decrease max_recursion by 1 on every recursion,
  // we also divide min_scale by any scale actions we see.
  // This is equivalent to accumulating the scales and checking if it's less than min_scale.
  for action in &rhs.actions {
    if let &Terminal::Transform(Transform { scale, .. }) = action {
      min_scale.x /= scale.x;
      min_scale.y /= scale.y;
    }
  }

  Atom {
    actions : rhs.actions.clone(),
    subword :
      rhs.next
      .iter()
      .map(|nt| generate_inner(t, *nt, max_recursion - 1, max_iterations, min_scale))
      .collect(),
  }
}

pub fn generate<Texture: Clone>(
  t: &grammar::T<Texture>,
  max_recursion: u32,
  mut max_iterations: u32,
  min_scale: f32,
) -> T<Texture> {
  vec!(generate_inner(t, grammar::Nonterminal(0), max_recursion, &mut max_iterations, Vector::new(min_scale, min_scale)))
}
