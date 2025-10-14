use framework::command;
use framework::prelude::*;
use serenity::all::CreateAttachment;
use serenity::all::CreateMessage;

use crate::templates::error_message;
use crate::wrappers::ffmpeg_wrapper::minify_gif;

#[command]
pub async fn minify<'a>(context: PrefixContext<'a>) -> CommandResult {
    if let Some(attachment) = context.msg.attachments.first() {
        let file = match attachment.download().await {
            Ok(file) => file,
            Err(e) => {
                return CommandResult::Err(error_message(
                    "Failed to download attachment",
                    &e.to_string(),
                ));
            }
        };
        
        let small = match minify_gif(file).await {
            Ok(file) => file,
            Err(e) => {
                return CommandResult::Err(error_message(
                    "Failed to minify file",
                    &e.to_string(),
                ));
            }
        };

        let new_attachment = CreateAttachment::bytes(small, &attachment.filename);
        let message = CreateMessage::new().add_file(new_attachment);

        return message.into();
    }

    CommandResult::Err(error_message(
        "No image specified",
        "Try attaching or replying to an image",
    ))
}
