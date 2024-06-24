use lazy_static::lazy_static;
use std::{collections::HashMap, fmt::Display};
use tracing::info;

use crate::error::Error;


#[derive(Debug)]
pub struct StringFormat {
    string_character: Vec<String>,
    escape_character: char,
    literal_string_start: Option<Vec<String>>,
    literal_string_end: Option<Vec<String>>,
}

impl StringFormat {
    pub fn escape_char(&self) -> char {
        self.escape_character
    }
}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub enum Language {
    Python,
    Rust,
    Javascript,
    Unknown(String),
    None,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Language {
    pub fn from_filename(filename: &str) -> Language {
        match filename.split('.').last() {
            Some(extension) => {
                info!("Found file extension '.{}'", extension);
                match extension {
                    "py" => Language::Python,
                    "rs" => Language::Rust,
                    "js" => Language::Javascript,
                    _ => Language::Unknown(extension.to_string()),
                }
            }
            None => Language::None,
        }
    }
}

lazy_static! {
    /// Mapping of file extension to the language's string format
    pub static ref STRING_FORMAT: HashMap<Language, StringFormat> = HashMap::from([
        (Language::Python, StringFormat {
                string_character: ["\""].iter().map(|x| x.to_string()).collect(),
                escape_character: '\\',
                literal_string_start: Some(["r\""].iter().map(|x| x.to_string()).collect()),
                literal_string_end: Some(["\""].iter().map(|x| x.to_string()).collect()),
        }),
        (Language::Rust, StringFormat {
                string_character: ["\""].iter().map(|x| x.to_string()).collect(),
                escape_character: '\\',
                literal_string_start: Some(["r\""].iter().map(|x| x.to_string()).collect()),
                literal_string_end: Some(["\""].iter().map(|x| x.to_string()).collect()),
        }),
        (Language::Javascript, StringFormat {
                string_character: ["\""].iter().map(|x| x.to_string()).collect(),
                escape_character: '\\',
                literal_string_start: None,
                literal_string_end: None,
        })
    ]);
}

#[derive(Default)]
pub struct RegexExtractor {}

impl RegexExtractor {
    /// Create new instance of RegexExtractor
    pub fn new() -> RegexExtractor {
        RegexExtractor {}
    }

    /// Find string characters used for file type
    fn get_string_format(&self, language: &Language) -> Result<&StringFormat, Error> {
        match STRING_FORMAT.get(language) {
            Some(string_format) => {
                info!("Found escape character '{:?}'", string_format);
                Ok(string_format)
            }
            None => Err(Error::UnsupportedLanguage(language.clone())),
        }
    }

    /// Checks if start/end of text is consistent with the language's string specification
    /// and strips the start/end characters
    fn strip_string_start_end(&self, text: &str, start: &[String], end: &[String]) -> String {
        // Ensure text is long enough to contain start and end characters
        let text_len = text.len();

        let mut max_start_len = 0;
        let mut max_end_len = 0;

        for s in start.iter() {
            if text_len > s.len() {
                info!("Start: {} - {:?}", &text[0..s.len()], s);
                if s.contains(&text[0..s.len()].to_string()) {
                    max_start_len = std::cmp::max(max_start_len, s.len());
                }
            }
        }
        for e in end.iter() {
            if text_len > e.len() {
                info!("End: {} - {:?}", &text[text_len - end.len()..], end);
                if end.contains(&text[text_len - e.len()..].to_string()) {
                    max_end_len = std::cmp::max(max_end_len, e.len());
                }
            }
        }
        text[max_start_len..text_len - max_end_len].to_string()
    }

    /// Check if text is a regular expression based on language
    pub fn get_regex<'a>(&'a self, language: &Language, text: &'a str) -> Result<String, Error> {
        let string_format = self.get_string_format(language)?;

        // Iterate through line and check for literal string
        if string_format.literal_string_start.is_some()
            && string_format.literal_string_end.is_some()
        {
            let str_start = string_format
                .literal_string_start
                .as_ref()
                .expect("Literal string start already checked with '.is_some()'");
            let str_end = string_format
                .literal_string_end
                .as_ref()
                .expect("Literal string end already checked with '.is_some()'");
            // Ensure text is long enough to be a valid regex
            Ok(self.strip_string_start_end(text, str_start, str_end))
        } else {
            // Not a literal string, lets check for a normal string
            let str_character = string_format.string_character.as_ref();
            Ok(self.strip_string_start_end(text, str_character, str_character))
        }
    }
}
