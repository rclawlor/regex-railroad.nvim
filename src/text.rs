use std::ops::Deref;
use tracing::{error, info};

use crate::{
    error::Error,
    parser::{AnchorType, CharacterType, RegEx, RepetitionType},
};

type HighlightRegion = (usize, usize, usize);

pub struct TextRenderer {}

impl Default for TextRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl TextRenderer {
    pub fn new() -> TextRenderer {
        TextRenderer {}
    }

    pub fn render_text(tree: &RegEx) -> Result<(Vec<String>, Vec<HighlightRegion>), Error> {
        let mut text = Vec::new();
        let mut highlight = Vec::new();
        info!("Rendering text...");
        match tree {
            RegEx::Element(a) => {
                for i in a.iter() {
                    info!("{:?}", i);
                    match **i {
                        RegEx::Terminal(_) => {
                            info!("Terminal");
                            let msg = "EXACTLY:".to_string();
                            highlight.push((text.len(), 0, msg.len()));
                            text.push(msg);
                            let msg = Self::render_text_element(i, &mut text, &mut highlight)?;
                            text.push(format!("    {}", msg));
                        }
                        _ => {
                            let newmsg = Self::render_text_element(i, &mut text, &mut highlight)?;
                            for submsg in newmsg.split('\n') {
                                text.push(submsg.to_string());
                            }
                        }
                    }
                }
            }
            RegEx::Alternation(a) => {
                let mut msg =
                    Self::render_text_element(a.first().unwrap(), &mut text, &mut highlight)?
                        .to_string();
                for i in a.iter().skip(1) {
                    msg = format!(
                        "{} OR {}",
                        msg,
                        Self::render_text_element(i, &mut text, &mut highlight)?
                    );
                }
                text.push(msg);
            }
            other => {
                error!("Expected RegEx::Element, received {:?}", other);
                panic!("Expected RegEx::Element, received {:?}", other);
            }
        }
        Ok((text, highlight))
    }

    fn render_text_element(
        tree: &RegEx,
        text: &mut Vec<String>,
        highlight: &mut Vec<HighlightRegion>,
    ) -> Result<String, Error> {
        info!("Rendering text element...");
        match tree {
            RegEx::Anchor(a) => {
                match a {
                    AnchorType::Start => Ok(String::from("Start")),
                    AnchorType::End => Ok(String::from("End")),
                    _ => panic!()
                }
            },
            RegEx::Element(a) => {
                let mut msg = "".to_string();
                for i in a.iter() {
                    msg = format!(
                        "{}{}",
                        msg,
                        Self::render_text_element(i.deref(), text, highlight)?
                    )
                }
                Ok(msg)
            }
            RegEx::Repetition(t, a) => match t {
                RepetitionType::ZeroOrOne => Ok(format!(
                    "0 OR 1:\n    {}",
                    Self::render_text_element(a, text, highlight)?
                )),
                RepetitionType::OrMore(n) => {
                    let msg = format!("{} OR MORE:", n);
                    highlight.push((text.len(), 0, msg.len()));
                    Ok(format!(
                        "{}\n    {}",
                        msg,
                        Self::render_text_element(a, text, highlight)?
                    ))
                }
                RepetitionType::Exactly(n) => {
                    let msg = format!("EXACTLY {}:", n);
                    highlight.push((text.len(), 0, msg.len()));
                    Ok(format!(
                        "{}\n    {}",
                        msg,
                        Self::render_text_element(a, text, highlight)?
                    ))
                }
                RepetitionType::Between(n, m) => {
                    let msg = format!("BETWEEN {} AND {}:", n, m);
                    highlight.push((text.len(), 0, msg.len()));
                    Ok(format!(
                        "{}\n    {}",
                        msg,
                        Self::render_text_element(a, text, highlight)?
                    ))
                }
            },
            RegEx::Alternation(a) => {
                let mut msg =
                    Self::render_text_element(a.first().unwrap(), text, highlight)?.to_string();
                for i in a.iter().skip(1) {
                    msg = format!(
                        "{} OR {}",
                        msg,
                        Self::render_text_element(i, text, highlight)?
                    );
                }
                Ok(msg)
            }
            RegEx::Character(a) => match a {
                CharacterType::Any(b) => {
                    let mut msg = String::from("MATCH:\n");
                    highlight.push((text.len(), 0, msg.len()));
                    for i in b.iter() {
                        msg = format!("{} {}", msg, Self::render_character(i)?)
                    }
                    Ok(msg)
                }
                CharacterType::Not(b) => {
                    let mut msg = String::from("DON'T MATCH:\n");
                    highlight.push((text.len(), 0, msg.len()));
                    for i in b.iter() {
                        msg = format!("{}{}", msg, Self::render_character(i)?)
                    }
                    Ok(msg)
                }
                _ => Err(Error::InvalidParsing),
            },
            RegEx::Terminal(a) => Ok(format!("'{}'", a)),
            RegEx::Capture(name, a) => panic!()
        }
    }

    fn render_character(character: &CharacterType) -> Result<String, Error> {
        match character {
            CharacterType::Between(a, b) => Ok(format!(
                "[{}-{}]",
                Self::render_character(a)?,
                Self::render_character(b)?
            )),
            CharacterType::Terminal(a) => Ok(format!("{}", a)),
            _ => Err(Error::InvalidParsing),
        }
    }
}
