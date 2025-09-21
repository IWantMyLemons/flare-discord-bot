use crate::framework::FlareFramework;
use crate::structs::command::PrefixCommand;

#[derive(Debug)]
/// Used to construct and configure a [`FlareFramework`] struct 
pub struct FlareFrameworkBuilder {
    /// The word the bot looks for in the beginning of a command, defaults to `;`
    prefix: String,
    /// A list of all commands the bot knows, i recommend using the `"macro"`
    /// feature alongside [`FlareFrameworkBuilder::macro_commands()`]
    commands: Vec<PrefixCommand>,
}

impl Default for FlareFrameworkBuilder {
    /// initialises a default bot with the prefix `;` 
    fn default() -> Self {
        let commands = Vec::new();
        Self {
            prefix: ";".to_string(),
            commands,
        }
    }
}

impl FlareFrameworkBuilder {
    /// initialises a default bot with the prefix `;` 
    pub fn new() -> Self {
        Self::default()
    }

    /// Changes the prefix of the bot
    /// A prefix is the word the bot looks for in the beginning of a command
    /// defaults to `;`
    pub fn prefix(&mut self, prefix: &str) -> &mut Self {
        self.prefix = prefix.to_string();
        self
    }

    /// Adds a command to the bot
    pub fn command(&mut self, command: impl Into<PrefixCommand>) -> &mut Self {
        self.commands.push(command.into());
        self
    }

    /// Adds multiple commands to the bot
    pub fn commands(
        &mut self,
        commands: impl IntoIterator<Item = impl Into<PrefixCommand>>,
    ) -> &mut Self {
        self.commands
            .extend(commands.into_iter().map(|command| command.into()));
        self
    }

    #[cfg(feature = "macros")]
    /// Adds commands defined by [`framework::command`]
    pub fn macro_commands(&mut self) -> &mut Self {
        self.commands
            .extend(inventory::iter::<PrefixCommand>.into_iter().cloned());
        self
    }

    /// Builds the framework, read [`FlareFrameworkBuilder`] for `Default`s
    pub fn build(&self) -> FlareFramework {
        FlareFramework {
            prefix: self.prefix.clone(),
            commands: self.commands.clone(),
        }
    }
}
