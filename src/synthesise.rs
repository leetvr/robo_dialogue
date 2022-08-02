use std::path::{Path, PathBuf};

use aws_sdk_polly::{
    model::{Engine, OutputFormat, VoiceId},
    Client, Error,
};
use tokio::io::AsyncWriteExt;

use crate::DialogueLine;

// Create speech from text.
// Lifted from AWS examples: https://github.com/awslabs/aws-sdk-rust/blob/main/examples/polly/src/bin/synthesize-speech.rs
pub async fn synthesise(client: &Client, dialogue_line: DialogueLine) -> Result<PathBuf, Error> {
    let line = &dialogue_line.dialogue;

    // This is incredibly specific to us, but is trivial to change.
    let voice_id = if dialogue_line.actor == "Alice" {
        VoiceId::Salli
    } else {
        VoiceId::Amy
    };

    let resp = client
        .synthesize_speech()
        .engine(Engine::Neural)
        .output_format(OutputFormat::OggVorbis)
        .text(line)
        .voice_id(voice_id)
        .send()
        .await?;

    // Get OGG data from response and save it
    let mut blob = resp
        .audio_stream
        .collect()
        .await
        .expect("failed to read data");

    let file_name = format!("{}{}", dialogue_line.id, ".ogg");
    let out_file = Path::new(&file_name);

    let mut file = tokio::fs::File::create(out_file)
        .await
        .expect("failed to create file");

    file.write_all_buf(&mut blob)
        .await
        .expect("failed to write to file");

    Ok(out_file.to_path_buf())
}
