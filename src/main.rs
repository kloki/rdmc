use bpaf::*;
use rdmc::{
    inputfile::get_fallback,
    parser::get_commands,
};
#[derive(Debug, Clone)]
pub struct Args {
    list: bool,
    command: String,
}

pub fn args() -> OptionParser<Args> {
    let command = positional("COMMAND").help("Command from readme");

    let list = short('l')
        .long("list")
        .help("List all possible readme commands")
        .switch(); // turn this into a switch

    construct!(Args { list, command }).to_options()
}
fn main() -> Result<(), String> {
    let input_file = get_fallback();
    let commands = get_commands(input_file).expect("Failed to parse markdown");
    let parsed = args().run();
    if parsed.list {
        for c in commands.values() {
            println!("{} - '{}'", c.name, c.command);
        }
        return Ok(());
    }
    match commands.get(&parsed.command) {
        None => return Err("Unknown input command".to_string()),
        Some(command) => {
            println!("{}", command.command);
            Ok(())
        }
    }
}
