use std::convert::From;

use crate::Language;

#[derive(Clone, Debug)]
pub enum Error {
    CharacterRange(char, char),
    StringIterator(char, char),
    RepetitionValue(char),
    FileType(Language),
    UnsupportedLanguage(Language),
    InvalidString(Language, String),
    InvalidParsing,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CharacterRange(a, b) => write!(f, "Invalid character range [{}-{}]", a, b),
            Self::StringIterator(a, b) => write!(
                f,
                "Unknown error when parsing character '{}', expected character '{}'",
                a, b
            ),
            Self::RepetitionValue(a) => {
                write!(f, "Expected number for repetition amount, received '{}'", a)
            }
            Self::FileType(a) => write!(f, "Unsupported file type {}", a),
            Self::UnsupportedLanguage(a) => write!(f, "Unsupported language {}", a),
            Self::InvalidString(lang, string) => write!(f, "Invalid {} string {}", lang, string),
            Self::InvalidParsing => write!(f, "Invalid parsing"),
        }
    }
}

impl From<Error> for std::string::String {
    fn from(value: Error) -> Self {
        format!("{}", value)
    }
}
