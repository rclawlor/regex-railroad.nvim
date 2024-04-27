use lazy_static::lazy_static;
use neovim_lib::{Neovim, NeovimApi, Session};
use std::{collections::HashMap, fs::File, sync::Arc};
use tracing::{error, info, warn};
use tracing_subscriber::{self, layer::SubscriberExt};

#[derive(Debug)]
struct StringFormat {
    open: Vec<String>,
    escape_character: String,
    literal_string_start: Option<String>,
    literal_string_end: Option<String>,
}

lazy_static! {
    /// Mapping of file extension to the language's string format
    static ref STRING_FORMAT: HashMap<&'static str, StringFormat> = HashMap::from([
        ("py", StringFormat {
                open: ["\""].iter().map(|x| x.to_string()).collect(),
                escape_character: "\\".to_string(),
                literal_string_start: Some("r\"".to_string()),
                literal_string_end: Some("\"".to_string()),
        }),
        ("rs", StringFormat {
                open: ["\""].iter().map(|x| x.to_string()).collect(),
                escape_character: "\\".to_string(),
                literal_string_start: Some("r\"".to_string()),
                literal_string_end: Some("\"".to_string()),
        })
    ]);
}

struct RegexRailroad {}

impl RegexRailroad {
    fn new() -> RegexRailroad {
        RegexRailroad {}
    }

    /// Extract regular expression closes to the cursor
    fn extract_regex(&self, filename: &str, position: u64, line: &str) -> Result<String, String> {
        let extension = self.get_file_extension(filename)?;
        let string_format = self.get_string_format(&extension)?;

        // TODO: what if regex contains escaped string character (e.g. \")
        // Iterate through line and check for literal string
        let _test = r"This is a literal string";
        let windows: Vec<char> = line.chars().collect();
        if string_format.literal_string_start.is_some() {
            let str_start = string_format
                .literal_string_start
                .as_ref()
                .expect("Literal string start already checked with '.is_some()'");
            for (idx, val) in windows.windows(str_start.len()).enumerate() {
                let substr: String = val.into_iter().collect();
                if &substr == str_start {
                    info!(
                        "Found matching string literal start '{}' at index  '{}'",
                        substr, idx
                    );
                }
            }
        }
        let mut idxs = vec![];
        for (idx, _) in line.match_indices("\"") {
            info!("{}", idx);
            idxs.push(idx);
        }
        info!("{:?}", line.len());
        let start = idxs
            .iter()
            .max_by_key(|x| {
                if **x <= position.try_into().unwrap() {
                    **x + 1
                } else {
                    0
                }
            })
            .unwrap();
        let end = *idxs
            .iter()
            .min_by_key(|x| {
                if **x > position.try_into().unwrap() {
                    **x
                } else {
                    line.len()
                }
            })
            .unwrap();
        info!("Start: {}  End: {}", start, end);
        let regex = line.get(*start + 1..end).unwrap();
        info!("{}", regex);
        Ok(regex.to_string())
    }

    /// Parse filename to extract file extension
    fn get_file_extension(&self, filename: &str) -> Result<String, String> {
        match filename.split(".").last() {
            Some(extension) => {
                info!("Found file extension '.{}'", extension);
                Ok(extension.to_string())
            }
            None => Err("File extension not found".to_string()),
        }
    }

    /// Find string characters used for file type
    fn get_string_format(&self, extension: &str) -> Result<&StringFormat, String> {
        match STRING_FORMAT.get(extension) {
            Some(string_format) => {
                info!("Found escape character '{:?}'", string_format);
                Ok(string_format)
            }
            None => Err(format!("File extension .{} not supported", extension)),
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

    fn recv(&mut self) {
        let receiver = self.nvim.session.start_event_loop_channel();
        info!("Started RPC event loop");
        for (event, value) in receiver {
            info!("Received RPC: {:?}", value);
            match Message::from(event) {
                Message::Echo => {
                    // Message sends index, current line
                    let msg = &value[0];
                    // TODO: handle errors if arguments incorrect
                    let filename = msg[0].as_str().unwrap();
                    let position = msg[1].as_u64().unwrap();
                    let current_line = msg[2].as_str().unwrap();
                    let regex =
                        match self
                            .regex_railroad
                            .extract_regex(filename, position, current_line)
                        {
                            Ok(regex) => regex,
                            Err(e) => {
                                error!("{}", e);
                                panic!("{}", e)
                            }
                        };
                    info!("Received echo message: {} {:?}", position, current_line);
                    let buf = self.nvim.get_current_buf().unwrap();
                    let buf_len = buf.line_count(&mut self.nvim).unwrap();
                    buf.set_lines(&mut self.nvim, 0, buf_len, true, vec![regex])
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
    Unknown(String),
}

impl From<String> for Message {
    fn from(event: String) -> Self {
        match &event[..] {
            "echo" => Message::Echo,
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
