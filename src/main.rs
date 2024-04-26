use lazy_static::lazy_static;
use neovim_lib::{
    Neovim,
    NeovimApi,
    Session
};
use std::{collections::HashMap, fs::File, sync::Arc};
use tracing::{info, warn, error};
use tracing_subscriber::{self, layer::SubscriberExt};


lazy_static! {
    /// Mapping of file extension to the language's escape character
    static ref ESCAPE_CHARACTER: HashMap<&'static str, &'static str> = HashMap::from([
        ("py", "\\"),
        ("rs", "\\")
    ]);
}


lazy_static! {
    /// Mapping of file extension to the language's literal string definition
    static ref LITERAL_STRING: HashMap<&'static str, &'static str> = HashMap::from([
        ("py", "r"),
        ("rs", "r")
    ]);
}
struct RegexRailroad {

}

impl RegexRailroad {
    fn new() -> RegexRailroad {
        RegexRailroad{}
    }

    /// Extract regular expression closes to the cursor
    fn extract_regex(&self, filename: &str, position: u64, line: &str) -> Result<String, String> {
        // Find extension of file if it exists
        let extension = match filename.split(".").last() {
            Some(extension) => extension,
            None => {
                error!("File extension not found");
                return Err("File extension not found".to_string())
            }
        };
        let escape_character = match ESCAPE_CHARACTER.get(extension) {
            Some(escape_character) => escape_character,
            None => {
                return Err(format!("File extension .{} not supported", extension));
            }
        };
        info!("Found file extension {} with escape character {}", extension, escape_character);
        // TODO: what if regex contains escaped string character (e.g. \")
        let mut idxs = vec![];
        for (idx, _) in line.match_indices("\"") {
            info!("{}", idx);
            idxs.push(idx);
        }
        info!("{:?}", line.len());
        let start = idxs.iter().max_by_key(
            |x| if **x <= position.try_into().unwrap() {
                **x + 1
            } else {
                0
           }
        ).unwrap();
        let end = *idxs.iter().min_by_key(
            |x| if **x > position.try_into().unwrap() {
                **x
            } else {
                line.len()
            }
        ).unwrap();
        info!("Start: {}  End: {}", start, end);
        let regex = line.get(*start + 1..end).unwrap();
        info!("{}", regex);
        Ok(regex.to_string())
    }
}

struct EventHandler {
    nvim: Neovim,
    regex_railroad: RegexRailroad
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

        EventHandler{ nvim, regex_railroad }
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
                    let regex = match self.regex_railroad.extract_regex(filename, position, current_line) {
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
                },
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
    Unknown(String)
}

impl From<String> for Message {
    fn from(event: String) -> Self {
        match &event[..] {
            "echo" => Message::Echo,
            _ => Message::Unknown(event)
        }
    }
}

fn main() {
     // A layer that logs events to a file.
    let file = File::create("debug.log");
    let file = match file  {Ok(file) => file,Err(error) => panic!("Error: {:?}",error),};
    let subscriber = tracing_subscriber::Registry::default()
        .with(
            tracing_subscriber::fmt::Layer::default()
            .pretty()
            .with_ansi(false)
            .with_writer(Arc::new(file))
        );
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let mut event_handler = EventHandler::new();

    event_handler.recv();
}
