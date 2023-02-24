// Copyright 2022 JJ <https://j-james.me>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

#![allow(unused_variables)]

use crate::{Node, Parameter};
use std::fmt::Error;
// why is std::other::Result not usable when i import std::fmt::Result?

impl std::fmt::Display for Node<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Node::CharacterEntity { character, .. } => write!(f, "{}", character),
            Node::Text { value, .. } => write!(f, "{}", value),
            Node::ParagraphBreak { .. } => write!(f, " "),
            Node::HorizontalDivider { .. } => write!(f, "\n------\n"),

            Node::Heading { nodes, level, .. } => {
                match nodes.get(0) {
                    Some(node) => write!(f, "\x1b[1m{}\x1b[0m", node),
                    None => Err(Error),
                }
            },
            Node::Link { text, target, .. } => {
                match text.get(0) {
                    Some(node) => write!(f, "{}", node),
                    None => Err(Error),
                }
            },
            Node::Template { name, parameters, .. } => {
                match name.get(0) {
                    Some(name) =>
                        if let Node::Text { value, .. } = name {
                            write!(f, "{}", handle_template(*value, parameters)?)
                        } else {
                            Err(Error)
                        },
                    None => Err(Error),
                }
            },

            Node::OrderedList { items, .. } => {
                for (i, item) in items.iter().enumerate() {
                    write!(f, "\n{}. ", i+1)?;
                    for node in &item.nodes {
                        write!(f, "{}", node)?
                    }
                }
                return Ok(());
            },
            Node::UnorderedList { items, .. } => {
                for item in items {
                    write!(f, "\n• ")?;
                    for node in &item.nodes {
                        write!(f, "{}", node)?
                    }
                }
                return Ok(());
            },
            Node::DefinitionList { items, .. } => {
                for item in items {
                    write!(f, "\n")?;
                    for node in &item.nodes {
                        write!(f, "{}", node)?
                    }
                    write!(f, " ")?;
                }
                return Ok(());
            },

            Node::ExternalLink { nodes, .. } => {
                for node in nodes {
                    write!(f, "{}", node)?;
                }
                return Ok(());
            },
            // todo: everything below here
            Node::Image { target, text, .. } => Ok(()),
            Node::Table { .. } => todo!(),
            Node::Parameter { default, name, .. } => todo!(),
            Node::Redirect { target, .. } => Ok(()),
            Node::Preformatted { nodes, .. } => Ok(()),
            Node::Category { target, ordinal, .. } => Ok(()),
            Node::Comment { .. } | Node::MagicWord { .. } => Ok(()),
            Node::StartTag { .. } | Node::Tag { .. } | Node::EndTag { .. } => Ok(()),
            Node::Bold { .. } | Node::Italic { .. } | Node::BoldItalic { .. } => Ok(()),
        }
    }
}

// https://en.wiktionary.org/wiki/Wiktionary:Templates
// wow, a function entirely composed of edge cases
fn handle_template(name: &str, parameters: &Vec<Parameter>) -> Result<String, Error> {
    let mut buffer = String::from("");
    match name {
        "m" => {
            let mut index: usize = 1;
            for parameter in parameters.iter().skip(1) {
                if let Some(Node::Text { value, .. }) = parameter.value.get(0) {
                    if let Some(_) = &parameter.name {
                        buffer.push_str(&format!(" (\"{value}\")"));
                    } else if index == 2 {
                        index += 1;
                        buffer.push_str(&format!(" (\"{value}\")"));
                    } else {
                        index += 1;
                        buffer.push_str(&format!("{value}"));
                    }
                }
            }
        },
        "l" | "bor" | "tlb" | "inh" | "der" | "defdate" | "noncog" => buffer.push_str("-"),
        "ux" | "lb" | "l-lite" | "root" => buffer.push_str("-"),
        "a" | "audio" | "non-gloss definition" | "rhymes" | "clipping" | "senseid" => buffer.push_str("-"),
        "quote-journal" | "quote-song" | "quote-book" => buffer.push_str("-"),
        x if x.contains("RQ:") => buffer.push_str("-"),
        x if x.contains("wikidata") => buffer.push_str("-"),
        "w" | "sense" => buffer.push_str(get(parameters, 0)?),
        "alternative case form of" => {
            let form = get(parameters, 1)?;
            buffer.push_str(&format!("Alternative case form of {form}."));
        },
        "syn" => (),
        "suf" => {
            for parameter in parameters.iter().skip(1) {
                let value =
                    if let Some(Node::Text{ value, .. }) = parameter.value.get(0) {
                        value
                    } else {
                        return Err(Error)
                    };
                if let Some(_) = &parameter.name {
                    buffer.push_str(&format!(" (\"{value}\") + "));
                } else {
                    buffer.push_str(&format!("{value}"));
                }
            }
        },
        "desc" | "desctree" => {
            let country_code = get(parameters, 0)?;

            let language = get_language(country_code);
            buffer.push_str(language);
            buffer.push_str(": ");
            buffer.push_str(get(parameters, 1)?);
        },
        "cog" => {
            // let language =
            // let value = get(parameters, 1)?;
            // if value == "-" {

            // } else {

            // }
        },
        "etydate" => {
            let date = get(parameters, 0)?;
            buffer.push_str(&format!("First attested in {date}"))
        },
        x if x.contains("-adj") || x.contains("-noun") || x.contains("-verb")
          || x.contains("-prep") || x.contains("-conj") || x.contains("-interj") => { // todo
            let part = get(parameters, 0).unwrap_or_default();
            if part != "-" {
                buffer.push_str(&format!("{part}"));
            }
        },
        x if x.contains("-IPA") => { // todo
            let ipa = get(parameters, 0)?;
            buffer.push_str(&format!("IPA: {ipa}"));
        },
        "top4" | "bottom" | "head" | "head-lite" | "was wotd" | "wikipedia" => (),
        _ => buffer.push_str(&format!("\x1b[1m--{name}--\x1b[0m")),
    };
    return Ok(buffer);
}

// really missing uniform function call syntax right about now
fn get<'a>(parameters: &'a Vec<Parameter>, index: usize) -> Result<&'a str, Error> {
    if let Some(parameter) = parameters.get(index) {
        if let Some(Node::Text { value, .. }) = parameter.value.get(0) {
            return Ok(value);
        }
    }
    return Err(Error);
}


fn get_language<'a>(country_code: &'a str) -> &'a str {
    return "English";
    // todo: implement necessary parts of isolang
    // if country_code.len() == 3 {
    //     return Language::from_639_3(country_code).map_or(country_code, |x| x.to_name());
    // } else {
    //     return Language::from_639_1(country_code).map_or(country_code, |x| x.to_name());
    // }
}
