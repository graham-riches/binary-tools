extern crate clap;
use clap::{App};
use binarytools::binary_utils::parser::symbol_table::SymbolTable;

fn main() {
    let matches = App::new("Binary Disassembly Parser")
        .version("1.0.0")
        .author("Graham Riches")
        .about("Parses disassembled application binaries into static HTML")
        .args_from_usage(
            "<INPUT>                'The disassembled binary file to parse'
            -v, --verbosity=[LEVEL] 'Enables verbose output with optional levels: <0,1,2>'")
        .get_matches();

    let file = matches.value_of("INPUT").unwrap();
    let symbol_table = SymbolTable::from_file(file).expect("Could not parse input file as a symbol table");

    let mut bss = symbol_table.into_iter()
        .filter(|x| x.section == ".bss")
        .collect::<SymbolTable>();
    bss.sort_by_size_descending();
    print!("{:?}", bss);
    


}
