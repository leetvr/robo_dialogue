use serde::Deserialize;
use std::path::Path;

fn main() {}

#[derive(Deserialize, Debug)]
struct DialogueLine {
    ID: usize,
    Actor: String,
    Dialogue: String,
    Trigger: String,
    Notes: String,
}

pub fn record<P: AsRef<Path>>(path: P) {
    let file = std::fs::File::open(path.as_ref()).unwrap();
    let mut reader = csv::Reader::from_reader(&file);
    for row in reader.deserialize::<DialogueLine>() {
        let line = row.unwrap();
        println!("{} - {}", line.Actor, line.Dialogue);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_dialogue() {
        // Open the file, and write it.
        record("./test_dialogue.csv");
    }
}
