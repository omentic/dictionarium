// Copyright 2022 JJ <https://j-james.me>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

#![allow(unused_variables)]

use crate::{Node, Parameter};
use std::fmt::Error;

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
                            write!(f, "")
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
            } ,
            Node::Image { target, text, .. } => {
                write!(f, "{}", target)?;
                for node in text {
                    write!(f, "{}", node)?;
                }
                return Ok(());
            },

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
fn handle_template(name: &str, parameters: &Vec<Parameter>) -> String {
    match name {
        _ => "--template--".to_string(),
    }
}
