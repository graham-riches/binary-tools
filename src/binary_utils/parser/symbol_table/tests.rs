use nom::error::ErrorKind;
use super::*;


#[test]
fn test_parse_decimal_from_string() {
    assert_eq!(parse_u32("12345"), Ok(("", 74565)));
    assert!(parse_u32("gabcdeg").is_err());
}

#[test]
fn test_parse_flag_bit_scope() {
    assert_eq!(parse_flag_bit_scope("l"), Ok(("", SymbolScope::Local)));
    assert_eq!(parse_flag_bit_scope("g"), Ok(("", SymbolScope::Global)));
    assert_eq!(parse_flag_bit_scope(" "), Ok(("", SymbolScope::Neither)));
    assert_eq!(parse_flag_bit_scope("!"), Ok(("", SymbolScope::Both)));
    assert_eq!(
        parse_flag_bit_scope("x"),
        Err(nom::Err::Error(nom::error::Error {
            input: "x",
            code: ErrorKind::OneOf
        }))
    );
}

#[test]
fn test_parse_flag_bit_weakness() {
    assert_eq!(parse_flag_bit_weakness("w"), Ok(("", SymbolWeakness::Weak)));
    assert_eq!(
        parse_flag_bit_weakness(" "),
        Ok(("", SymbolWeakness::Strong))
    );
    assert_eq!(
        parse_flag_bit_weakness("x"),
        Err(nom::Err::Error(nom::error::Error {
            input: "x",
            code: ErrorKind::OneOf
        }))
    );
}

#[test]
fn test_parse_flag_bit_constructor() {
    assert_eq!(
        parse_flag_bit_constructor("C"),
        Ok(("", SymbolConstructor::Constructor))
    );
    assert_eq!(
        parse_flag_bit_constructor(" "),
        Ok(("", SymbolConstructor::Regular))
    );
    assert_eq!(
        parse_flag_bit_constructor("x"),
        Err(nom::Err::Error(nom::error::Error {
            input: "x",
            code: ErrorKind::OneOf
        }))
    );
}

#[test]
fn test_parse_flag_bit_warning() {
    assert_eq!(
        parse_flag_bit_warning("W"),
        Ok(("", SymbolWarning::Warning))
    );
    assert_eq!(
        parse_flag_bit_warning(" "),
        Ok(("", SymbolWarning::Regular))
    );
    assert_eq!(
        parse_flag_bit_warning("x"),
        Err(nom::Err::Error(nom::error::Error {
            input: "x",
            code: ErrorKind::OneOf
        }))
    );
}

#[test]
fn test_parse_flag_bit_reference() {
    assert_eq!(
        parse_flag_bit_reference("l"),
        Ok(("", SymbolReference::Reference))
    );
    assert_eq!(
        parse_flag_bit_reference(" "),
        Ok(("", SymbolReference::Regular))
    );
    assert_eq!(
        parse_flag_bit_reference("x"),
        Err(nom::Err::Error(nom::error::Error {
            input: "x",
            code: ErrorKind::OneOf
        }))
    );
}

#[test]
fn test_parse_flag_bit_debugging() {
    assert_eq!(
        parse_flag_bit_debugging("d"),
        Ok(("", SymbolDebugging::Debug))
    );
    assert_eq!(
        parse_flag_bit_debugging("D"),
        Ok(("", SymbolDebugging::Dynamic))
    );
    assert_eq!(
        parse_flag_bit_debugging(" "),
        Ok(("", SymbolDebugging::Regular))
    );
    assert_eq!(
        parse_flag_bit_debugging("x"),
        Err(nom::Err::Error(nom::error::Error {
            input: "x",
            code: ErrorKind::OneOf
        }))
    );
}

#[test]
fn test_parse_flag_bit_type() {
    assert_eq!(parse_flag_bit_type("F"), Ok(("", SymbolType::Function)));
    assert_eq!(parse_flag_bit_type("f"), Ok(("", SymbolType::File)));
    assert_eq!(parse_flag_bit_type("O"), Ok(("", SymbolType::Object)));
    assert_eq!(parse_flag_bit_type(" "), Ok(("", SymbolType::Regular)));
    assert_eq!(
        parse_flag_bit_type("x"),
        Err(nom::Err::Error(nom::error::Error {
            input: "x",
            code: ErrorKind::OneOf
        }))
    );
}

#[test]
fn test_parse_symbol_table_flags() {
    assert_eq!(
        parse_symbol_flags("l    df"),
        Ok((
            "",
            SymbolTableFlags {
                scope: SymbolScope::Local,
                weakness: SymbolWeakness::Strong,
                constructor: SymbolConstructor::Regular,
                warning: SymbolWarning::Regular,
                reference: SymbolReference::Regular,
                debugging: SymbolDebugging::Debug,
                symbol_type: SymbolType::File
            }
        ))
    );
}

#[test]
fn test_parse_symbol_table_flags_with_empty_start() {
    assert_eq!(
        parse_symbol_flags(" w   df"),
        Ok((
            "",
            SymbolTableFlags {
                scope: SymbolScope::Neither,
                weakness: SymbolWeakness::Weak,
                constructor: SymbolConstructor::Regular,
                warning: SymbolWarning::Regular,
                reference: SymbolReference::Regular,
                debugging: SymbolDebugging::Debug,
                symbol_type: SymbolType::File
            }
        ))
    );
}

#[test]
fn test_parse_symbol_table_entry() {
    let result =
        parse_symbol_table_entry("08020000 l    d  .vectors	00000000 .vectors").unwrap();
    let entry = result.1;
    assert_eq!(entry.address, 134348800);
    assert_eq!(entry.name, ".vectors");
    assert_eq!(entry.alignment_or_size, 0);
    assert_eq!(entry.section, ".vectors");
    assert_eq!(
        entry.flags,
        SymbolTableFlags {
            scope: SymbolScope::Local,
            weakness: SymbolWeakness::Strong,
            constructor: SymbolConstructor::Regular,
            warning: SymbolWarning::Regular,
            reference: SymbolReference::Regular,
            debugging: SymbolDebugging::Debug,
            symbol_type: SymbolType::Regular
        }
    );
}

#[test]
fn test_parse_symbol_table_entry_with_empty_flag_start() {
    let result =
        parse_symbol_table_entry("0803f76a  w    F .text	00000002 __printf_unlock").unwrap();
    let entry = result.1;
    assert_eq!(entry.address, 134477674);
    assert_eq!(entry.section, ".text");
    assert_eq!(entry.alignment_or_size, 2);
    assert_eq!(entry.name, "__printf_unlock");
    assert_eq!(
        entry.flags,
        SymbolTableFlags {
            scope: SymbolScope::Neither,
            weakness: SymbolWeakness::Weak,
            constructor: SymbolConstructor::Regular,
            warning: SymbolWarning::Regular,
            reference: SymbolReference::Regular,
            debugging: SymbolDebugging::Regular,
            symbol_type: SymbolType::Function
        }
    );
}
