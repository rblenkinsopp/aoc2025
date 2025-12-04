use memchr::memchr;
use memmap2::Mmap;
use std::alloc::System;
use std::ffi::OsString;
use std::fs::File;
use std::io::BufReader;
use std::sync::OnceLock;
use std::{env, fs};
use std::str::FromStr;

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

        self.offset = start + self.stride;
        Some(unsafe { self.bytes.get_unchecked(start..end) })
    }
}

pub struct Grid {
    bytes: Vec<u8>,
    width: usize,
    height: usize,
}

pub struct GridPoint<'a> {
    grid: &'a Grid,
    x: usize,
    y: usize,
    value: &'a u8,
}

impl<'a> GridPoint<'a> {
    #[inline(always)]
    pub fn value(&self) -> u8 {
        *self.value
    }

    #[inline(always)]
    pub fn value_ref(&self) -> &'a u8 {
        self.value
    }

    #[inline(always)]
    pub fn index(&self) -> usize {
        self.y * self.grid.width + self.x
    }

    #[inline(always)]
    pub fn adjacent_iter(&self) -> impl Iterator<Item = GridPoint<'a>> + '_ {
        #[rustfmt::skip]
        const OFFSETS: [(isize, isize); 8] = [
            (-1, -1), (0, -1), (1, -1),
            (-1,  0), /* GP */ (1,  0),
            (-1,  1), (0,  1), (1,  1),
        ];

        OFFSETS.into_iter().filter_map(move |(dx, dy)| {
            let nx = self.x as isize + dx;
            let ny = self.y as isize + dy;

            if nx < 0 || ny < 0 || nx >= self.grid.width as isize || ny >= self.grid.height as isize
            {
                return None;
            }

            let x = nx as usize;
            let y = ny as usize;
            let idx = y * self.grid.width + x;
            let value = unsafe {
                // Safety: Indices have already been validated above.
                self.grid.bytes.get_unchecked(idx)
            };

            Some(GridPoint {
                grid: self.grid,
                x,
                y,
                value,
            })
        })
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_uniform_input_iter(UniformInputIterator::from_bytes(s.as_bytes())))
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

    pub fn iter(&self) -> GridPointIterator<'_> {
        GridPointIterator {
            grid: self,
            x: 0,
            y: 0,
        }
    }

    #[inline(always)]
    pub fn set_index(&mut self, idx: usize, v: u8) {
        self.bytes[idx] = v;
    }
}

pub struct GridPointIterator<'a> {
    grid: &'a Grid,
    x: usize,
    y: usize,
}

impl<'a> Iterator for GridPointIterator<'a> {
    type Item = GridPoint<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.grid.height {
            return None;
        }

        let index = self.y * self.grid.width + self.x;
        let value = unsafe {
            // Safety: These indices have been previously checked to be in-range.
            self.grid.bytes.get_unchecked(index)
        };

        let point = GridPoint {
            grid: self.grid,
            x: self.x,
            y: self.y,
            value,
        };

        self.x += 1;
        if self.x >= self.grid.width {
            self.x = 0;
            self.y += 1;
        }

        Some(point)
    }
}
