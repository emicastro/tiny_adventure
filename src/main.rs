use csv::{ReaderBuilder, StringRecord};
use std::collections::HashMap;
use std::fs;

const FILENAME: &str = "story.csv";
const FIRST_TAG: &str = "BEGINNING";

#[derive(Debug)]
struct Sentence {
    output_type: String,
    tag: String,
    text: String,
    health: i32,
    options: Vec<Sentence>,
}

impl Sentence {
    fn new(row: StringRecord) -> Sentence {
        let health = row.get(3).unwrap().trim();
        let health: i32 = health.parse().unwrap_or(0);

        let sentence = Sentence {
            output_type: row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            text: row.get(2).unwrap().trim().to_string(),
            health: health,
            options: vec![],
        };
        sentence
    }
}


fn main() {
    let mut health = 100;
    let mut current_tag = FIRST_TAG;

    let mut last_sentence = String::new();

    let mut story_sentences: HashMap<String, Sentence> = HashMap::new();

    let content = fs::read_to_string(FILENAME).unwrap();

    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());

    for result in rdr.records() {
        let result = result.unwrap();
        let sentence = Sentence::new(result);

        if sentence.output_type == "SITUATION" {
            let sentence_tag = sentence.tag.clone();
            story_sentences.insert(sentence_tag.clone(), sentence);
            last_sentence = sentence_tag;

        } else if sentence.output_type == "OPTION" {
            if let Some(data) = story_sentences.get_mut(&last_sentence) {
                (*data).options.push(sentence);
            }
        }
    }

    // Game Loop
    loop {
        println!("You have {}% of health", health);

        if let Some(data) = story_sentences.get(current_tag) {
            println!("{}", data.text);

            for (idx, option) in data.options.iter().enumerate() {
                println!("[{}]{}", idx, option.text);
            }

            break;
        }
    }
}
