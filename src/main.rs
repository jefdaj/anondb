// extern crate rio_api;
// extern crate rio_turtle;
// extern crate rio_xml;
// extern crate sophia;

// fn main() {
//   println!("Hello world!");
// }

// TODO overall data structure?
// TODO wait, do the signatures not really need to be stored after verification once?
// TODO create a graph
// TODO read gpg turtle file
// TODO check signature first
// TODO parse the ttl
// TODO add it to the graph

// TODO report typo in trig documentation link (capitalize the G)

use rio_turtle::{TriGParser, TurtleError};
use rio_api::parser::QuadsParser;
// use rio_api::model::NamedNode;
use std::fs;

fn main() {
  // let file = b"<http://example.com/foo> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://schema.org/Person> <http://example.com/> .
  //   <http://example.com/foo> <http://schema.org/name> \"Foo\" <http://example.com/> .
  //   <http://example.com/bar> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://schema.org/Person> .
  //   <http://example.com/bar> <http://schema.org/name> \"Bar\" .";

  // let file2 = b"@prefix schema: <http://schema.org/> .
  // <http://example/> {
  //     <http://example.com/foo> a schema:Person ; schema:name  \"Foo\" .
  //     <http://example.com/bar> a schema:Person ; schema:name  \"Bar\" .
  // }";

  // let rdf_type = NamedNode { iri: "http://www.w3.org/1999/02/22-rdf-syntax-ns#type" };
  // let schema_person = NamedNode { iri: "http://schema.org/Person" };
  let mut count = 0;

  let filename = "example02.trig";
  println!("parsing file: {}", filename);
  let contents = fs::read_to_string(filename)
      .expect("Something went wrong reading the file");

  TriGParser::new(contents.as_ref(), "").unwrap().parse_all(&mut |t| {
      println!("quad: {}", t);
      // println!("\tsubject: {}", t.subject);
      // println!("\tpredicate: {}", t.predicate);
      // println!("\tobject: {}", t.object);
      // if t.predicate == rdf_type && t.object == schema_person.into() {
      count += 1;
      // }
      Ok(()) as Result<(), TurtleError>
  }).unwrap();
  println!("quads read: {}", count);
  // assert_eq!(2, count)
}
