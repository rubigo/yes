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
        _ => assert!(false)
    }
}

#[test]
fn repeat_buffer_uses_given_word() {
    let buf = repeat_buffer("word");

    // make sure buf only contains the word followed by a newline
    let re = Regex::new(r"^(word\n)+$").unwrap();
    assert!(re.is_match(&buf));
}

#[test]
fn version_text_returns_version() {
    let re = Regex::new(r"^.* version \d+(\.\d+)*.*$").unwrap();
    assert!(re.is_match(&version_text()));
}
