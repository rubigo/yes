extern crate getopts;

use getopts::{Options, ParsingStyle};
use std::io::{stdout, Write};
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

/// Represents a request, this tells the yes utility what it has to do.
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

impl Request {
    /// Do the request. This performs the action suggested by the request.
    pub fn do_it(&self) {
        match *self {
            ShowHelp => println!("{}", self),
            ShowVersion => println!("{}", self),
            RepeatString(ref s) => repeat_string(s)
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

#[test]
fn test_parse_args() {
    // test help options
    assert_eq!(parse_args(vec!["-h".to_string()]), Ok(ShowHelp));
    assert_eq!(parse_args(vec!["--help".to_string()]), Ok(ShowHelp));

    // test version options
    assert_eq!(parse_args(vec!["-v".to_string()]), Ok(ShowVersion));
    assert_eq!(parse_args(vec!["--version".to_string()]), Ok(ShowVersion));

    // use the DEAFULT_STRING if nothing is specified
    assert_eq!(parse_args(vec![]), Ok(RepeatString(DEFAULT_STRING.to_string())));

    // use whatever is given on command line
    assert_eq!(parse_args(vec!["str".to_string()]),
               Ok(RepeatString("str".to_string())));

    // join multiple arguments together
    assert_eq!(parse_args(vec!["str".to_string(), "dup".to_string()]),
               Ok(RepeatString("str dup".to_string())));

    // allow command line flags as string
    assert_eq!(parse_args(vec!["--".to_string(), "-h".to_string()]),
               Ok(RepeatString("-h".to_string())));
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

/// Print a string repeatedly to `stdout`.
pub fn repeat_string(string: &str) {
    // creates a buffer, containing a lot of repetitions of our string
    let buf = repeat_buffer(string);

    // create a handle to stdout
    let stdout = stdout();
    let mut stdout = stdout.lock();

    // continuously print to stdout
    loop {
        match stdout.write_all(buf.as_bytes()) {
            Ok(_) => continue,
            Err(_) => break
        }
    }
}

/// Generates a buffer, which is maximally `BUFSIZE` big, consisting of the
/// string repeated as many times as will fit into the buffer, joined with
/// newline characters.
pub fn repeat_buffer(string: &str) -> String {
    // append newline to string
    let mut s = string.to_owned();
    s.push('\n');

    // calculate how many times our string fits into a bufsize
    let times = BUFSIZE / s.len();

    // repeat the string this many times and join it together
    repeat(s).take(times).collect::<Vec<String>>().join("")
}
