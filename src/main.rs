extern crate clap;
extern crate atty;
use clap::{App, Arg};
use binarytools::binary_utils::parser::symbol_table::{SymbolTable, SymbolTableEntry, SymbolType};
use std::io::{self, BufRead};


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
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("output")
            .help("Choose the output format. Options: terminal (default), html")
            .required(false)
            .default_value("terminal"));

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
    let filter: fn(&SymbolTableEntry) -> bool = match matches.value_of("filter") {
        Some("objects")   => |x| x.flags.symbol_type == SymbolType::Object,
        Some("functions") => |x| x.flags.symbol_type == SymbolType::Function,
        Some("files")     => |x| x.flags.symbol_type == SymbolType::File,
        _                 => |_| true,        
    };

    // Get the symbol table, filter it, sort it and print it out
    let mut bss = symbol_table.into_iter()
        .filter(filter)
        .collect::<SymbolTable>();
    bss.sort_by_size_descending();
    print!("{:?}", bss);
}
