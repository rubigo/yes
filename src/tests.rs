extern crate regex;
use self::regex::Regex;
use super::*;

#[test]
fn parse_args_help_works() {
    assert_eq!(parse_args(vec!["-h".to_string()]), Ok(ShowHelp));
    assert_eq!(parse_args(vec!["--help".to_string()]), Ok(ShowHelp));
}

#[test]
fn parse_args_version_works() {
    assert_eq!(parse_args(vec!["-v".to_string()]), Ok(ShowVersion));
    assert_eq!(parse_args(vec!["--version".to_string()]), Ok(ShowVersion));
}

#[test]
fn parse_args_uses_default_string() {
    assert_eq!(parse_args(vec![]), Ok(RepeatString(DEFAULT_STRING.to_string())));
}

#[test]
fn parse_args_uses_arguments() {
    assert_eq!(parse_args(vec!["str".to_string()]),
               Ok(RepeatString("str".to_string())));
}

#[test]
fn parse_args_concatenates_arguments() {
    assert_eq!(parse_args(vec!["str".to_string(), "dup".to_string()]),
               Ok(RepeatString("str dup".to_string())));
}

#[test]
fn parse_args_allows_escaped_flags() {
    assert_eq!(parse_args(vec!["--".to_string(), "-h".to_string()]),
               Ok(RepeatString("-h".to_string())));
}

#[test]
fn parse_args_raises_error_on_illegal_argument() {
    assert!(parse_args(vec!["-g".to_string()]).is_err());
    match parse_args(vec!["--global".to_string()]) {
        Err(InvalidOption(_)) => assert!(true),
        _ => unreachable!()
    }
}

#[test]
fn parse_args_error_has_message() {
    let re = Regex::new(r"Unrecognized option").unwrap();

    assert!(parse_args(vec!["-g".to_string()]).is_err());
    match parse_args(vec!["--global".to_string()]) {
        Err(e) => assert!(re.is_match(&e.message())),
        _ => unreachable!()
    }
}

#[test]
fn string_newline_terminate_works() {
    assert_eq!("word".to_string().newline_terminate(), "word\n");
    assert_eq!("myname".to_string().newline_terminate(), "myname\n");
}

#[test]
fn string_fill_works() {
    let max = 1500;
    let buf = "myword\n".to_string().fill(max);

    // make sure string isn't too long
    assert!(buf.len() <= max);

    // make sure buf only contains the word followed by a newline
    let re = Regex::new(r"^(myword\n)+$").unwrap();
    assert!(re.is_match(&buf));
}

#[test]
fn version_text_returns_version() {
    let re = Regex::new(r"^.* version \d+(\.\d+)*.*$").unwrap();
    assert!(re.is_match(&version_text()));
}

#[test]
fn help_text_documents_options() {
    // test if it documents the help flag
    let help_flag = Regex::new(r"--help").unwrap();
    assert!(help_flag.is_match(&help_text()));

    // test if it documents the version flag
    let version_flag = Regex::new(r"--version").unwrap();
    assert!(version_flag.is_match(&help_text()));

    // test if it documents the usage
    let usage = Regex::new(r"Usage: ").unwrap();
    assert!(usage.is_match(&help_text()));
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
