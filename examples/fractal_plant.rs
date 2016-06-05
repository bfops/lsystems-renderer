/// https://en.wikipedia.org/wiki/L-system#Example_7:_Fractal_plant

extern crate cgmath;
#[macro_use]
extern crate glium;
extern crate glutin;
extern crate lsystems;
extern crate rand;

mod support;

use support::prelude::*;
use lsystems::alphabet;
use lsystems::grammar;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TextureId {
  Stem,
}

impl rand::Rand for TextureId {
  fn rand<Rng: rand::Rng>(_: &mut Rng) -> Self {
    TextureId::Stem
  }
}

impl support::Texture for TextureId {
  fn to_fragment_shader(&self) -> String {
    match self {
      &TextureId::Stem => "
        #version 330

        in vec2 f_texture_posn;

        out vec4 frag_color;

        // http://amindforeverprogramming.blogspot.ca/2013/07/random-floats-in-glsl-330.html
        uint hash( uint x ) {
          x += ( x << 10u );
          x ^= ( x >>  6u );
          x += ( x <<  3u );
          x ^= ( x >> 11u );
          x += ( x << 15u );
          return x;
        }

        float random( float f ) {
          const uint mantissaMask = 0x007FFFFFu;
          const uint one          = 0x3F800000u;

          uint h = hash( floatBitsToUint( f ) );
          h &= mantissaMask;
          h |= one;

          float  r2 = uintBitsToFloat( h );
          return r2 - 1.0;
        }

        void main() {
          float f = random(f_texture_posn.x * 1337 + f_texture_posn.y);
          frag_color = vec4(mix(vec3(0.0, 0.4, 0.0), vec3(0.4, 0.6, 0.1), f), 1);
        }".to_string()
    }
  }
}

fn rotate(degrees: f32) -> alphabet::Transform {
  alphabet::Transform {
    rotation : std::f32::consts::PI * degrees / 180.0,
    scale    : Vector::new(1.0, 1.0),
  }
}

fn scale(s: f32) -> alphabet::Transform {
  alphabet::Transform {
    rotation : 0.0,
    scale    : Vector::new(s, s),
  }
}

fn new() -> grammar::T<TextureId> {
  let s       = grammar::Nonterminal(0);
  let s2      = grammar::Nonterminal(1);
  let l       = grammar::Nonterminal(2);
  let r       = grammar::Nonterminal(3);
  let recurse = grammar::Nonterminal(4);

  let rotate = |degrees| alphabet::Terminal::Transform(rotate(degrees));
  let scale = |s| alphabet::Terminal::Transform(scale(s));

  let add_branch = || {
    alphabet::Terminal::AddBranch {
      texture_id : TextureId::Stem,
      width      : 0.2,
      length     : 1.0,
    }
  };

  let rules =
    vec!(
      (vec!(add_branch())               , vec!(l, recurse, s2)),
      (vec!(add_branch(), add_branch()) , vec!(r, l)),
      (vec!(rotate( 25.0))              , vec!(recurse)),
      (vec!(rotate(-25.0))              , vec!(recurse)),
      (vec!(scale(0.5))                 , vec!(s)),
    );
  let rules =
    rules
    .into_iter()
    .map(|(actions, next)| grammar::RHS { actions: actions, next: next })
    .collect();

  grammar::T {
    rules: rules,
  }
}

pub fn main() {
  use cgmath::*;
  support::main(new())
}
