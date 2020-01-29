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
        println!("\t{}", t);
        println!("\t\ts: {}", t.subject);
        println!("\t\tp: {}", t.predicate);
        println!("\t\to: {}", t.object);
        t.graph_name.expect("error: must supply a named graph");
        println!("\t\tg: {:?}", t.graph_name);
        Ok(()) as Result<(), TurtleError>
    }).unwrap();
  }
  // println!("parsed {} statements", count);
}
