#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
#![feature(let_chains)]

use std::io::*;
use std::fs::File;
// note that bufread::MultiBzDecoder is _distinct_ from read::MultiBzDecoder
use bzip2::bufread::*;
use parse_wiki_text::*;

// https://github.com/rust-lang/rfcs/issues/1349
const version: &str = env!("CARGO_PKG_VERSION");
const index_path: &str = env!("index_path");
const dictionary_path: &str = env!("dictionary_path");

pub fn handle(word: String, state: &State) {
    // if lets are kinda clunky
    if let Some(definition) = lookup(&word) {
        display(definition, &state);
    } else if let Some(corrected) = correct(&word) {
        println!("Could not find word {}, continuing with {}...", word, corrected);
        if let Some(definition) = lookup(&corrected) {
            display(definition, &state);
        } else {
            println!("Could not find corrected word {}.", corrected);
        }
    } else {
        println!("Could not find word {}. Check your spelling?", word);
    }
}

// WHY can you not implement traits on external types, like what??
// fortunately we needed to copy-paste the parse_wiki_text library to fix some bugs anyhow
fn lookup(word: &str) -> Option<String> {
    let file = File::open(index_path).expect("Failed to open index file");
    let reader = BufReader::new(MultiBzDecoder::new(BufReader::new(file)));
    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        // format: file-offset:page-id:page-title
        let line = line.splitn(3, ":").collect::<Vec<&str>>();
        assert!(line.len() == 3, "Failed to parse line. Is your index file valid?");

        let offset = line.get(0).unwrap().parse::<u64>()
            .expect("Failed to parse offset. Is your index file valid?");
        let id = line.get(1).unwrap().parse::<u64>()
            .expect("Failed to parse id. Is your index file valid?");
        let title = *line.get(2).unwrap(); // this dereference now makes sense

        if title == word {
            let file = File::open(dictionary_path)
                .expect("Failed to open dictionary file");
            let mut reader = BufReader::new(file);

            // note: our chunk contains multiple pages
            let offset = reader.seek(SeekFrom::Start(offset))
                .expect("Bad offset. Is your index file valid?");
            let reader = BufReader::new(BzDecoder::new(reader));

            let mut buffer = String::new();
            let mut page = false;
            for line in reader.lines() {
                let line = line.unwrap();
                if line == format!("    <title>{}</title>", title) {
                    buffer.push_str("  <page>");
                    buffer.push_str("\n");
                    page = true;
                }
                if page {
                    buffer.push_str(&line);
                    buffer.push_str("\n");
                    if line == "  </page>" {
                        break;
                    }
                }
            }
            return Some(buffer);
        }
    }
    return None;
}

// http://norvig.com/spell-correct.html
fn correct(word: &str) -> Option<&str> {
    // todo: implement
    return None;
}

// now we do somewhat inefficient string manipulation
// but it's fine because we're working with MUCH smaller strings lol
fn display(definition: String, state: &State) {
    let definition = Configuration::default().parse(&definition);

    // this is really quite terrible code
    if !display_ii(&definition, |value| value == &state.lang) {
        display_ii(&definition, |value| true);
    }
}

// i really miss static blocks
const skippable_headers: &[&str; 15] =
    &["Synonyms", "Antonyms", "Hyponyms", "Anagrams", "Translations",
    "Pronunciation", "Declension", "Inflection", "Descendants",
    "Derived terms", "Related terms", "See also", "Further reading",
    "References", "Alternative forms"];

// no overloading?? O_O
// matching on an enum of structs SUCKS
fn display_ii<F: Fn(&str) -> bool>(definition: &Output, f: F) -> bool {
    let mut inside_heading = false;
    let mut correct_language = false;
    let mut skipping_heading = false;
    for (i, node) in definition.nodes.iter().enumerate() {

        if let Node::Heading { nodes, level, .. } = node
        && let Some(Node::Text { value, .. }) = nodes.get(0) {
            if inside_heading {
                if *level == 2 {
                    inside_heading = false;
                } else if skippable_headers.contains(value) {
                    skipping_heading = true;
                } else {
                    if skipping_heading && !skippable_headers.contains(value) {
                        skipping_heading = false;
                    }
                    print!("\n{}\n", node);
                }
            } else if *level == 2 && f(*value) {
                inside_heading = true;
                correct_language = true;
                print!("{}", node);
            }
        } else if inside_heading && !skipping_heading {
            if let Node::OrderedList { .. } | Node::UnorderedList { .. } | Node::DefinitionList { .. } = node {
                print!("{}", format!("{}", node).trim());
            } else {
                print!("{}", node);
            }
        }
    }
    if correct_language {
        println!();
    }
    return correct_language;
}

// default values on structs please ;_;
pub struct State {
    pub full: bool,
    pub lang: String,
}

impl State {
    pub fn new() -> State {
        return State {
            full: false,
            lang: String::from("English"),
        }
    }
}

// mut state: State, yet state: &mut State?? huh??
pub fn param(word: String, state: &mut State) {
    match word.as_str() { // curious about this
        "--help" => {
            println!("dictionarium {}", version);
            println!("");
            println!("Usage: dictionarium <word>");
        },
        "--full" => { // set some global variable
            state.full = true;
        },
        _ => {
            println!("Unknown flag \"{}\".", word);
        }
    }
}
