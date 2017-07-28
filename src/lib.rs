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
pub trait Fillable: ToOwned {
    /// Repeat self until it fills up to `max`.
    fn fill(&self, max: usize) -> Self::Owned;
}

impl Fillable for str {
    fn fill(&self, max: usize) -> Self::Owned {
        // calculate how many times our string fits into a bufsize
        let times = max / self.len();

        // repeat the string this many times and join it together
        repeat(self).take(times).collect::<Vec<&str>>().join("")
    }
}

/// A trait for something that can be terminated with a newline.
pub trait WithNewline: ToOwned {
    fn push(dest: &mut Self::Owned, ch: char);
    fn with_newline(&self) -> Self::Owned {
        let mut copy = self.to_owned();
        Self::push(&mut copy, '\n');
        copy
    }
}

impl WithNewline for str {
    fn push(dest: &mut Self::Owned, ch: char) {
        dest.push(ch);
    }
}

pub trait Yes {
    fn yes(&mut self, string: &str) -> io::Error;
}

impl<T: WriteRepeat> Yes for T {
    fn yes(&mut self, string: &str) -> io::Error {
        self.write_repeat(
            string.with_newline()
                .fill(BUFSIZE)
                .as_bytes())
    }
}
