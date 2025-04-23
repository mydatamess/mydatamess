use std::collections::HashMap;

use super::command::Command;

pub struct CommandRegistry {
    commands: HashMap<String, Box<dyn Command>>,
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn add_cmd(mut self, cmd: impl Command + 'static) -> CommandRegistry {
        self.commands.insert(cmd.operation_id(), Box::new(cmd));
        self
    }

    pub fn get(&self, operation_id: &str) -> Option<&dyn Command> {
        self.commands.get(operation_id).map(|boxed| boxed.as_ref())
    }

    pub fn get_commands(&self) -> impl Iterator<Item = &dyn Command> {
        self.commands.values().map(|boxed| boxed.as_ref())
    }
}
