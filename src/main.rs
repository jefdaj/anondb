// TODO parse any rdf format, either automatically or with a cli format flag
// 24 days of rust: http://zsiciarz.github.io/24daysofrust/index.html

// extern crate rio_api;
// extern crate rio_turtle;
// extern crate rio_xml;
// extern crate sophia;
// extern crate docopt;

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
use docopt::Docopt;

const USAGE: &'static str = "
Usage: anondb <rdf>...

Options:
";

fn main() {
  // Parse argv and exit the program with an error message if it fails.
  let args = Docopt::new(USAGE)
                     .and_then(|d| d.parse())
                     .unwrap_or_else(|e| e.exit());
  let rdfs = args.get_vec("<rdf>");
  println!("rdf files to parse: {:?}", rdfs);

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

  // example01.ttl works too, if you use TurtleParser
  let filename = "example02.trig";
  println!("parsing {}...", filename);
  let contents = fs::read_to_string(filename)
      .expect("Something went wrong reading the file");

  TriGParser::new(contents.as_ref(), "").unwrap().parse_all(&mut |t| {
  // TurtleParser::new(contents.as_ref(), "file:example01.ttl").unwrap().parse_all(&mut |t| {
      println!("statement: {}", t);
      // println!("\tgraph: {}", t.graph);
      // println!("\tsubject: {}", t.subject);
      // println!("\tpredicate: {}", t.predicate);
      // println!("\tobject: {}", t.object);
      // if t.predicate == rdf_type && t.object == schema_person.into() {
      count += 1;
      // }
      Ok(()) as Result<(), TurtleError>
  }).unwrap();
  println!("parsed {} statements", count);
  // assert_eq!(2, count)
}
