use std::{fs::File, io::{self, BufRead, Write}};
use binarytools::binary_utils::parser::symbol_table::{SymbolTable, SymbolTableEntry, SymbolType};
extern crate clap;
use clap::{App, Arg};
extern crate atty;

fn main() {
    // Read piped input
    let mut lines: Vec<String> = Vec::new();
    if !atty::is(atty::Stream::Stdin) {        
        let stdin = io::stdin();        
        for line in stdin.lock().lines() {
            lines.push(line.unwrap());
        }
    }    

    // Build up the CLI parsing arguments
    let mut app = App::new("Binary Disassembly Parser")
        .version("1.0.0")
        .author("Graham Riches")
        .about("Parses disassembled application binaries and outputs some formatted data")
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .value_name("input")
            .help("The disassembled binary file to parse")
            .required(false))
        .arg(Arg::with_name("filter")
            .short("f")
            .long("filter")
            .value_name("filter")
            .help("Filter by. Options: objects, functions, files")
            .required(false))
        .arg(Arg::with_name("filter-by-section")
            .short("s")
            .long("filter-by-section")
            .value_name("filter_section")
            .help("Filter by section (e.g., .text, .bss, etc.)")
            .required(false))        
        .arg(Arg::with_name("html")
            .short("o")
            .long("html")
            .value_name("html")
            .help("Write formatted HTML output to a file: --html=filename.html")
            .required(false));            

    let matches = app.clone().get_matches();

    // Check if we already read the input from a pipe, otherwise get it from a file
    let symbol_table = if lines.len() > 0 {
        SymbolTable::from_lines(&lines)
    } else {
        match matches.value_of("input") {
            Some(file) => SymbolTable::from_file(file).expect("Could not parse input file as a symbol table"),
            None => {
                eprintln!("ERROR: missing input file");
                app.print_long_help().unwrap();                
                std::process::exit(1);
            }
        }
    };

    // Filter the symbol table based on the filter option
    let filter: Option<fn(&SymbolTableEntry) -> bool> = match matches.value_of("filter") {
        Some("objects")   => Some(|x| x.flags.symbol_type == SymbolType::Object),
        Some("functions") => Some(|x| x.flags.symbol_type == SymbolType::Function),
        Some("files")     => Some(|x| x.flags.symbol_type == SymbolType::File),
        Some(_)           => None,
        None              => None      
    };

    let mut filtered = SymbolTable::new();
    if let Some(f) = filter {
        // Get the symbol table, filter it, sort it and print it out
        filtered = symbol_table.into_iter()
            .filter(f)
            .collect::<SymbolTable>();                
    } else if let Some(section) = matches.value_of("filter-by-section") {
        filtered = symbol_table.into_iter()
            .filter(|x| x.section == section)
            .collect::<SymbolTable>();        
    }
    filtered.sort_by_size_descending();


    // Output formatted HTML if requested
    match matches.value_of("html") {
        Some(filename) => {
            let mut object_table = File::create(filename).expect(&format!("Could not create file: {}", filename));
            write!(object_table, "{}", filtered.to_html()).expect("Could not write to file");
        }
        None => print!("{:?}", filtered)
    }
}

