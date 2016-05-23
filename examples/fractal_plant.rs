/// https://en.wikipedia.org/wiki/L-system#Example_7:_Fractal_plant

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
          float f = random(f_texture_posn.x * 1337 + f_texture_posn.y);
          frag_color = vec4(mix(vec3(0.0, 0.4, 0.0), vec3(0.4, 0.6, 0.1), f), 1);
        }".to_string()
    }
  }
}

fn branch() -> T<TextureId> {
  DrawTexture {
    texture_id: TextureId::Wood,
    texture_bounds: (Point::new(-1.0, -1.0), Vector::new(2.0, 2.0)),
    screen_bounds: (Point::new(-0.1, 0.0), Vector::new(0.2, 1.0)),
  }
}

fn forward() -> Transform {
  Translate(0.0, 1.0)
}

fn turn_left() -> Transform {
  Rotate( std::f32::consts::PI * 25.0 / 180.0)
}

fn turn_right() -> Transform {
  Rotate(-std::f32::consts::PI * 25.0 / 180.0)
}

fn x(depth: u32) -> T<TextureId> {
  if depth == 0 {
    return All(vec!())
  }

  let recurse = || {
    WithTransform(
      Scale(0.5),
      vec!(x(depth - 1)),
    )
  };

  All(vec!(
    branch(),
    WithTransform(
      forward(),
      vec!(
        WithTransform(
          turn_left(),
          vec!(recurse()),
        ),
        recurse(),
        branch(),
        WithTransform(
          forward(),
          vec!(
            WithTransform(
              turn_right(),
              vec!(
                branch(),
                WithTransform(
                  forward(),
                  vec!(recurse()),
                ),
              ),
            ),
            WithTransform(
              turn_left(),
              vec!(recurse()),
            ),
          ),
        ),
      ),
    ),
  ))
}

fn new(depth: u32) -> T<TextureId> {
  x(depth)
}

pub fn main() {
  let transform =
    Seq(vec!(
      Translate(-0.8, -0.8),
      Scale(0.3),
      Rotate(-std::f32::consts::PI * 25.0 / 180.0),
    )).to_matrix();
  support::main(&transform, new(8))
}
