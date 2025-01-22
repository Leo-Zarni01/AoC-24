use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// Reads lines from a file and returns an iterator over the lines as `String`s.
///
/// # Arguments
/// - `filename`: The path to the file to read.
///
/// # Returns
/// - `io::Result<impl Iterator<Item = String>>`: An iterator over the lines in the file, or an error if the file cannot be opened or read.
///
pub fn read_lines<P>(filename: P) -> io::Result<impl Iterator<Item = String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().filter_map(Result::ok))
}
