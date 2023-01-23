use csv::{ReaderBuilder, StringRecord};
use std::fs;

const FILENAME: &str = "story.csv";

fn main() {
    let content = fs::read_to_string(FILENAME).unwrap();

    println!("{}", content);
}
