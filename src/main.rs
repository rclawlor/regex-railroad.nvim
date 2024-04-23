use neovim_lib::{
    Neovim,
    NeovimApi,
    Session, Value
};
use std::{fs::File, sync::Arc};
use tracing::{info, warn, error};
use tracing_subscriber::{self, layer::SubscriberExt};

struct RegexRailroad {

}

impl RegexRailroad {
    fn new() -> RegexRailroad {
        RegexRailroad{}
    }
}

struct EventHandler {
    nvim: Neovim,
    regex_railroad: RegexRailroad
}

impl EventHandler {
    fn new() -> EventHandler {
        info!("Creating event handler");
        let mut session = match Session::new_parent() {
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
            info!("Received RPC");
            match Message::from(event) {
                Message::Echo => {
                    let msg = value[0].as_str().unwrap();
                    info!("Received echo message: {:?}", value);
                    let buf = self.nvim.get_current_buf().unwrap();
                    let buf_len = buf.line_count(&mut self.nvim).unwrap();
                    let line = self.nvim.get_current_line().unwrap();
                    buf.set_lines(&mut self.nvim, 0, buf_len, true, vec![msg.to_string()])
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
