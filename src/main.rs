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

#[macro_use] extern crate log;
extern crate simplelog;

use simplelog::*;
use std::fs::File;

use oxigraph::sparql::*;

const USAGE: &'static str = "
Usage: anondb <rdf>...

Options:
";

fn main() {
  CombinedLogger::init(
    vec![
      TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed).unwrap(),
      WriteLogger::new(LevelFilter::Debug, Config::default(), File::create("anondb.log").unwrap()),
    ]
  ).unwrap();

  let args = Docopt::new(USAGE)
                     .and_then(|d| d.parse())
                     .unwrap_or_else(|e| e.exit());
  let rdfs = args.get_vec("<rdf>");
  info!("rdf files to parse: {:?}", rdfs);

  // let mut count = 0;

  for rdf in rdfs {
    info!("parsing {:?}...", rdf);
    let contents = fs::read_to_string(rdf)
        .expect("parse error :(");

    TriGParser::new(contents.as_ref(), "").unwrap().parse_all(&mut |t| {
        info!("{}", t);
        debug!("subject: {}", t.subject);
        debug!("predicate: {}", t.predicate);
        debug!("object: {}", t.object);
        t.graph_name.expect("error: must supply a named graph");
        debug!("graph: {:?}", t.graph_name);
        Ok(()) as Result<(), TurtleError>
    }).unwrap();
  }
  // println!("parsed {} statements", count);

  // parse a SPARQL query too
  let raw_query = "\n\
    PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>\n\
    PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>\n\
    PREFIX dmv: <http://www.example.org/dmv#>\n\
    PREFIX : <http://www.example.org/exampleDocument#>\n\
    SELECT *\n\
    WHERE {\n\
      ?anon dmv:name \"Monica Murphy\" .\n\
      ?anon dmv:over21 true .\n\
    }";
  match Query::parse(&raw_query, None) {
    Err(error) => panic!("Parse failure: {}", error),
    Ok(query) => {
      info!("{}", query.to_string());
    }
  }
}
