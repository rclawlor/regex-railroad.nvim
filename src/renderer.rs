use lazy_static::lazy_static;
use std::collections::HashMap;
use std::ops::Deref; 
use tracing::error;

use crate::parser::{CharacterType, RegEx, RepetitionType};

lazy_static! {
    static ref DRAWING_CHARS: HashMap<&'static str, char> = [
        ("START", '╟'),
        ("END", '╢'),
        ("LINE_HORZ", '─'),
        ("LINE_VERT", '│'),
        ("CORNER_TL_SQR", '┌'),
        ("CORNER_TR_SQR", '┐'),
        ("CORNER_BL_SQR", '└'),
        ("CORNER_BR_SQR", '┘'),
        ("CORNER_TL_RND", '╭'),
        ("CORNER_TR_RND", '╮'),
        ("CORNER_BL_RND", '╰'),
        ("CORNER_BR_RND", '╯')
    ].iter().copied().collect();
}

pub struct RegExRenderer {
    diagram: Vec<String>
}

impl Default for RegExRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl RegExRenderer {
    pub fn new() -> RegExRenderer {
        RegExRenderer { diagram: vec![String::new()]}
    }

    pub fn render_diagram(tree: &RegEx) -> Result<Vec<Vec<String>>, String> {
        let mut msg = Vec::new();
        match tree {
            RegEx::Element(a) => {
                for i in a.iter() {
                    msg.push(RegExRenderer::render_diagram_element()?);
                };
            },
            other => {
                error!("Expected RegEx::Element, received {:?}", other);
                panic!("Expected RegEx::Element, received {:?}", other);
            }
        }
        Ok(msg)
    }

    fn render_diagram_element() -> Result<Vec<String>, String> {
        Ok(vec![]) 
    }

    pub fn render_text(tree: &RegEx) -> Result<Vec<String>, String> {
        let mut msg = Vec::new();
        match tree {
            RegEx::Element(a) => {
                for i in a.iter() {
                    msg.push(RegExRenderer::render_text_element(i)?);
                };
            },
            other => {
                error!("Expected RegEx::Element, received {:?}", other);
                panic!("Expected RegEx::Element, received {:?}", other);
            }
        }
        Ok(msg)
    }

    fn render_text_element(tree: &RegEx) -> Result<String, String> {
        match tree {
            RegEx::Element(a) => {
                let mut msg = "".to_string();
                for i in a.iter() {
                    msg = format!("{}{}", msg, Self::render_text_element(i.deref())?)
                }
                Ok(msg)
            }
            RegEx::Repetition(t, a) => {
                match t {
                    RepetitionType::ZeroOrOne => Ok(format!("{}: 0 or 1", Self::render_text_element(a)?)),
                    RepetitionType::OrMore(n) => {
                        Ok(format!("{} or more '{}'", n, Self::render_text_element(a)?))
                    }
                    RepetitionType::Exactly(n) => {
                        Ok(format!("Exactly {} '{}'", n, Self::render_text_element(a)?))
                    }
                    RepetitionType::Between(n, m) => {
                        Ok(format!("Between {} and {} '{}'", n, m, Self::render_text_element(a)?))
                    }
                }
            }
            RegEx::Alternation(a) => {
                let mut msg = Self::render_text_element(a.first().unwrap())?;
                for i in a.iter().skip(1) {
                    msg = format!("{} or {}", msg, Self::render_text_element(i)?);
                }
                Ok(msg)
            }
            RegEx::Character(a) => match a {
                CharacterType::Any(b) => {
                    let mut msg = String::from("Match string:");
                    for i in b.iter() {
                        msg = format!("{} {}", msg, Self::render_character(i)?)
                    }
                    Ok(msg)
                }
                CharacterType::Not(b) => {
                    let mut msg = String::from("Don't match string:");
                    for i in b.iter() {
                        msg = format!("{} {}", msg, Self::render_character(i)?)
                    }
                    Ok(msg)
                }
                _ => Err("Invalid parsing: RegEx::Character cannot begin with CharacterType::Between or CharacterType::Terminal".to_string()),
            },
            RegEx::Terminal(a) => Ok(a.to_string())
        }
    }

    fn render_character(character: &CharacterType) -> Result<String, String> {
        match character {
            CharacterType::Between(a, b) => Ok(format!(
                "{} to {}",
                Self::render_character(a)?,
                Self::render_character(b)?
            )),
            CharacterType::Terminal(a) => Ok(format!("{}", a)),
            _ => Err("Invalid parsing".to_string()),
        }
    }
}
