use std;

use vertex;

pub struct T<TextureId> {
  // vertices by texture id
  vertices: std::collections::HashMap<TextureId, Vec<vertex::T>>,
}

pub fn new<TextureId: Eq + std::hash::Hash>() -> T<TextureId> {
  T {
    vertices: std::collections::HashMap::new(),
  }
}

impl<TextureId: Eq + std::hash::Hash> T<TextureId> {
  pub fn push(&mut self, texture_id: TextureId, vertex: vertex::T) {
    self.vertices.entry(texture_id).or_insert(vec!()).push(vertex);
  }

  pub fn to_hashmap(self) -> std::collections::HashMap<TextureId, Vec<vertex::T>> {
    self.vertices
  }
}
