use rand;
use std;

use alphabet::*;
use prelude::*;
use grammar::*;

const MAX_ANGLE_CHANGE: f32 = std::f32::consts::PI / 12.0;
const MAX_SCALE_CHANGE: f32 = 0.1;

fn len<Texture>(t: &T<Texture>) -> usize {
  let mut len = 0;
  for rhs in &t.rules {
    len += rhs.actions.len();
    len += rhs.next.len();
  }
  len
}

fn random_rescale<Rng: rand::Rng>(rng: &mut Rng) -> f32 {
  1.0 - MAX_SCALE_CHANGE + 2.0*MAX_SCALE_CHANGE*rng.next_f32()
}

fn random_rerotate<Rng: rand::Rng>(rng: &mut Rng) -> f32 {
  rng.next_f32() * MAX_ANGLE_CHANGE*2.0 - MAX_ANGLE_CHANGE
}

fn point_action<Texture, Rng: rand::Rng>(t: &mut Terminal<Texture>, rng: &mut Rng) {
  match t {
    &mut Terminal::Transform(ref mut t) => {
      let mut f = rng.next_f32();

      f -= 0.5;
      if f < 0.0 {
        t.rotation *= random_rerotate(rng);
        return
      }

      f -= 0.25;
      if f < 0.0 {
        t.scale.x *= random_rescale(rng);
        t.scale.y *= random_rescale(rng);
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

fn random_nonterminal<Texture, Rng: rand::Rng>(t: &T<Texture>, rng: &mut Rng) -> Nonterminal {
  let max_idx = len(t);
  let mut idx = rng.gen_range(0, max_idx);
  for (i, rhs) in t.rules.iter().enumerate() {
    let rule_len = rhs.actions.len() + rhs.next.len();
    if idx < rule_len {
      return Nonterminal(i as u32)
    }
    idx -= rule_len
  }

  panic!("This shouldn't happen");
}

fn point<Texture, Rng: rand::Rng>(t: &mut T<Texture>, rng: &mut Rng) {
  let max_idx = len(t);
  let mut idx = rng.gen_range(0, max_idx);
  let t_ptr = t as *const T<Texture>;

  for rhs in &mut t.rules {
    if idx < rhs.actions.len() {
      point_action(&mut rhs.actions[idx], rng);
      return
    }
    idx -= rhs.actions.len();

    if idx < rhs.next.len() {
      let t: &T<Texture> = unsafe { &*t_ptr };
      rhs.next[idx] = random_nonterminal(t, rng);
      return
    }
    idx -= rhs.next.len();
  }
}

fn random_action<Texture: rand::Rand, Rng: rand::Rng>(rng: &mut Rng) -> Terminal<Texture> {
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
        rotation : random_rerotate(rng),
        scale    : Vector::new(1.0, 1.0)
      })
  }

  Terminal::Transform(Transform {
    rotation : 0.0,
    scale    : Vector::new(random_rescale(rng), random_rescale(rng)),
  })
}

fn add<Texture: rand::Rand, Rng: rand::Rng>(t: &mut T<Texture>, rng: &mut Rng) {
  // We can insert before any symbol, or at the end of any chromosome.
  let max_idx = len(t) + t.rules.len();
  let mut idx = rng.gen_range(0, max_idx);
  let t_ptr = t as *const T<Texture>;

  for rhs in &mut t.rules {
    if idx < rhs.actions.len() + 1 {
      rhs.actions.insert(idx, random_action(rng));
      return
    }
    idx -= rhs.actions.len() + 1;

    if idx < rhs.next.len() + 1 {
      let t: &T<Texture> = unsafe { &*t_ptr };
      rhs.next.insert(idx, random_nonterminal(t, rng));
      return
    }
    idx -= rhs.next.len() + 1;
  }
}

fn remove<Texture, Rng: rand::Rng>(t: &mut T<Texture>, rng: &mut Rng) {
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

pub fn mutate<Texture: rand::Rand, Rng: rand::Rng>(t: &mut T<Texture>, rng: &mut Rng) {
  let mut f = rng.next_f32();

  f -= 0.1;
  if f <= 0.0 {
    add(t, rng);
    return
  }

  f -= 0.1;
  if f <= 0.0 {
    t.rules.push(RHS { actions: vec!(), next: vec!() });
    return
  }

  f -= 0.1;
  if f <= 0.0 {
    remove(t, rng);
    return
  }

  point(t, rng);
}
