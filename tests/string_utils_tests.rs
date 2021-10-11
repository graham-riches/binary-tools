use binarytools::string_utils;

#[test]
fn test_read_lines_from_file() -> Result<(), std::io::Error> {
    let lines = string_utils::read_lines_from_file("tests/lines.txt")?;
    assert_eq!(lines.len(), 10);
    Ok(())
}