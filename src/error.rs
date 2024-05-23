use std::convert::From;


#[derive(Clone, Copy, Debug)]
pub enum Error {
    CharacterRange(char, char),
    StringIterator(char, char),
    RepetitionValue(char)
}


impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CharacterRange(a, b) => write!(f, "Invalid character range [{}-{}]", a, b),
            Self::StringIterator(a, b) => write!(f, "Unknown error when parsing character '{}', expected character '{}'", a, b),
            Self::RepetitionValue(a) => write!(f, "Expected number for repetition amount, received '{}'", a)
        }
    }
}


impl From<Error> for std::string::String {
    fn from(value: Error) -> Self {
        format!("{}", value)
    }
}
