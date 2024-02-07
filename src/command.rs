#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub command: String,
}

impl Command {
    pub fn new(name: &str, command: &str) -> Self {
        let name = name.trim().replace(" ", "-").to_string();
        let command = command.trim().to_string();
        Self { name, command }
    }
}
