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
