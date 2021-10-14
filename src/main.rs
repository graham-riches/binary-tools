extern crate clap;
extern crate atty;
use clap::{App};
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

    let mut app = App::new("Binary Disassembly Parser")
        .version("1.0.0")
        .author("Graham Riches")
        .about("Parses disassembled application binaries and outputs some formatted data")
        .args_from_usage(
            "-i, --input=[FILE]      'The disassembled binary file to parse'
             -h, --html              'WIP - Exports all binary information as static HTML, ignores other options like function size, etc.'
             -f, --filter=[TYPE]     'Filter by. Options: objects, functions, files'
             -v, --verbosity=[LEVEL] 'Enables verbose output with optional levels: <0,1,2>'");

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
