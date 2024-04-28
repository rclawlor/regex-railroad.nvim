use lazy_static::lazy_static;
use neovim_lib::{Neovim, NeovimApi, Session};
use std::{collections::HashMap, fs::File, sync::Arc};
use tracing::{error, info, warn};
use tracing_subscriber::{self, layer::SubscriberExt};

pub mod parser;
pub mod renderer;

const _TEST_LITERAL: &str = r"This is a literal string";
const _TEST_NORMAL: &str = "This is a normal string";

#[derive(Debug)]
struct StringFormat {
    string_character: Vec<String>,
    _escape_character: String,
    literal_string_start: Option<Vec<String>>,
    literal_string_end: Option<Vec<String>>,
}

#[derive(Eq, Hash, PartialEq, Debug)]
enum Language {
    Python,
    Rust,
    Unknown(String),
    None,
}

impl Language {
    fn from_filename(filename: &str) -> Language {
        match filename.split('.').last() {
            Some(extension) => {
                info!("Found file extension '.{}'", extension);
                match extension {
                    "py" => Language::Python,
                    "rs" => Language::Rust,
                    _ => Language::Unknown(extension.to_string()),
                }
            }
            None => Language::None,
        }
    }
}

lazy_static! {
    /// Mapping of file extension to the language's string format
    static ref STRING_FORMAT: HashMap<Language, StringFormat> = HashMap::from([
        (Language::Python, StringFormat {
                string_character: ["\""].iter().map(|x| x.to_string()).collect(),
                _escape_character: "\\".to_string(),
                literal_string_start: Some(["r\""].iter().map(|x| x.to_string()).collect()),
                literal_string_end: Some(["\""].iter().map(|x| x.to_string()).collect()),
        }),
        (Language::Rust, StringFormat {
                string_character: ["\""].iter().map(|x| x.to_string()).collect(),
                _escape_character: "\\".to_string(),
                literal_string_start: Some(["r\""].iter().map(|x| x.to_string()).collect()),
                literal_string_end: Some(["\""].iter().map(|x| x.to_string()).collect()),
        })
    ]);
}

struct RegexRailroad {}

impl RegexRailroad {
    /// Create new instance of RegexRailroad
    fn new() -> RegexRailroad {
        RegexRailroad {}
    }

    /// Find string characters used for file type
    fn get_string_format(&self, language: &Language) -> Result<&StringFormat, String> {
        match STRING_FORMAT.get(language) {
            Some(string_format) => {
                info!("Found escape character '{:?}'", string_format);
                Ok(string_format)
            }
            None => Err(format!("File extension not supported: {:?}", language)),
        }
    }

    /// Checks if start and end of text is consistent with the language's string specification
    fn check_string_start_end(&self, text: &str, start: &[String], end: &[String]) -> bool {
        // Ensure text is long enough to contain start and end characters
        let text_len = text.len();

        let mut start_present = false;
        let mut end_present = false;

        for s in start.iter() {
            if text_len > s.len() {
                info!("Start: {} - {:?}", &text[0..s.len()], s);
                start_present = start_present || s.contains(&text[0..start.len()].to_string());
            }
        }
        for e in end.iter() {
            if text_len > e.len() {
                info!("End: {} - {:?}", &text[text_len - end.len()..], end);
                end_present =
                    end_present || end.contains(&text[text_len - end.len()..].to_string());
            }
        }
        start_present && end_present
    }

    /// Check if text is a regular expression based on language
    fn is_regex(&self, filename: &str, text: &str) -> Result<bool, String> {
        let language = Language::from_filename(filename);
        let string_format = self.get_string_format(&language)?;

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
            if self.check_string_start_end(text, str_start, str_end) {
                return Ok(true);
            }
        }
        // Not a literal string, lets check for a normal string
        else {
            let str_character = string_format.string_character.as_ref();
            if self.check_string_start_end(text, str_character, str_character) {
                return Ok(true);
            }
        };

        Ok(false)
    }
}

struct EventHandler {
    nvim: Neovim,
    regex_railroad: RegexRailroad,
}

impl EventHandler {
    fn new() -> EventHandler {
        info!("Creating event handler");
        let session = match Session::new_parent() {
            Ok(session) => session,
            Err(e) => {
                error!("Couldn't create neovim session {}", e);
                panic!("Couldn't create neovim session {}", e);
            }
        };

        let nvim = Neovim::new(session);
        let regex_railroad = RegexRailroad::new();

        EventHandler {
            nvim,
            regex_railroad,
        }
    }

    fn recv(&mut self) {
        let receiver = self.nvim.session.start_event_loop_channel();
        info!("Started RPC event loop");
        for (event, value) in receiver {
            info!("Received RPC: {:?}", value);
            match Message::from(event) {
                Message::Echo => {
                    let msg = &value[0];
                    let text = msg[0].as_str().unwrap();
                    info!("ECHO: {}", text);
                }
                Message::ParseRegex => {
                    // Message sends index, current line
                    let msg = &value[0];
                    // TODO: handle errors if arguments incorrect
                    let filename = msg[0].as_str().unwrap();
                    let row = msg[1].as_u64().unwrap();
                    let col = msg[2].as_u64().unwrap();
                    let _len = msg[3].as_u64().unwrap();
                    let text = msg[4].as_str().unwrap();
                    info!("Received message: {}", text);
                    let is_regex = match self.regex_railroad.is_regex(filename, text) {
                        Ok(is_regex) => is_regex,
                        Err(e) => {
                            error!("{}", e);
                            panic!("{}", e)
                        }
                    };
                    info!("Received echo message: ({}, {}) {:?}", row, col, text);
                    let buf = self.nvim.get_current_buf().unwrap();
                    let buf_len = buf.line_count(&mut self.nvim).unwrap();
                    buf.set_lines(
                        &mut self.nvim,
                        0,
                        buf_len,
                        true,
                        vec![format!("{}", is_regex)],
                    )
                    .unwrap();
                }
                Message::Unknown(unknown) => {
                    self.nvim
                        .command(&format!("echo \"Unknown command: {}\"", unknown))
                        .unwrap();
                    warn!("Unknown command: {}", unknown);
                }
            }
        }
    }
}

enum Message {
    Echo,
    ParseRegex,
    Unknown(String),
}

impl From<String> for Message {
    fn from(event: String) -> Self {
        match &event[..] {
            "echo" => Message::Echo,
            "parseregex" => Message::ParseRegex,
            _ => Message::Unknown(event),
        }
    }
}

fn main() {
    // A layer that logs events to a file.
    let file = File::create("debug.log");
    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Error: {:?}", error),
    };
    let subscriber = tracing_subscriber::Registry::default().with(
        tracing_subscriber::fmt::Layer::default()
            .pretty()
            .with_ansi(false)
            .compact()
            .with_writer(Arc::new(file)),
    );
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let mut event_handler = EventHandler::new();

    event_handler.recv();
}
