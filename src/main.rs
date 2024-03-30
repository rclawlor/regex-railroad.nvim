use neovim_lib::{
    Neovim,
    NeovimApi,
    Session
};

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
        let mut session = match Session::new_parent() {
            Ok(session) => session,
            Err(e) => panic!("Couldn't create neovim session {}", e)
        };

        let nvim = Neovim::new(session);
        let regex_railroad = RegexRailroad::new();

        EventHandler{ nvim, regex_railroad }
    }

    fn recv(&mut self) {
        let receiver = self.nvim.session.start_event_loop_channel();

        for (event, value) in receiver {
            match Message::from(event) {
                Message::Echo => {
                    let mut nums = value.iter();
                    let p = nums.next().unwrap().as_i64().unwrap();
                    self.nvim
                        .command(&format!("echo \"ECHO: {}", p))
                        .unwrap();
                },
                Message::Unknown(unknown) => {

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
    let mut event_handler = EventHandler::new();

    event_handler.recv();
}
