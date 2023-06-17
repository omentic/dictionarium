#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
#![feature(let_chains)]

pub mod correct;
pub mod display;
pub mod lookup;
pub mod state;

// https://github.com/rust-lang/rfcs/issues/1349
const version: &str = env!("CARGO_PKG_VERSION");
const index_path: &str = env!("index_path");
const dictionary_path: &str = env!("dictionary_path");

pub fn handle_word(word: String, state: &state::State) {
    // if lets are kinda clunky
    if let Some(definition) = lookup::lookup(&word).unwrap() {
        display::display(definition, &state);
    } else if let Some(corrected) = correct::correct(&word) {
        println!("Could not find word {}, continuing with {}...", word, corrected);
        if let Some(definition) = lookup::lookup(&corrected).unwrap() {
            display::display(definition, &state);
        } else {
            println!("Could not find corrected word {}.", corrected);
        }
    } else {
        println!("Could not find word {}. Check your spelling?", word);
    }
}

// mut state: State, yet state: &mut State?? huh??
pub fn handle_parameter(word: &str, state: &mut state::State) {
    match word { // todo: extend
        "--help" => {
            println!("dictionarium {}\n", version);
            println!("Usage: dictionarium <word>");
        },
        _ => {
            println!("Unknown flag \"{}\".", word);
        }
    }
}
