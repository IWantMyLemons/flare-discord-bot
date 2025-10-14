use std::{error::Error, process::Stdio};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    process::Command,
};

pub async fn minify_gif(bytes: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut ffmpeg = Command::new("ffmpeg")
        .args(["-i", "pipe:0", "-f", "gif", "-vf", "scale=32:32", "pipe:1"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    {
        let mut stdin = ffmpeg.stdin.take().expect("Failed to open stdin");
        stdin.write_all(&bytes).await?;
    }

    let mut stdout = ffmpeg.stdout.take().expect("Failed to open stdout");
    let mut output: Vec<u8> = Vec::new();
    stdout.read_to_end(&mut output).await?;
    
    ffmpeg.wait().await?; // wait for ffmpeg to actually close

    Ok(output)
}
