use serenity::all::{Color, CreateEmbed, CreateMessage};

const ERROR_COLOR: Color = Color::ROSEWATER;

pub fn error_message(title: &str, description: &str) -> CreateMessage {
    let embed = CreateEmbed::new()
        .color(ERROR_COLOR)
        .title(title)
        .description(description);
    CreateMessage::new().embed(embed)
}
