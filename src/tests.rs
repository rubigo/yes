extern crate regex;
use self::regex::Regex;
use std::str;
use super::*;

#[test]
fn string_with_newline_works() {
    assert_eq!("word".with_newline(), "word\n");
    assert_eq!("myname".with_newline(), "myname\n");
}

#[test]
fn string_fill_works() {
    let max = 1500;
    let buf = "myword\n".fill(max);

    // make sure string isn't too long
    assert!(buf.len() <= max);

    // make sure buf only contains the word followed by a newline
    let re = Regex::new(r"^(myword\n)+$").unwrap();
    assert!(re.is_match(&buf));
}

struct Full {
}

impl Full {
    fn new() -> Self {
        Full{}
    }
}

impl Write for Full {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::BrokenPipe, "full"))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn write_repeat_stops_on_error() {
    assert_eq!(Full::new().write_repeat("some data".as_bytes()).kind(),
               io::ErrorKind::BrokenPipe);
}

struct Checker {
    re: Regex
}

impl Checker {
    fn new(reg: &str) -> Self {
        Checker { re: Regex::new(reg).unwrap() }
    }
}

impl Write for Checker {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        assert!(self.re.is_match(str::from_utf8(data).unwrap()));
        Err(io::Error::new(io::ErrorKind::BrokenPipe, "full"))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn yes_works() {
    Checker::new(r"^(some name\n)+$").yes("some name");
}
