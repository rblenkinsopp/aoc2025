use memchr::memchr;
use memmap2::Mmap;
use std::alloc::System;
use std::ffi::OsString;
use std::fs::File;
use std::io::BufReader;
use std::sync::OnceLock;
use std::{env, fs};

#[global_allocator]
static GLOBAL: System = System;

#[inline]
/// Returns the filename of the input file as an OsString
pub fn get_input_filename() -> OsString {
    env::args_os().nth(1).expect("Input file was not specified")
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

#[inline(always)]
/// Gets the input file as static memory-mapped bytes without allocations.
pub fn get_input_as_bytes() -> &'static [u8] {
    unsafe {
        static MMAP: OnceLock<Mmap> = OnceLock::new();
        MMAP.get_or_init(|| {
            Mmap::map(&File::open(get_input_filename()).expect("Failed to open input file"))
                .expect("Failed to mmap input file")
        })
        .as_ref()
    }
}

#[inline(always)]
/// Gets the input file as a static memory-mapped string without allocations.
pub fn get_input_as_str() -> &'static str {
    unsafe { str::from_utf8_unchecked(get_input_as_bytes()) }
}

pub struct UniformInputIterator<'a> {
    bytes: &'a [u8],
    length: usize,
    line_length: usize,
    stride: usize,
    offset: usize,
}

impl<'a> UniformInputIterator<'a> {
    #[inline(always)]
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        let length = bytes.len();
        let line_length = memchr(b'\n', bytes).unwrap_or(length);
        Self::from_bytes_with_line_length(bytes, line_length)
    }

    #[inline(always)]
    pub fn from_bytes_with_line_length(bytes: &'a [u8], line_length: usize) -> Self {
        Self {
            bytes,
            length: bytes.len(),
            line_length,
            stride: line_length + 1,
            offset: 0,
        }
    }
}

impl<'a> Iterator for UniformInputIterator<'a> {
    type Item = &'a [u8];

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let start = self.offset;
        let end = start + self.line_length;
        if end > self.length {
            return None;
        }

        self.offset =  start + self.stride;
        Some(unsafe { self.bytes.get_unchecked(start..end) })
    }
}
