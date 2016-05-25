//! This module implements languages/grammars/words with rules of the form
///   Nonterminal -> Terminal Nonterminal*
/// Every Nonterminal should appear on the left hand side of no more than one rule.

use alphabet::*;

/// A Nonterminal is just a rule index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Nonterminal(pub u32);

#[derive(Debug, Clone)]
pub struct RHS<Texture> {
  pub actions : Vec<Terminal<Texture>>,
  pub next    : Vec<Nonterminal>,
}

#[derive(Debug, Clone)]
pub struct T<Texture> {
  pub rules: Vec<RHS<Texture>>,
}
