use std::ops::Deref;

use crate::parser::{CharacterType, RegEx, RepetitionType};

pub struct RegExRenderer {}

impl Default for RegExRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl RegExRenderer {
    pub fn new() -> RegExRenderer {
        RegExRenderer {}
    }

    pub fn render_text(tree: &RegEx) -> Result<String, String> {
        match tree {
            RegEx::Element(a) => {
                let mut msg = "".to_string();
                for i in a.iter() {
                    msg = format!("{}{}", msg, Self::render_text(i.deref())?)
                }
                Ok(msg)
            }
            RegEx::Repetition(t, a) => {
                match t {
                    RepetitionType::ZeroOrOne => Ok(format!("{}: 0 or 1", Self::render_text(a)?)),
                    RepetitionType::OrMore(n) => {
                        Ok(format!("{}: {} or more", Self::render_text(a)?, n))
                    }
                    RepetitionType::Exactly(n) => {
                        Ok(format!("{}: Exactly {}", Self::render_text(a)?, n))
                    }
                    RepetitionType::Between(n, m) => {
                        Ok(format!("{}: Between {} and {}", Self::render_text(a)?, n, m))
                    }
                }
            }
            RegEx::Alternation(a) => {
                let mut msg = Self::render_text(a.first().unwrap())?;
                for i in a.iter().skip(1) {
                    msg = format!("{} or {}", msg, Self::render_text(i)?);
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
                "{}{}",
                Self::render_character(a)?,
                Self::render_character(b)?
            )),
            CharacterType::Terminal(a) => Ok(format!("{}", a)),
            _ => Err("Invalid parsing".to_string()),
        }
    }
}
