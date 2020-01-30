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

// mod testdb; // test database constructed from files at runtime
use oxigraph::model::*;
use oxigraph::{Repository, RepositoryConnection, MemoryRepository, Result};
use oxigraph::sparql::{PreparedQuery, QueryOptions};
use oxigraph::sparql::QueryResult;


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

    // TriGParser::new(contents.as_ref(), "").unwrap().parse_all(&mut |t| {
    //     info!("{}", t);
    //     debug!("subject: {}", t.subject);
    //     debug!("predicate: {}", t.predicate);
    //     debug!("object: {}", t.object);
    //     t.graph_name.expect("error: must supply a named graph");
    //     debug!("graph: {:?}", t.graph_name);
    //     Ok(()) as Result<(), TurtleError>
    // }).unwrap();
  }
  // println!("parsed {} statements", count);

  // parse a SPARQL query too
  // let raw_query = fs::read_to_string("examples/over21.sparql")
  //                     .expect("parse error :(");
  // match Query::parse(&raw_query, None) {
  //   Err(error) => panic!("Parse failure: {}", error),
  //   Ok(query) => {
  //     info!("{}", query.to_string());
  //   }
  // }

  // try making an in-memory oxigraph repository and querying it
  // based on:
  // https://github.com/Tpt/oxigraph/blob/master/lib/src/repository.rs
  // https://github.com/Tpt/oxigraph/blob/master/lib/tests/sparql_test_cases.rs
  let repository = MemoryRepository::default();
  let mut connection = repository.connection().unwrap();
  let ex = NamedNode::parse("http://example.com").unwrap();
  let quad = Quad::new(ex.clone(), ex.clone(), ex.clone(), None);
  connection.insert(&quad);
  let prepared_query = connection.prepare_query("SELECT ?s WHERE { ?s ?p ?o }", QueryOptions::default()).unwrap();
  let results = prepared_query.exec().unwrap();
  if let QueryResult::Bindings(results) = results {
  //   assert_eq!(results.into_values_iter().next().unwrap()?[0], Some(ex.into()));
    info!("results: {:?}", results.into_values_iter().next().unwrap().unwrap()[0]);
  };
}
