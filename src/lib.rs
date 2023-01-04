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

pub fn handle(word: String) {
    // if lets are kinda clunky
    if let Some(definition) = lookup(&word) {
        display(definition);
    } else if let Some(corrected) = correct(&word) {
        println!("Could not find word {}, continuing with {}...", word, corrected);
        if let Some(definition) = lookup(&corrected) {
            display(definition);
        } else {
            println!("Could not find corrected word {}.", corrected);
        }
    } else {
        println!("Could not find word {}. Check your spelling?", word);
    }
}

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

// now we do inefficient string manipulation
// but it's fine because we're working with MUCH smaller strings lol
fn display(definition: String) {
    // todo: implement
    let definition = Configuration::default().parse(&definition);
    // dbg!(definition.warnings);
    // dbg!(&definition.nodes);

    // this is really quite terrible code
    let mut inside_heading = false;
    let mut english_heading = false;
    for node in &definition.nodes {
        if !inside_heading {
            if let Node::Heading { nodes, level, .. } = node {
                if *level == 2 {
                    if let Node::Text { value, .. } = nodes.get(0).unwrap() { // :-(
                        if *value == "English" {
                            inside_heading = true;
                            english_heading = true;
                            println!("{}", value);
                        }
                    }
                }
            }
        } else {
            if let Node::Heading { nodes, level, .. } = node && *level == 2 {
                inside_heading = false;
            } else {
                display_ii(node);
            }
        }
    }
    if !english_heading {
        assert!(inside_heading == false);
        for node in &definition.nodes {
            if !inside_heading {
                if let Node::Heading { nodes, level, .. } = node {
                    if *level == 2 {
                        if let Node::Text { value, .. } = nodes.get(0).unwrap() {
                            inside_heading = true;
                            println!("{}", value);
                        }
                    }
                }
            } else {
                if let Node::Heading { nodes, level, .. } = node && *level == 2 {
                    inside_heading = false;
                } else {
                    display_ii(node);
                }
            }
        }
    }
}

// no overloading?? O_O
fn display_ii(node: &Node) {
    match node {
        Node::CharacterEntity { character, .. } => print!("{}", character),
        Node::Text { value, .. } => print!("{}", value),
        Node::Link { text, target, .. } => {
            assert!(text.len() == 1);
            display_ii(text.get(0).unwrap());
        },

        Node::Heading { nodes, level, .. } => {
            assert!(nodes.len() == 1);
            display_ii(nodes.get(0).unwrap());
            println!();
        },
        Node::HorizontalDivider { end, start } => println!("\n------"),
        Node::ParagraphBreak { .. } => print!(" "),
        Node::Template { name, parameters, .. } => {
            for parameter in parameters.iter().rev() {
                if let Some(name) = &parameter.name {
                    // print!("(");
                    // for value in &parameter.value {
                    //     display_ii(&value);
                    // }
                    // print!(") ");
                } else {
                    for value in &parameter.value {
                        display_ii(&value);
                    }
                    break;
                }
            }
        }
        Node::OrderedList { items, .. } => (),
        Node::UnorderedList { items, .. } => (),

        Node::Preformatted { nodes, .. } => (),
        Node::Category { .. } => (),
        Node::Italic { .. } => (),
        _ => todo!()
    }
}

pub fn param(word: String) {
    match word.as_str() { // curious about this
        "--help" => {
            println!("dictionarium {}", version);
            println!("");
            println!("Usage: dictionarium <word>");
        },
        "--full" => { // set some global variable

        },
        _ => {
            println!("Unknown flag \"{}\".", word);
        }
    }
}
