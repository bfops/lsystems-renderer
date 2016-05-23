#[macro_use]
extern crate glium;
extern crate glutin;
extern crate lsystem_renderer;

mod support;

use support::prelude::*;
use lsystem_renderer::language::*;

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
          float f = random(int(f_texture_posn.x * 100 + random(int(f_texture_posn.y * 10))));
          frag_color = vec4(mix(vec3(0.2, 0.1, 0.0), vec3(0.6, 0.4, 0.2), f), 1);
        }".to_string()
    }
  }
}

fn new(depth: u32) -> T<TextureId> {
  if depth == 0 {
    All(vec!())
  } else {
    All(vec!(
      DrawTexture {
        texture_id: TextureId::Wood,
        texture_bounds: (Point::new(-1.0, -1.0), Vector::new(2.0, 2.0)),
        screen_bounds: (Point::new(-0.1, 0.0), Vector::new(0.2, 1.0)),
      },
      WithTransform(
        Translate(0.0, 1.0),
        vec!(
          WithTransform(
            Seq(vec!(Rotate( std::f32::consts::PI / 4.0), Scale(0.5))),
            vec!(new(depth - 1)),
          ),
          WithTransform(
            Seq(vec!(Rotate(-std::f32::consts::PI / 4.0), Scale(0.5))),
            vec!(new(depth - 1)),
          ),
        ),
      ),
    ))
  }
}

pub fn main() {
  let transform =
    Seq(vec!(
      Translate(0.0, -1.0),
    )).to_matrix();
  support::main(&transform, new(8))
}
