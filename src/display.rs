use crate::state::*;
use parse_wiki_text::*;

// i really miss static blocks
const skippable_headers: &[&str; 15] =
    &["Synonyms", "Antonyms", "Hyponyms", "Anagrams", "Translations",
    "Pronunciation", "Declension", "Inflection", "Descendants",
    "Derived terms", "Related terms", "See also", "Further reading",
    "References", "Alternative forms"];

// now we do somewhat inefficient string manipulation
// but it's fine because we're working with MUCH smaller strings lol
pub fn display(definition: String, state: &State) {
    let definition = Configuration::default().parse(&definition);
    display_language(&definition, &state.lang);
}

/// Prints only the provided language if present, otherwise prints the first language
fn display_language(definition: &Output, lang: &str) {
    let mut has_lang = false;
    for node in definition.nodes.iter() {
        if let Node::Heading { nodes, level, .. } = node
        && let Some(Node::Text { value, .. }) = nodes.get(0) {
            if *level == 2 && *value == lang {
                has_lang = true;
                break;
            }
        }
    }

    let mut skipping_heading = false;
    let mut inside_main_heading = false;
    let mut nodes = definition.nodes.iter().peekable();
    while let Some(node) = nodes.next() {
        if let Node::Heading { nodes, level, .. } = node && let Some(Node::Text { value, .. }) = nodes.get(0) {
            // if at a language header
            if *level == 2 {
                // if we're done with the main heading, break
                if inside_main_heading {
                    break;
                }
                // otherwise, mark the start of the main heading
                if *value == lang || !has_lang {
                    inside_main_heading = true;
                }
            }
            // if the header is in our skippable headers list: skip until the next header
            if inside_main_heading {
                skipping_heading = skippable_headers.contains(value);
            }
        }
        if inside_main_heading && !skipping_heading {
            print!("{}", node);
            match node {
                Node::Heading { .. } => println!(),
                _ => match nodes.peek() {
                    Some(Node::Heading { .. }) |
                    Some(Node::OrderedList { .. }) |
                    Some(Node::UnorderedList { .. }) |
                    Some(Node::DefinitionList { .. }) |
                    None => println!(),
                    _ => (),
                }
            }
        }
    }
}
