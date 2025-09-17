use serenity::all::CreateMessage;

#[derive(Debug)]
pub struct CommandResult {
    pub message: Option<CreateMessage>,
    pub value: Option<String>,
}