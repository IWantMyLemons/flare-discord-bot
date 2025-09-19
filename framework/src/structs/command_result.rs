use serenity::all::CreateMessage;

#[derive(Debug)]
pub enum CommandResult {
    Ok(CommandOk),
    Err(CreateMessage),
}

#[derive(Debug)]
pub struct CommandOk {
    pub message: Option<CreateMessage>,
    pub value: Option<String>,
}
impl CommandOk {
    /// Shorthand for setting both value and a message witih the same content
    pub fn from_same(value: String) -> Self {
        Self {
            message: Some(CreateMessage::new().content(value.clone())),
            value: Some(value),
        }
    }
}

impl From<CommandOk> for CommandResult {
    fn from(command_ok: CommandOk) -> Self {
        Self::Ok(command_ok)
    }
}
impl From<CreateMessage> for CommandResult {
    fn from(message: CreateMessage) -> Self {
        Self::Ok(CommandOk {
            message: Some(message),
            value: None,
        })
    }
}
impl From<Option<CreateMessage>> for CommandResult {
    fn from(message: Option<CreateMessage>) -> Self {
        Self::Ok(CommandOk {
            message,
            value: None,
        })
    }
}
impl From<String> for CommandResult {
    fn from(value: String) -> Self {
        Self::Ok(CommandOk {
            message: None,
            value: Some(value),
        })
    }
}
impl From<Option<String>> for CommandResult {
    fn from(value: Option<String>) -> Self {
        Self::Ok(CommandOk {
            message: None,
            value,
        })
    }
}