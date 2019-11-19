// extern crate anondb;
use sophia::graph::inmem::FastGraph;
use sophia::parser::nt;
use sophia::triple::stream::*;

use sophia::dataset::inmem::small::FastDataset;

static NT_DOC: &str = r#"
  <http://champin.net/#pa> <http://schema.org/name> "Pierre-Antoine Champin".
"#;

fn main() {
  // learning sophia
  let mut g = FastGraph::new();
  let inserted = nt::parse_str(NT_DOC).in_graph(&mut g);
  println!("inserted: {}", inserted.unwrap());

  let mut g2 = FastDataset::new();
  let inserted2 = nt::parse_str(NT_DOC).in_graph(&mut g2);
  println!("inserted2: {}", inserted2.unwrap());
  // println!("g: {}", g.);

  // let mut db = AnonDB::new();

  // TODO wait, do the signatures not really need to be stored after verification once?
  // TODO create a graph
  // TODO read gpg turtle file
  // TODO check signature first
  // TODO parse the ttl
  // TODO add it to the graph

  // println!("Hello world!");
}
