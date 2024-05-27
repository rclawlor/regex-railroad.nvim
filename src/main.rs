use lazy_static::lazy_static;
use neovim_lib::{Neovim, NeovimApi, Session, Value};
use std::{collections::HashMap, fmt::Display, fs::File, sync::Arc};
use tracing::{error, info, warn};
use tracing_subscriber::{self, layer::SubscriberExt};

use crate::{
    error::Error,
    parser::RegExParser,
    railroad::RailroadRenderer,
    text::TextRenderer
};

pub mod error;
pub mod parser;
pub mod railroad;
pub mod text;

const _TEST_LITERAL: &str = r"This is a literal string";
const _TEST_NORMAL: &str = "(a|b)+hello(cd){5,}";
const _TEST_CHARACTER: &str = "[^aoeu_0-a]";
const _TEST_OPTIONS: &str = "(ab|bc|cd)";

#[derive(Debug)]
struct StringFormat {
    string_character: Vec<String>,
    _escape_character: String,
    literal_string_start: Option<Vec<String>>,
    literal_string_end: Option<Vec<String>>,
}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub enum Language {
    Python,
    Rust,
    Unknown(String),
    None,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
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
    fn get_regex<'a>(&'a self, filename: &str, text: &'a str) -> Result<String, Error> {
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
            Ok(self.strip_string_start_end(text, str_start, str_end))
        } else {
            // Not a literal string, lets check for a normal string
            let str_character = string_format.string_character.as_ref();
            Ok(self.strip_string_start_end(text, str_character, str_character))
        }
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

    /// Retrieve filename and node text from RPC arguments
    fn parse_rpc_args(&self, value: Vec<Value>) -> Result<(String, String), Error> {
        // TODO: handle errors if arguments incorrect
        let msg = &value[0];
        let filename = msg[0].as_str().unwrap();
        let node = msg[1].as_str().unwrap();
        info!("Received message: {}", node);

        Ok((filename.to_string(), node.to_string()))
    }

    fn recv(&mut self) -> Result<(), Error> {
        let receiver = self.nvim.session.start_event_loop_channel();
        info!("Started RPC event loop");
        for (event, value) in receiver {
            match Message::from(event) {
                Message::RegexRailroad => {
                    // Handle RPC arguments
                    let (filename, node) = self.parse_rpc_args(value)?;

                    // Obtain regular expression from received text
                    let regex = self.regex_railroad.get_regex(&filename, &node)?;
                    self.send_msg(&regex);

                    // Parse and render regular expression
                    let mut parser = RegExParser::new(&regex);
                    let parsed_regex = parser.parse()?;
                    info!("Parsed regular expression: {:?}", parsed_regex);
                    let diagram = RailroadRenderer::render_diagram(&parsed_regex)?;
                    let diagram = &diagram[0];
                    info!("Successfully rendered diagram");

                    // Create neovim buffer and window
                    let buf = match self.nvim.call_function(
                        "nvim_create_buf",
                        vec![Value::Boolean(false), Value::Boolean(true)],
                    ) {
                        Ok(buf) => buf,
                        Err(e) => {
                            error!("Error creating buffer: {}", e);
                            panic!();
                        }
                    };
                    let win_opts = Value::Map(vec![
                        // Increase height and width by 2 for whitespace padding
                        (
                            Value::from("width"),
                            Value::from(diagram.iter().max_by_key(|x| x.len()).unwrap().len() + 2),
                        ),
                        (Value::from("height"), Value::from(diagram.len() + 2)),
                        // TODO: allow styles to be set by the user
                        (Value::from("style"), Value::from("minimal")),
                        (Value::from("relative"), Value::from("cursor")),
                        // Slight offset for readability
                        (Value::from("row"), Value::from(1)),
                        (Value::from("col"), Value::from(0)),
                    ]);
                    match self.nvim.call_function(
                        "nvim_open_win",
                        vec![buf.clone(), Value::Boolean(true), win_opts],
                    ) {
                        Ok(win) => {
                            info!("Opened window with ID {}", win);
                            win
                        }
                        Err(e) => {
                            error!("Error creating window: {}", e);
                            panic!();
                        }
                    };

                    match self.nvim.call_function(
                        "nvim_buf_set_lines",
                        vec![
                            buf.clone(),
                            Value::from(1),
                            Value::from(-1),
                            Value::from(true),
                            diagram.iter().map(|x| format!(" {} ", x)).collect(),
                        ],
                    ) {
                        Ok(_) => (),
                        Err(e) => error!("Error setting buffer lines: {}", e),
                    };
                }
                Message::RegexText => {
                    // Handle RPC arguments
                    let (filename, node) = self.parse_rpc_args(value)?;

                    // Obtain regular expression from received text
                    let regex = self.regex_railroad.get_regex(&filename, &node)?;
                    self.send_msg(&regex);

                    // Parse and render regular expression
                    let mut parser = RegExParser::new(&regex);
                    let parsed_regex = parser.parse()?;
                    info!("Parsed regular expression: {:?}", parsed_regex);
                    let (text, highlight) = TextRenderer::render_text(&parsed_regex)?;
                    info!("Successfully rendered text");

                    // Create neovim buffer and window
                    let buf = match self.nvim.call_function(
                        "nvim_create_buf",
                        vec![Value::Boolean(false), Value::Boolean(true)],
                    ) {
                        Ok(buf) => buf,
                        Err(e) => {
                            error!("Error creating buffer: {}", e);
                            panic!();
                        }
                    };
                    let win_opts = Value::Map(vec![
                        // Increase height and width by 2 for whitespace padding
                        (
                            Value::from("width"),
                            Value::from(text.iter().max_by_key(|x| x.len()).unwrap().len() + 2),
                        ),
                        (Value::from("height"), Value::from(text.len() + 2)),
                        // TODO: allow styles to be set by the user
                        (Value::from("style"), Value::from("minimal")),
                        (Value::from("relative"), Value::from("cursor")),
                        // Slight offset for readability
                        (Value::from("row"), Value::from(1)),
                        (Value::from("col"), Value::from(0)),
                    ]);
                    match self.nvim.call_function(
                        "nvim_open_win",
                        vec![buf.clone(), Value::Boolean(true), win_opts],
                    ) {
                        Ok(win) => {
                            info!("Opened window with ID {}", win);
                            win
                        }
                        Err(e) => {
                            error!("Error creating window: {}", e);
                            panic!();
                        }
                    };
                    match self.nvim.call_function(
                        "nvim_buf_set_lines",
                        vec![
                            buf.clone(),
                            Value::from(1),
                            Value::from(-1),
                            Value::from(true),
                            text.iter().map(|x| format!(" {} ", x)).collect(),
                        ],
                    ) {
                        Ok(_) => (),
                        Err(e) => error!("Error setting buffer lines: {}", e),
                    };

                    // Highlight keywords in text
                    for (line, start, end) in highlight.iter() {
                        self.nvim
                            .call_function(
                                "nvim_buf_add_highlight",
                                vec![
                                    buf.clone(),
                                    Value::from(0),
                                    Value::from("RegexHighlight"),
                                    // 0/1 indexing fun
                                    Value::from(1 + *line),
                                    Value::from(1 + *start),
                                    Value::from(1 + *end),
                                ],
                            )
                            .unwrap();
                    }
                    info!("Finished");
                }
                Message::Unknown(unknown) => {
                    self.nvim
                        .command(&format!("echo \"Unknown command: {}\"", unknown))
                        .unwrap();
                    warn!("Unknown command: {}", unknown);
                }
            }
        }
        Ok(())
    }

    /// Send message to the command line
    fn send_msg(&mut self, msg: &String) {
        self.nvim.command(&format!("echo \"{}\"", msg)).unwrap();
    }

    /// Echo error to the command line and exit
    fn send_error(&mut self, error: Error) -> ! {
        error!("{}", error);
        self.nvim
            .command(&format!(
                "echohl ErrorMsg | echo \"{}\" | echohl None",
                error
            ))
            .unwrap();
        panic!()
    }
}

enum Message {
    RegexRailroad,
    RegexText,
    Unknown(String),
}

impl From<String> for Message {
    fn from(event: String) -> Self {
        match &event[..] {
            "regexrailroad" => Message::RegexRailroad,
            "regextext" => Message::RegexText,
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

    match event_handler.recv() {
        Ok(_) => (),
        Err(e) => event_handler.send_error(e),
    }
}
