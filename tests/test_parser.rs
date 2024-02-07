use std::fs;

use rdmc::parser::get_commands;
#[test]
fn test_simple() {
    let input = fs::read_to_string("./tests/test_files/simple.md").expect("Can't read file");
    let commands = get_commands(input).expect("Failed to parse markdown");
    assert_eq!(commands.keys().len(), 2)
}

#[test]
fn test_empty() {
    let input = fs::read_to_string("./tests/test_files/empty.md").expect("Can't read file");
    let commands = get_commands(input).expect("Failed to parse markdown");
    assert_eq!(commands.keys().len(), 0)
}

#[test]
fn test_weird_header() {
    let input = fs::read_to_string("./tests/test_files/weird_header.md").expect("Can't read file");
    let commands = get_commands(input).expect("Failed to parse markdown");
    assert_eq!(commands.keys().len(), 2);
    dbg!(&commands);
    assert!(commands.contains_key("root"));
    assert!(commands.contains_key("Header"));
}

#[test]
fn test_duplicate() {
    let input = fs::read_to_string("./tests/test_files/duplicate.md").expect("Can't read file");
    let commands = get_commands(input).expect("Failed to parse markdown");
    assert_eq!(commands.keys().len(), 2);
}
