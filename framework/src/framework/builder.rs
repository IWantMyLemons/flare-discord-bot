use crate::{FlareFramework, structs::command::PrefixCommand};

#[derive(Debug)]
pub struct FlareFrameworkBuilder {
    prefix: String,
    commands: Vec<PrefixCommand>,
}

impl Default for FlareFrameworkBuilder {
    fn default() -> Self {
        Self {
            prefix: ";".to_string(),
            commands: Vec::new(),
        }
    }
}

impl FlareFrameworkBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn prefix() {
        
    }

    pub fn command(mut self, command: impl Into<PrefixCommand>) -> Self {
        self.commands.push(command.into());
        self
    }

    pub fn commands(
        mut self,
        commands: impl IntoIterator<Item = impl Into<PrefixCommand>>,
    ) -> Self {
        self.commands.extend(commands.into_iter().map(|command| command.into()));
        self
    }

    pub fn build(self) -> FlareFramework {
        FlareFramework {
            prefix: self.prefix,
            commands: self.commands,
        }
    }
}
