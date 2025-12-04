use memchr::{memchr, memchr_iter};
use memmap2::Mmap;
use std::alloc::System;
use std::ffi::OsString;
use std::fs::File;
use std::io::BufReader;
use std::ops::Deref;
use std::str::FromStr;
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
    const DOUBLE_NEWLINE: &str = "\n\n";

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

        self.offset = start + self.stride;
        Some(unsafe { self.bytes.get_unchecked(start..end) })
    }
}

pub struct Grid {
    bytes: Vec<u8>,
    width: usize,
    height: usize,
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_uniform_input_iter(
            UniformInputIterator::from_bytes(s.as_bytes()),
        ))
    }
}

impl Grid {
    pub fn from_uniform_input_iter(iter: UniformInputIterator<'_>) -> Self {
        let mut bytes = vec![];
        let mut lines = 0;
        let line_length = OnceLock::new();

        for line in iter {
            line_length.get_or_init(|| line.len());
            bytes.extend_from_slice(line);
            lines += 1;
        }

        Self {
            bytes,
            width: *line_length.get().unwrap(),
            height: lines,
        }
    }

    #[inline(always)]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline(always)]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = GridPoint<'_>> + '_ {
        (0..self.bytes.len()).map(move |offset| GridPoint { grid: self, offset })
    }

    #[inline(always)]
    pub fn filter_iter(&self, needle: u8) -> impl Iterator<Item = GridPoint<'_>> + '_ {
        memchr_iter(needle, &self.bytes).map(move |offset| GridPoint { grid: self, offset })
    }

    #[inline(always)]
    pub fn set_offset(&mut self, offset: usize, v: u8) {
        self.bytes[offset] = v;
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }
}

impl AsRef<[u8]> for Grid {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl AsMut<[u8]> for Grid {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }
}

#[derive(Clone, Copy)]
pub struct GridPoint<'a> {
    grid: &'a Grid,
    offset: usize,
}

impl<'a> GridPoint<'a> {
    #[inline(always)]
    pub fn offset(self) -> usize {
        self.offset
    }

    #[inline(always)]
    pub fn index(self) -> (usize, usize) {
        (self.x(), self.y())
    }

    #[inline(always)]
    pub fn x(self) -> usize {
        self.offset % self.grid.width
    }

    #[inline(always)]
    pub fn y(self) -> usize {
        self.offset / self.grid.width
    }

    #[inline(always)]
    pub fn value(self) -> u8 {
        unsafe { *self.grid.bytes.get_unchecked(self.offset) }
    }

    #[inline(always)]
    pub fn adjacent_iter(&self) -> impl Iterator<Item = GridPoint<'a>> + 'a {
        let grid = self.grid;
        let w = grid.width;
        let h = grid.height;
        let x = self.offset % w;
        let y = self.offset / w;

        let up = y > 0;
        let down = y + 1 < h;
        let left = x > 0;
        let right = x + 1 < w;

        [
            (up && left).then_some(self.offset - w - 1),
            up.then_some(self.offset - w),
            (up && right).then_some(self.offset - w + 1),
            left.then_some(self.offset - 1),
            right.then_some(self.offset + 1),
            (down && left).then_some(self.offset + w - 1),
            down.then_some(self.offset + w),
            (down && right).then_some(self.offset + w + 1),
        ]
        .into_iter()
        .flatten()
        .map(move |offset| GridPoint { grid, offset })
    }
}

impl Deref for GridPoint<'_> {
    type Target = u8;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.grid.bytes[self.offset]
    }
}
