extern crate getopts;
extern crate textwrap;

#[cfg(test)]
mod tests;

use std::io::{self, Write};
use std::iter::repeat;

/// Version of this crate. It is parsed from the environment variable that
/// cargo sets when building the crate.
pub const VERSION: &'static str         = env!("CARGO_PKG_VERSION");

/// Name of this tool.
pub const NAME: &'static str            = "yes";

/// A description of this tool.
pub const DESCRIPTION: &'static str     = "Repeats a given string and a newline \
character infinitely on stdout. If no string is supplied, it uses the default \
string ‘y’. Command exits as soon as stdout is closed and no more data can be \
written to it. ";

/// The default string to repeat.
pub const DEFAULT_STRING: &'static str  = "y";

/// The buffer size to use when printing the repeated string.
pub const BUFSIZE: usize                = 16384;

/// A trait for something that can be repeatedly (read: infintely) written to.
pub trait WriteRepeat {
    /// Write the given data repeatedly, until writing triggers an error.
    fn write_repeat(&mut self, data: &[u8]) -> io::Error;
}

impl<T: Write> WriteRepeat for T {
    fn write_repeat(&mut self, data: &[u8]) -> io::Error {
        loop {
            match self.write_all(data) {
                Ok(_) => continue,
                Err(e) => return e
            }
        }
    }
}

/// A trait for something that can be repeated to fit a certain size.
pub trait Fillable {
    /// Repeat self until it fills up to `max`.
    fn fill(&self, max: usize) -> Self;
}

/// A trait for something that can be terminated with a newline.
pub trait NewlineTerminate: Clone {
    fn push(&mut self, ch: char);
    fn newline_terminate(&self) -> Self {
        let mut copy = self.clone();
        copy.push('\n');
        copy
    }
}

impl Fillable for String {
    fn fill(&self, max: usize) -> Self {
        // calculate how many times our string fits into a bufsize
        let times = max / self.len();

        // repeat the string this many times and join it together
        repeat(self.as_str()).take(times).collect::<Vec<&str>>().join("")
    }
}

impl NewlineTerminate for String {
    fn push(&mut self, ch: char) {
        self.push(ch);
    }
}
