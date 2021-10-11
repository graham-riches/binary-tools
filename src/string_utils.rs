use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

/// Read lines from a file into a vector of strings. Returns an IO result to handle any errors
/// like file missing, etc.
///
/// # Arguments
/// * 'filename' - Filename/path to read the data from
///
/// # Examples
/// ```no_run
/// use binarytools::string_utils;
/// let mut lines = string_utils::read_lines_from_file("temp.txt").expect("Could not read lines");
/// ```
pub fn read_lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

/// Split a string slice into a tuple of slices based on the first found location of a delimiter.
/// Returns an optional tuple of slices containing the two halves if they exist.
///
/// # Arguments
/// * 's' - The string slice to split by the delimiter
/// * 'd' - The char delimiter to split the string by
///
/// # Examples
/// ```
/// use binarytools::string_utils;
/// assert_eq!(string_utils::split_into_tuple("ABC:DEF", ':'),     Some(("ABC", "DEF")));
/// assert_eq!(string_utils::split_into_tuple("ABC", ':'),         None);
/// assert_eq!(string_utils::split_into_tuple("ABC:DEF:GHI", ':'), Some(("ABC", "DEF:GHI")));
/// assert_eq!(string_utils::split_into_tuple("ABC:", ':'),        Some(("ABC", "")));
/// assert_eq!(string_utils::split_into_tuple(":ABC", ':'),        Some(("", "ABC")));
/// ```
pub fn split_into_tuple(s: &str, d: char) -> Option<(&str, &str)> {
    match s.find(d) {
        Some(p) => Some((&s[0..p], &s[p + 1..])),
        None => None,
    }
}
