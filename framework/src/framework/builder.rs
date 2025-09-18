use crate::framework::FlareFramework;
use crate::structs::command::PrefixCommand;

#[derive(Debug)]
pub struct FlareFrameworkBuilder {
    prefix: String,
    commands: Vec<PrefixCommand>,
}

impl Default for FlareFrameworkBuilder {
    fn default() -> Self {
        let commands = Vec::new();
        Self {
            prefix: ";".to_string(),
            commands,
        }
    }
}

impl FlareFrameworkBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn prefix(mut self, prefix: &str) -> Self {
        self.prefix = prefix.to_string();
        self
    }

    pub fn command(mut self, command: impl Into<PrefixCommand>) -> Self {
        self.commands.push(command.into());
        self
    }

    pub fn commands(
        mut self,
        commands: impl IntoIterator<Item = impl Into<PrefixCommand>>,
    ) -> Self {
        self.commands
            .extend(commands.into_iter().map(|command| command.into()));
        self
    }

    #[cfg(feature = "macros")]
    pub fn macro_commands(mut self) -> Self {
        self.commands
            .extend(inventory::iter::<PrefixCommand>.into_iter().cloned());
        self
    }

    pub fn build(self) -> FlareFramework {
        FlareFramework {
            prefix: self.prefix,
            commands: self.commands,
        }
    }
}
