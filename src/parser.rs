use std::collections::HashMap;

use markdown::{
    mdast::{
        Code,
        Heading,
        Node,
    },
    to_mdast,
    ParseOptions,
};

const BASH: &'static str = "bash";

use crate::command::Command;
pub fn get_commands(input: String) -> Result<HashMap<String, Command>, String> {
    let mut commands: HashMap<String, Command> = HashMap::new();
    let parsed = to_mdast(&input, &ParseOptions::default())?;
    extract(&mut commands, parsed, "root".to_string());

    Ok(commands)
}

fn extract(commands: &mut HashMap<String, Command>, node: Node, name: String) {
    let mut current_name = name;

    if let Some(children) = node.children() {
        for child in children {
            match child {
                Node::Code(Code {
                    value: command,
                    lang: Some(lang),
                    ..
                }) => {
                    if lang == BASH {
                        let c = Command::new(&current_name, &command);
                        commands.insert(c.name.clone(), c);
                        current_name.push('\'');
                    }
                }

                Node::Heading(Heading {
                    children: hchildren,
                    ..
                }) => {
                    if hchildren.len() > 0 {
                        if let Node::Text(text) = &hchildren[0] {
                            current_name = text.value.clone();
                        }
                    }
                }

                _ => extract(commands, child.clone(), current_name.clone()),
            }
        }
    }
}
