use neovim_lib::{Neovim, NeovimApi, Session, Value};
use std::{fs::File, sync::Arc};
use tracing::{error, info, warn};
use tracing_subscriber::{self, layer::SubscriberExt};

use crate::{
    error::Error,
    extract::RegexExtractor,
    parser::RegExParser,
    railroad::renderer::RailroadRenderer,
    text::TextRenderer
};

pub mod error;
pub mod extract;
pub mod parser;
pub mod railroad;
pub mod text;
pub mod test;


struct EventHandler {
    nvim: Neovim,
    regex_railroad: RegexExtractor,
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
        let regex_railroad = RegexExtractor::new();

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
                    let parsed_regex = match parser.parse() {
                        Ok(parsed_regex) => parsed_regex,
                        Err(e) => {
                            error!("Error parsing regex: {}", e);
                            panic!()
                        }
                    };
                    info!("Parsed regular expression: {:?}", parsed_regex);
                    let diagram = match RailroadRenderer::generate_diagram(&parsed_regex) {
                        Ok(diagram) => diagram,
                        Err(e) => {
                            error!("{}", e);
                            panic!()
                        }
                    };
                    info!("Successfully generated diagram: {:?}", diagram);
                    let text = match RailroadRenderer::render_diagram(&diagram) {
                        Ok(text) => text,
                        Err(e) => {
                            error!("{}", e);
                            panic!()
                        }
                    };
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
                            Value::from(text[0].chars().count() + 2),
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
        Err(e) => {
            error!("Error: {}", e);
            event_handler.send_error(e)
        },
    }
}
