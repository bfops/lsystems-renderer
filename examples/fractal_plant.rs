/// https://en.wikipedia.org/wiki/L-system#Example_7:_Fractal_plant

#[macro_use]
extern crate glium;
extern crate glutin;
extern crate lsystem_renderer;

mod support;

use support::prelude::*;
use lsystem_renderer::language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TextureId {
  Wood,
}

impl support::Texture for TextureId {
  fn to_fragment_shader(&self) -> String {
    match self {
      &TextureId::Wood => "
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

fn rotate(degrees: f32) -> language::Transform {
  language::Transform {
    rotation : std::f32::consts::PI * degrees / 180.0,
    scale    : Vector::new(1.0, 1.0),
  }
}

fn scale(s: f32) -> language::Transform {
  language::Transform {
    rotation : 0.0,
    scale    : Vector::new(s, s),
  }
}

fn new() -> language::T<TextureId> {
  let s       = language::Nonterminal(0);
  let s2      = language::Nonterminal(1);
  let l       = language::Nonterminal(2);
  let r       = language::Nonterminal(3);
  let recurse = language::Nonterminal(4);

  let rotate = |degrees| language::Terminal::Transform(rotate(degrees));
  let scale = |s| language::Terminal::Transform(scale(s));

  let add_branch = || {
    language::Terminal::AddBranch {
      texture_id : TextureId::Wood,
      width      : 0.2,
      length     : 1.0,
    }
  };

  let rules =
    vec!(
      (s      , vec!(add_branch())              , vec!(l, recurse, s2)),
      (s2     , vec!(add_branch(), add_branch()), vec!(r, l)),
      (l      , vec!(rotate( 25.0))             , vec!(recurse)),
      (r      , vec!(rotate(-25.0))             , vec!(recurse)),
      (recurse, vec!(scale(0.5))                , vec!(s)),
    );
  let rules =
    std::iter::FromIterator::from_iter(
      rules
      .into_iter()
      .map(|(nt, actions, next)| (nt, language::RHS { actions: actions, next: next }))
    );

  language::T {
    rules: rules,
  }
}

pub fn main() {
  let transform =
    language::translate(&Vector::new(-0.8, -0.8)) *
    rotate(-25.0).to_matrix() *
    scale(0.3).to_matrix();
  support::main(&transform, new())
}
