#[macro_use]
pub(crate) mod connection;
pub(crate) mod links;
pub(crate) mod open_graph;

use lazy_static::lazy_static;
use rbatis::rbatis::Rbatis;

lazy_static! {
  pub(crate) static ref RB : Rbatis = Rbatis::new();
}

