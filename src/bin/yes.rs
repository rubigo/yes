#![doc(html_logo_url = "https://rubigo.github.io/coreutils/logo.png")]

extern crate rubigo_yes;
extern crate clap;
use clap::{App, Arg};
use std::io::stdout;
use rubigo_yes::*;

fn main() {
    let matches = App::new(rubigo_yes::NAME)
        .version(rubigo_yes::VERSION)
        .about(rubigo_yes::DESCRIPTION)
        .arg(Arg::with_name("STRING")
            .help("The string to repeat endlessly")
            .multiple(true))
        .get_matches();

    let string = matches.values_of("STRING")
        .map(|v| v.collect::<Vec<&str>>().join(" "))
        .unwrap_or(rubigo_yes::DEFAULT_STRING.to_string());

    // get a reference to stdout
    let out = stdout();

    // lock stdout, and repeatedly write our string
    out.lock().yes(&string);
}
