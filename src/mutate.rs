use rand;
use std;

use prelude::*;
use grammar::*;

fn len<TextureId>(t: &T<TextureId>) -> usize {
  let mut len = 0;
  for rhs in &t.rules {
    len += rhs.actions.len();
    len += rhs.next.len();
  }
  len
}

fn point_action<TextureId, Rng: rand::Rng>(t: &mut Terminal<TextureId>, rng: &mut Rng) {
  match t {
    &mut Terminal::Transform(ref mut t) => {
      let mut f = rng.next_f32();

      f -= 0.5;
      if f < 0.0 {
        t.rotation = rng.next_f32() * 2.0 * std::f32::consts::PI;
        return
      }

      f -= 0.25;
      if f < 0.0 {
        t.scale.x = rng.next_f32().sqrt();
        t.scale.y = rng.next_f32().sqrt();
        return
      }
    },
    &mut Terminal::AddBranch { ref mut width, ref mut length, .. } => {
      if rng.next_f32() < 0.5 {
        *width += rng.next_f32();
        return
      }

      *length += rng.next_f32();
    },
  }
}

fn point<TextureId, Rng: rand::Rng>(t: &mut T<TextureId>, rng: &mut Rng) {
  let max_idx = len(t);
  let len = t.rules.len();
  let mut idx = rng.gen_range(0, max_idx);
  for rhs in &mut t.rules {
    if idx < rhs.actions.len() {
      point_action(&mut rhs.actions[idx], rng);
      return
    }
    idx -= rhs.actions.len();

    if idx < rhs.next.len() {
      rhs.next[idx] = Nonterminal(rng.gen_range(0, len) as u32);
      return
    }
    idx -= rhs.next.len();
  }
}

fn random_action<TextureId: rand::Rand, Rng: rand::Rng>(rng: &mut Rng) -> Terminal<TextureId> {
  let mut f = rng.next_f32();

  f -= 0.5;
  if f < 0.0 {
    return
      Terminal::AddBranch {
        texture_id : rand::Rand::rand(rng),
        width      : rng.next_f32() * 0.2 + 0.1,
        length     : rng.next_f32(),
      }
  }

  f -= 0.25;
  if f < 0.0 {
    return
      Terminal::Transform(Transform {
        rotation: rng.next_f32() * 2.0 * std::f32::consts::PI,
        scale: Vector::new(1.0, 1.0)
      })
  }

  Terminal::Transform(Transform {
    rotation: 0.0,
    scale: Vector::new(rng.next_f32().sqrt(), rng.next_f32().sqrt()),
  })
}

fn add<TextureId: rand::Rand, Rng: rand::Rng>(t: &mut T<TextureId>, rng: &mut Rng) {
  let max_idx = len(t) + t.rules.len() + 2;
  let len = t.rules.len();
  let mut idx = rng.gen_range(0, max_idx);

  if idx >= max_idx - 2 {
    t.rules.push(RHS { actions: vec!(), next: vec!() });
  }

  for rhs in &mut t.rules {
    if idx < rhs.actions.len() + 1 {
      rhs.actions.insert(idx, random_action(rng));
      return
    }
    idx -= rhs.actions.len() + 1;

    if idx < rhs.next.len() + 1 {
      rhs.next.insert(idx, Nonterminal(rng.gen_range(0, len) as u32));
      return
    }
    idx -= rhs.next.len() + 1;
  }
}

fn remove<TextureId, Rng: rand::Rng>(t: &mut T<TextureId>, rng: &mut Rng) {
  let max_idx = len(t);
  let mut idx = rng.gen_range(0, max_idx);

  for rhs in &mut t.rules {
    if idx < rhs.actions.len() {
      rhs.actions.remove(idx);
      return
    }
    idx -= rhs.actions.len();

    if idx < rhs.next.len() {
      rhs.next.remove(idx);
      return
    }
    idx -= rhs.next.len();
  }
}

pub fn mutate<TextureId: rand::Rand, Rng: rand::Rng>(t: &mut T<TextureId>, rng: &mut Rng) {
  let mut f = rng.next_f32();

  f -= 0.1;
  if f <= 0.0 {
    add(t, rng);
    return
  }

  f -= 0.1;
  if f <= 0.0 {
    remove(t, rng);
    return
  }

  point(t, rng);
}
