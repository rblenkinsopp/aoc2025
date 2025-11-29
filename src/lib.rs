use std::{env, fs};
use std::fs::File;
use std::io::BufReader;

#[inline]
/// Returns the filename of the input file as a string
pub fn get_input_filename() -> String {
    env::args().nth(1).expect("Input file was not specified")
}

#[inline]
/// Opens the input file and returns a BufReader
pub fn get_input_reader() -> BufReader<File> {
    BufReader::new(File::open(get_input_filename()).expect("Could not open input file"))
}

#[inline]
/// Reads the input file and returns it as a string
pub fn get_input_as_string() -> String {
    fs::read_to_string(get_input_filename()).expect("Could not open input file")
}

/// Gets the two different parts of the puzzle input as strings
#[inline]
pub fn get_two_part_input_as_strings() -> (String, String) {
    // Only support Unix newlines for now.
    const DOUBLE_NEWLINE: &str = "\n\n";

    // Find the split point and then split off the second part input.
    // This does not use 'String::split_once' as that leads to additional allocations.
    let mut input = get_input_as_string();
    let split_pos = input
        .find(DOUBLE_NEWLINE)
        .expect("Puzzle input is not seperated into two parts");
    let second = input.split_off(split_pos + DOUBLE_NEWLINE.len());
    input.truncate(split_pos);

    (input, second)
}
