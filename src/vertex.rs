#[derive(Copy, Clone)]
pub struct T {
  pub screen_posn  : [f32; 2],
  pub texture_posn : [f32; 2],
}

implement_vertex!(T, screen_posn, texture_posn);

