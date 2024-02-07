use std::fs;

use rdmc::parser::get_commands;
fn main() {
    let input = fs::read_to_string("./tests/test_files/duplicate.md").expect("Can't read file");
    let commands = get_commands(input).expect("Failed to parse markdown");
    for command in commands.values() {
        println!("#{} - '{}'", command.name, command.command)
    }
}
