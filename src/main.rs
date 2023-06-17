use dictionarium::*;

fn main() {
    let mut state = state::State::new();
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() == 0 {
        dictionarium::handle_parameter("--help", &mut state);
        std::process::exit(0);
    }

    let mut words = Vec::<String>::new();
    for word in args {
        if word.get(0..2) == Some("--") {
            dictionarium::handle_parameter(&word, &mut state);
        } else {
            words.push(word);
        }
    }

    // we accept multiple words gladly
    for word in words {
        dictionarium::handle_word(word, &state);
    }
}
