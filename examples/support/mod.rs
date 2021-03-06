pub mod prelude;
mod shader_cache;

use cgmath;
use glium;
use glutin;
use lsystems;
use lsystems::grammar;
use lsystems::word;
use rand;
use std;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;

pub trait Texture : Clone + Eq + std::hash::Hash + rand::Rand {
  fn to_fragment_shader(&self) -> String;
}

pub fn main<Texture: self::Texture>(mut t: grammar::T<Texture>) {
  use glium::DisplayBuild;

  let window =
    glutin::WindowBuilder::new()
    .with_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
    .build_glium()
    .unwrap();

  let mut rng: rand::XorShiftRng = rand::SeedableRng::from_seed([0x12345678, 0x9abcdef0, 0x13371337, 0x98765432]);

  let mut prev = t.clone();

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

  let mut shader_cache = shader_cache::new();

  loop {
    let mut target = window.draw();
    glium::Surface::clear(&mut target, None, Some((1.0, 1.0, 1.0, 1.0)), false, None, None);

    let word = word::generate(&t, 1 << 6, 1 << 18, 0.01, 1000000.0, 1000000.0);
    let vertices = lsystems::render(&word).to_hashmap();

    let mut min_x = std::f32::INFINITY;
    let mut min_y = std::f32::INFINITY;
    let mut max_x = std::f32::NEG_INFINITY;
    let mut max_y = std::f32::NEG_INFINITY;
    for (_, vertices) in &vertices {
      for vertex in vertices {
        let x = vertex.screen_posn[0];
        let y = vertex.screen_posn[1];
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
      }
    }

    for (texture_id, vertices) in &vertices {
      let vertex_buffer = glium::VertexBuffer::new(&window, &vertices).unwrap();

      let program = shader_cache.get(&window, texture_id.clone());

      let transform =
        cgmath::Matrix4::from_nonuniform_scale(2.0 / (max_x - min_x), 2.0 / (max_y - min_y), 1.0) *
        cgmath::Matrix4::from_translation(cgmath::Vector3::new(-(min_x + max_x) / 2.0, -(min_y + max_y) / 2.0, 0.0));

      let uniforms = uniform! {
        transform: cgmath::conv::array4x4(transform),
      };

      glium::Surface::draw(
        &mut target,
        &vertex_buffer,
        glium::index::IndicesSource::NoIndices { primitives: glium::index::PrimitiveType::TrianglesList },
        program,
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
