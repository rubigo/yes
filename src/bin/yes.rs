extern crate rubigo_yes;
extern crate clap;
use clap::{App, Arg};
use std::io::stdout;
use rubigo_yes::*;

fn main() {
    // parse the args and run
    /*
    match rubigo_yes::parse_args(std::env::args().skip(1).collect()) {
        Ok(ref req) => req.do_it(),
        Err(ref err) => err.show()
    }
    */

    let matches = App::new(rubigo_yes::NAME)
        .version(rubigo_yes::VERSION)
        .about(rubigo_yes::DESCRIPTION)
        .arg(Arg::with_name("STRING")
            .help("The string to repeat endlessly")
            .multiple(true))
        .get_matches();

    /*
    // get a reference to stdout
    let out = stdout();

    // lock stdout, and repeatedly write our string
    out.lock()
        .write_repeat(
            s.newline_terminate()
            .fill(BUFSIZE)
            .as_bytes());
            */

    let string = matches.values_of("STRING")
        .map(|v| v.collect::<Vec<&str>>().join(" "))
        .unwrap_or(rubigo_yes::DEFAULT_STRING.to_string());

    // get a reference to stdout
    let out = stdout();

    // lock stdout, and repeatedly write our string
    out.lock()
        .write_repeat(
            string.newline_terminate()
            .fill(BUFSIZE)
            .as_bytes());
}
