#![allow(unused_variables)]

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

    // impl display on that shit then
    // definition.display(&state.lang);

    // this is really quite terrible code
    if !display_language(&definition, &state.lang) {
        display_language(&definition, "");
    }
}

// no overloading?? O_O
// matching on an enum of structs SUCKS
// functions as parameters is too hard
pub fn display_language(definition: &Output, lang: &str) -> bool {
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
            } else if *level == 2 && *value == lang {
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

