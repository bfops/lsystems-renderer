extern crate rand;

pub mod prelude;

use glium;
use glutin;
use lsystems;
use lsystems::grammar;
use std;

use self::prelude::*;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;

pub trait Texture : Clone + Eq + std::hash::Hash + rand::Rand {
  fn to_fragment_shader(&self) -> String;
}

pub fn main<TextureId: Texture>(transform: &Matrix, mut t: grammar::T<TextureId>) {
  use glium::DisplayBuild;

  let window =
    glutin::WindowBuilder::new()
    .with_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
    .build_glium()
    .unwrap();

  let mut rng: rand::XorShiftRng = rand::SeedableRng::from_seed([0x12345678, 0x9abcdef0, 0x13371337, 0x98765432]);

  let mut prev = t.clone();

  loop {
    let draw_parameters =
      glium::DrawParameters {
        depth:
          glium::Depth {
            test: glium::DepthTest::Overwrite,
            write: false,
            .. Default::default()
          },
        blend:
          glium::Blend {
            color: glium::BlendingFunction::AlwaysReplace,
            alpha: glium::BlendingFunction::AlwaysReplace,
            constant_value: (0.0, 0.0, 0.0, 0.0),
          },
        .. Default::default()
      };

    let mut target = window.draw();
    glium::Surface::clear(&mut target, None, Some((1.0, 1.0, 1.0, 1.0)), false, None, None);

    let vertices = grammar::render(&t, 8, &transform).to_hashmap();

    for (texture_id, vertices) in &vertices {
      let vertex_buffer = glium::VertexBuffer::new(&window, &vertices).unwrap();

      let program =
        program!(
          &window,
          330 => {
            vertex: "
              #version 330

              uniform mat4 transform;

              in vec2 screen_posn;
              in vec2 texture_posn;

              out vec2 f_texture_posn;

              void main() {
                f_texture_posn = texture_posn;
                gl_Position = transform * vec4(screen_posn, 0.0, 1.0);
              }
            ",

            fragment: &texture_id.to_fragment_shader(),
          },
        ).unwrap();

      // building the uniforms
      let uniforms = uniform! {
        transform: [
          [1.0, 0.0, 0.0, 0.0],
          [0.0, 1.0, 0.0, 0.0],
          [0.0, 0.0, 1.0, 0.0],
          [0.0, 0.0, 0.0, 1.0f32]
        ],
      };

      glium::Surface::draw(
        &mut target,
        &vertex_buffer,
        glium::index::IndicesSource::NoIndices { primitives: glium::index::PrimitiveType::TrianglesList },
        &program,
        &uniforms,
        &draw_parameters,
      ).unwrap();
    }

    target.finish().unwrap();

    for event in window.poll_events() {
      match event {
        glutin::Event::Closed => return,
        glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(keycode)) => {
          use glutin::VirtualKeyCode::*;
          match keycode {
            Down => {
              t = prev.clone();
              lsystems::mutate(&mut t, &mut rng);
            },
            Up => {
              prev = t.clone();
              lsystems::mutate(&mut t, &mut rng);
            },
            _ => {},
          }
        },
        _ => {},
      }
    }
  }
}
