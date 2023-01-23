use csv::{ReaderBuilder, StringRecord};
use std::fs;

const FILENAME: &str = "story.csv";

`#[derive(Debug)]`
struct Sentence {
    output_type: String,
    tag: String,
    text: String,
    health: i32
}

impl Sentence {
    fn new(row: StringRecord) -> Sentence {
        let vida = row.get(3).unwrap().trim();
        let vida: i32 = vida.parse().unwrap_or(0);

        let sentence = Sentence {
            output_type: row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            text: row.get(2).unwrap().trim().to_string(),
            health: vida,
        };
    }
}


fn main() {
    let mut story_sentences: Vec<Sentence> = vec![];

    let content = fs::read_to_string(FILENAME).unwrap();

    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());

    for result in rdr.records() {
        let result = result.unwrap();

        let sentence = Sentence::new(result);

        story_sentences.push(sentence);
    }


    println!("{:?}", story_sentences);
}
