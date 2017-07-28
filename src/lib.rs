extern crate getopts;

#[cfg(test)]
mod tests;

use getopts::{Options, ParsingStyle};
use std::io::{self, stdout, Write};
use std::fmt;
use std::iter::repeat;

/// Version of this crate. It is parsed from the environment variable that
/// cargo sets when building the crate.
pub const VERSION: &'static str         = env!("CARGO_PKG_VERSION");

/// Name of this tool.
pub const NAME: &'static str            = "yes";

/// A description of this tool.
pub const DESCRIPTION: &'static str     = "Repeats a given string and a newline \
character infinitely on stdout.";

/// The default string to repeat.
pub const DEFAULT_STRING: &'static str  = "y";

/// The buffer size to use when printing the repeated string.
pub const BUFSIZE: usize                = 16384;

/// Represents a request, this tells the `yes` utility what it has to do.
/// Depending on the options passed on the command line, we might take
/// different actions, which are represented here.
#[derive(Debug, PartialEq)]
pub enum Request {
    /// Main purpose of this utility: send a given string followed by a newline
    /// repeatedly to `stdout`.
    RepeatString(String),

    /// Print help information to `stdout`.
    ShowHelp,

    /// Print the utility’s version number to `stdout`.
    ShowVersion
}

/// These are all the errors that can occur in this utility.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// An invalid option was encountered while parsing the options.
    InvalidOption(getopts::Fail)
}

use Request::*;
use Error::*;

impl Error {
    /// Returns the error message associated with the error.
    pub fn message(&self) -> String {
        match *self {
            InvalidOption(ref fail) => fail.to_string()
        }
    }

    /// Prints the error to stderr.
    pub fn show(&self) {
        eprintln!("{}", self.message());
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ShowHelp            => write!(f, "{}", help_text()),
            ShowVersion         => write!(f, "{}", version_text()),
            RepeatString(ref s) => write!(f, "{}", s)
        }
    }
}

/// A trait for something that can be repeatedly (read: infintely) written to.
trait WriteRepeat {
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
trait Fillable {
    /// Repeat self until it fills up to `max`.
    fn fill(&self, max: usize) -> Self;
}

/// A trait for something that can be terminated with a newline.
trait NewlineTerminate: Clone {
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

impl Request {
    /// Do the request. This performs the action suggested by the request.
    pub fn do_it(&self) {
        match *self {
            ShowHelp            => println!("{}", self),
            ShowVersion         => println!("{}", self),
            RepeatString(ref s) => {
                // get a reference to stdout
                let out = stdout();

                // lock stdout, and repeatedly write our string
                out.lock()
                    .write_repeat(
                        s.newline_terminate()
                        .fill(BUFSIZE)
                        .as_bytes());
            }
        }
    }
}

/// Create an options object.
fn options() -> Options {
    let mut opts = Options::new();
    opts.parsing_style(ParsingStyle::StopAtFirstFree);

    // command-line options that this utility supports
    opts.optflag("h", "help", "display help text and exit");
    opts.optflag("v", "version", "output version information and exit");

    opts
}

/// Parse the command line arguments as options, returning either the parsed
/// `Request` or an `Error`, depending on the arguments.
pub fn parse_args(args: Vec<String>) -> Result<Request, Error> {
    // parse the command-line arguments, and return on error.
    let matches = match options().parse(args) {
        Ok(matches)  => matches,
        Err(failure) => return Err(InvalidOption(failure))
    };

    // if the help flag is set, we want to show the help text.
    if matches.opt_present("help") {
        return Ok(ShowHelp);
    }

    // if the version flag is set, we want to show the version.
    if matches.opt_present("version") {
        return Ok(ShowVersion);
    }

    // if there is no string set, we use the default one, otherwise we take the
    // free strings and join them together.
    let string = if matches.free.is_empty() {
        DEFAULT_STRING.to_owned()
    } else {
        matches.free.join(" ")
    };

    Ok(RepeatString(string))
}

/// Generates a help text using the `getopts` crate. It is derived from the
/// `NAME`, the `DESCRIPTION`, as well as from the options.
pub fn help_text() -> String {
    let opts = options();
    let short_usage = format!("{} [STRING]", opts.short_usage(NAME));
    opts.usage(&short_usage)
}

/// Generates a version text, uses the `VERSION` as well as the `NAME` consts.
pub fn version_text() -> String {
    format!("{} version {}", NAME, VERSION)
}
