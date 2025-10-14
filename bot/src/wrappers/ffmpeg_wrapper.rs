use std::{error::Error, process::Stdio};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    process::Command,
};

pub async fn minify_gif(bytes: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    let ffmpeg = Command::new("ffmpeg")
        .args(["-i", "pipe:0", "-f", "gif", "-r", "1", "pipe:1"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    {
        let mut stdin = ffmpeg.stdin.expect("Failed to open stdin");
        stdin.write_all(&bytes).await?;
    }

    let mut stdout = ffmpeg.stdout.expect("Failed to open stdout");
    let mut output: Vec<u8> = Vec::new();
    stdout.read_to_end(&mut output).await?;

    Ok(output)
}
