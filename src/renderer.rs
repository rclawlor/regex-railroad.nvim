use std::ops::Deref;

use crate::parser::{CharacterType, RegEx, RepetitionType};

pub struct RegExRenderer {}

impl RegExRenderer {
    pub fn new() -> RegExRenderer {
        RegExRenderer {}
    }

    pub fn render_text(&self, tree: &RegEx) -> String {
        match tree {
            RegEx::Element(a) => {
                let mut msg = "".to_string();
                for i in a.iter() {
                    msg = format!("{}{}", msg, self.render_text(i.deref()))
                }
                msg
            }
            RegEx::Repetition(t, a) => {
                let msg = match t {
                    RepetitionType::ZeroOrOne => format!("{}: 0 or 1 -- ", self.render_text(a)),
                    RepetitionType::OrMore(n) => {
                        format!("{}: {} or more -- ", self.render_text(a), n)
                    }
                    RepetitionType::Exactly(n) => {
                        format!("{}: Exactly {} -- ", self.render_text(a), n)
                    }
                    RepetitionType::Between(n, m) => {
                        format!("{}: Between {} and {} -- ", self.render_text(a), n, m)
                    }
                };
                msg.to_string()
            }
            RegEx::Alternation(a, b) => {
                format!("{} or {}", self.render_text(a), self.render_text(b))
            }
            RegEx::Character(a) => {
                match a {
                    CharacterType::Any(b) => {
                        let mut msg = String::from("");
                        for i in b.iter() {
                        } 
                        msg
                    },
                    CharacterType::Not(b) => {
                        let mut msg = String::from("");
                        msg
                    },
                    _ => "".to_string()
                }
            }
            RegEx::Terminal(a) => a.to_string(),
        }
    }
}
