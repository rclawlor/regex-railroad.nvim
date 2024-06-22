use rmpv::Value;
use rsnvim::{api::Nvim, handler::RequestHandler};
use std::{fs::File, sync::Arc};
use tracing::{info, warn};
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


struct ReqHandler {
    regex_railroad: RegexExtractor
}

impl ReqHandler {
    pub fn new() -> ReqHandler {
        let regex_railroad = RegexExtractor::new();
        ReqHandler { regex_railroad }
    }

    /// Retrieve filename and node text from RPC arguments
    fn parse_rpc_args(&self, value: Vec<Value>) -> Result<(String, String), Error> {
        let msg = &value[0];
        let filename = msg[0].as_str().expect("Filename is the first argument of the Lua RPC");
        let node = msg[1].as_str().expect("Node is the second argument of the Lua RPC");
        info!("Received message: {}", node);

        Ok((filename.to_string(), node.to_string()))
    }

    fn regexrailroad(&self, params: Vec<Value>) -> Result<Value, Error> {
        // Handle RPC arguments
        let (filename, node) = self.parse_rpc_args(params)?;

        // Obtain regular expression from received text
        let regex = self.regex_railroad.get_regex(&filename, &node)?;

        // Parse and render regular expression
        let mut parser = RegExParser::new(&regex);
        let parsed_regex = parser.parse()?;
        info!("Parsed regular expression: {:?}", parsed_regex);

        // Generate and render diagram
        let diagram = RailroadRenderer::generate_diagram(&parsed_regex)?;
        info!("Successfully generated diagram: {:?}", diagram);
        let text = RailroadRenderer::render_diagram(&diagram)?;
        info!("Successfully rendered diagram");

        Ok(Value::Map(vec![
            (
                Value::from("text"), 
                Value::from(text.iter().map(|x| Value::from(x.as_str())).collect::<Vec<Value>>())
            ),
            (Value::from("width"), Value::from(text[0].chars().count())),
            (Value::from("height"), Value::from(text.len()))
        ]))
    }

    fn railroadtext(&self, params: Vec<Value>) -> Result<Value, Error> {
        // Handle RPC arguments
        let (filename, node) = self.parse_rpc_args(params)?;

        // Obtain regular expression from received text
        let regex = self.regex_railroad.get_regex(&filename, &node)?;

        // Parse and render regular expression
        let mut parser = RegExParser::new(&regex);
        let parsed_regex = parser.parse()?;
        info!("Parsed regular expression: {:?}", parsed_regex);
        let (text, _highlight) = TextRenderer::render_text(&parsed_regex)?;
        info!("Successfully rendered text");

        Ok(Value::Map(vec![
            (
                Value::from("text"), 
                Value::from(text.iter().map(|x| Value::from(x.as_str())).collect::<Vec<Value>>())
            ),
            (Value::from("width"), Value::from(text[0].chars().count())),
            (Value::from("height"), Value::from(text.len()))
        ]))
    }
}

impl RequestHandler for ReqHandler {
    fn handle_request(
            &self,
            _msgid: u64,
            method: String,
            params: Vec<Value>,
    ) -> Result<Value, rsnvim::error::Error> {
        match method.as_str() {
            "regexrailroad" => {
                info!("RegexRailroad command received");
                Ok(self.regexrailroad(params).unwrap())
            },
            "regextext" => {
                info!("RegexText command received");
                Ok(self.railroadtext(params).unwrap())
            }, 

            unknown => {
                warn!("Unknown command: {}", unknown);
                Ok(Value::from(""))
            }
        }
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

    let mut nvim = Nvim::from_parent().unwrap();
    let handler = ReqHandler::new();
    nvim.start_event_loop(Some(Box::new(handler)), None);

    loop {}
}
