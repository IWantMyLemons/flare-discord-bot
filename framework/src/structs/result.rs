use serenity::all::CreateMessage;

#[derive(Debug)]
pub struct CommandResult {
    pub message: Option<CreateMessage>,
    pub value: Option<String>,
}

impl From<CreateMessage> for CommandResult {
    fn from(message: CreateMessage) -> Self {
        Self { message: Some(message), value: None }
    }
}
impl From<Option<CreateMessage>> for CommandResult {
    fn from(message: Option<CreateMessage>) -> Self {
        Self { message, value: None }
    }
}
impl From<String> for CommandResult {
    fn from(value: String) -> Self {
        Self { message: None, value: Some(value) }
    }
}
impl From<Option<String>> for CommandResult {
    fn from(value: Option<String>) -> Self {
        Self { message: None, value}
    }
}