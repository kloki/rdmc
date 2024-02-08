use bpaf::*;
use rdmc::{
    inputfile::get_fallback,
    parser::get_commands,
};
#[derive(Debug, Clone)]
pub struct Args {
    command: String,
}
fn completer(input: &String) -> Vec<(String, Option<String>)> {
    let input_file = get_fallback();
    let commands = get_commands(input_file).expect("Failed to parse markdown");
    commands
        .values()
        .filter_map(|c| {
            if c.name.starts_with(input) {
                Some((c.name.to_string(), None))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}
pub fn args() -> OptionParser<Args> {
    let command = positional("COMMAND")
        .help("Command from readme")
        .complete(completer);

    construct!(Args { command }).to_options()
}
fn main() -> Result<(), String> {
    let input_file = get_fallback();
    let commands = get_commands(input_file).expect("Failed to parse markdown");
    let parsed = args().run();
    match commands.get(&parsed.command) {
        None => return Err("Unknown input command".to_string()),
        Some(command) => {
            println!("{}", command.command);
            Ok(())
        }
    }
}
