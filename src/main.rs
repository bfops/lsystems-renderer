use glium;
use glutin;
use rand;
use std;

use fractal_plant;
use language;

pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 800;

pub fn main() {
  use glium::DisplayBuild;

  let window =
    glutin::WindowBuilder::new()
    .with_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
    .build_glium()
    .unwrap();

  let mut rng: rand::XorShiftRng = rand::SeedableRng::from_seed([0x12345678, 0x9abcdef0, 0x13371337, 0x98765432]);

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

    let mut vertices = Vec::new();
    let transform =
      language::Seq(vec!(
        language::Translate(-0.8, -0.8),
        language::Scale(0.3),
        language::Rotate(-std::f32::consts::PI * 25.0 / 180.0),
      )).to_matrix();
    fractal_plant::new(8).render(&transform, &mut vertices);
    let vertex_buffer = glium::VertexBuffer::new(&window, &vertices).unwrap();

    let program =
      program!(
        &window,
        330 => {
          vertex: "
            #version 330

            uniform mat4 matrix;

            in vec2 position;

            void main() {
              gl_Position = matrix * vec4(position, 0.0, 1.0);
            }
          ",

          fragment: "
            #version 330

            out vec4 f_color;

            void main() {
              f_color = vec4(0.0, 0.0, 0.0, 1.0);
            }
          "
        },
      ).unwrap();

    // building the uniforms
    let uniforms = uniform! {
      matrix: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32]
      ],
    };

    glium::Surface::draw(
      &mut target,
      &vertex_buffer,
      glium::index::IndicesSource::NoIndices { primitives: glium::index::PrimitiveType::LinesList },
      &program,
      &uniforms,
      &draw_parameters,
    ).unwrap();

    target.finish().unwrap();

    for event in window.poll_events() {
      match event {
        glutin::Event::Closed => return,
        _ => {},
      }
    }
  }
}
