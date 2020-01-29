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
  let args = Docopt::new(USAGE)
                     .and_then(|d| d.parse())
                     .unwrap_or_else(|e| e.exit());
  let rdfs = args.get_vec("<rdf>");
  println!("rdf files to parse: {:?}", rdfs);

  // let mut count = 0;

  for rdf in rdfs {
    println!("parsing {:?}...", rdf);
    let contents = fs::read_to_string(rdf)
        .expect("parse error :(");

    TriGParser::new(contents.as_ref(), "").unwrap().parse_all(&mut |t| {
        println!("statement: {}", t);
        println!("\tsubject: {}", t.subject);
        println!("\tpredicate: {}", t.predicate);
        println!("\tobject: {}", t.object);
        println!("\tgraph: {:?}", t.graph_name);
        // if t.predicate == rdf_type && t.object == schema_person.into() {
        // count += 1;
        Ok(()) as Result<(), TurtleError>
    }).unwrap();
  }
  // println!("parsed {} statements", count);
}
