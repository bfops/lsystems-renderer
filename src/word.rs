use alphabet::*;
use grammar;

pub struct Atom<Texture> {
  pub actions: Vec<Terminal<Texture>>,
  pub subword: T<Texture>,
}

pub type T<Texture> = Vec<Atom<Texture>>;

fn generate_inner<Texture: Clone>(
  t             : &grammar::T<Texture>,
  max_recursion : u32,
  nt            : grammar::Nonterminal,
) -> Atom<Texture>
{
  if max_recursion == 0 {
    return
      Atom {
        actions: vec!(),
        subword: vec!(),
      }
  }

  let rhs = &t.rules[nt.0 as usize];

  Atom {
    actions : rhs.actions.clone(),
    subword :
      rhs.next
      .iter()
      .map(|nt| generate_inner(t, max_recursion - 1, *nt))
      .collect(),
  }
}

pub fn generate<Texture: Clone>(t: &grammar::T<Texture>, max_recursion: u32) -> T<Texture> {
  vec!(generate_inner(t, max_recursion, grammar::Nonterminal(0)))
}
