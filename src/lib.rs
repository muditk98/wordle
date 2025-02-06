use colored::{ColoredString, Colorize};

pub enum Correctness {
    /// Green
    Correct(char),
    /// Yellow
    Misplaced(char),
    /// Gray
    Incorrect(char),
}

impl Correctness {
    pub fn colored_string(&self) -> ColoredString {
        match self {
            Correctness::Correct(char) => ColoredString::from(char.to_string()).on_green().bold(),
            Correctness::Misplaced(char) => ColoredString::from(char.to_string()).on_yellow().bold(),
            Correctness::Incorrect(char) => ColoredString::from(char.to_string()).on_black().bold(),
        }
    }
}