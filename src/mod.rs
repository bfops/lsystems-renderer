extern crate cgmath;
#[macro_use]
extern crate glium;
extern crate glutin;
#[macro_use]
extern crate log;
extern crate rand;
extern crate time;

mod language;
mod main;
mod prelude;
mod tree;
mod vertex;

fn main() {
  main::main();
}
