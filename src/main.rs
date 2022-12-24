use std::env;
use dictionarium;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0 {
        dictionarium::param(String::from("--help"));
    } else {
        let mut words = Vec::<String>::new();
        for word in args {
            if word.len() > 2 && word.get(0..2).unwrap_or_default() == "--" {
                dictionarium::param(word);
            } else {
                words.push(word);
            }
        }

        // we accept multiple words gladly
        for word in words {
            dictionarium::handle(word);
        }
    }
}
