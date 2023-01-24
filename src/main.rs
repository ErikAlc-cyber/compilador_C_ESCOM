use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

use std::fs;
use std::io::Read;        
use std::fs::File;     

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct CLanguageParser;

/// It reads a file and parses it.
fn main() {
    
    let doc_entrada = std::env::args().nth(1).expect("Especifique una archivo porfavor");
    let mut cont = String::new();
    let mut arch = File::open(doc_entrada).unwrap();
    let mut nmb: String = String::new();
    arch.read_to_string(&mut cont).ok();
    cont = cont.replace('\n', " ");
 
    println!("Doc de entrada:\n{:?}\n\n",cont.as_str());

    let pairs = CLanguageParser::parse(Rule::program, cont.as_str()).unwrap_or_else(|e| panic!("{}", e));
    for pair in pairs {
        println!("Regla: {:?}", pair.as_rule());
        println!("Entrada: {:?}", pair.as_str());
        println!("Respuesta: {:?}", pair.clone().into_inner().collect::<Vec<_>>());
    }
}
