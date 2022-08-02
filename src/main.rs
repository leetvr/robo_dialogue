mod synthesise;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_polly::Client;
use serde::Deserialize;
use std::path::Path;
use synthesise::synthesise;

#[tokio::main]
async fn main() {
    let file = std::env::args()
        .nth(1)
        .expect("You must supply a file name");
    record(&file).await
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DialogueLine {
    pub id: usize,
    pub actor: String,
    pub dialogue: String,
    pub trigger: String,
    pub notes: String,
}

pub async fn record<P: AsRef<Path>>(path: P) {
    let file = std::fs::File::open(path.as_ref()).unwrap();
    let mut reader = csv::Reader::from_reader(&file);
    let region_provider = RegionProviderChain::default_provider();

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    for row in reader.deserialize::<DialogueLine>() {
        let dialogue_line = row.unwrap();
        synthesise(&client, dialogue_line)
            .await
            .expect("Unable to synthesise line");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn test_dialogue() {
        // Open the file, and record dialogue for each line.
        record("./test_dialogue.csv").await;
        for path in ["1.ogg", "2.ogg"] {
            // Two for one - ensure the file was written, then clean it up.
            let p = Path::new(path);
            std::fs::remove_file(p).unwrap();
        }
    }
}
