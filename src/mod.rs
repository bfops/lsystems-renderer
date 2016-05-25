extern crate cgmath;
#[macro_use]
extern crate glium;
#[macro_use]
extern crate log;
extern crate rand;
extern crate time;

mod mutate;
mod prelude;
mod render;

pub mod alphabet;
pub mod grammar;
pub mod vertex;
pub mod vertices;
pub mod word;

pub use mutate::mutate;
pub use render::render;
