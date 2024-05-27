use lazy_static::lazy_static;
use std::collections::HashMap;
use std::{iter, ops::Deref};
use tracing::{error, info};

use crate::{
    error::Error,
    parser::{CharacterType, RegEx, RepetitionType},
};

type HighlightRegion = (usize, usize, usize);

const H_PADDING: usize = 2;
const V_PADDING: usize = 2;

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
    ]
    .iter()
    .copied()
    .collect();
}

pub struct RegExRenderer {
    _diagram: Vec<String>,
}

impl Default for RegExRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl RegExRenderer {
    pub fn new() -> RegExRenderer {
        RegExRenderer {
            _diagram: vec![String::new()],
        }
    }

    pub fn render_diagram(tree: &RegEx) -> Result<Vec<Vec<String>>, Error> {
        let mut diagram = Vec::new();
        match tree {
            RegEx::Element(a) => {
                for i in a.iter() {
                    diagram.push(RegExRenderer::render_diagram_element(i)?);
                }
            }
            other => {
                error!("Expected RegEx::Element, received {:?}", other);
                panic!("Expected RegEx::Element, received {:?}", other);
            }
        }
        Ok(diagram)
    }

    fn render_diagram_element(tree: &RegEx) -> Result<Vec<String>, Error> {
        Ok(RegExRenderer::draw_box(&"Hello!".to_string()))
    }

    fn draw_box(text: &String) -> Vec<String> {
        let width = text.len() + H_PADDING;
        let height = V_PADDING;
        let mut diagram: Vec<String> = Vec::new();

        // Top row
        diagram.push(format!(
            "{}{}{}",
            DRAWING_CHARS["CORNER_TL_SQR"],
            iter::repeat(DRAWING_CHARS["LINE_HORZ"])
                .take(width)
                .collect::<String>(),
            DRAWING_CHARS["CORNER_TR_SQR"]
        ));
        // Padding rows
        for _ in 0..height / 2 {
            diagram.push(format!(
                "{}{}{}",
                DRAWING_CHARS["LINE_VERT"],
                iter::repeat(' ').take(width).collect::<String>(),
                DRAWING_CHARS["LINE_VERT"]
            ))
        }
        // Text row
        diagram.push(format!(
            "{} {} {}",
            DRAWING_CHARS["LINE_VERT"], text, DRAWING_CHARS["LINE_VERT"]
        ));
        // Padding rows
        for _ in 0..height / 2 {
            diagram.push(format!(
                "{}{}{}",
                DRAWING_CHARS["LINE_VERT"],
                iter::repeat(' ').take(width).collect::<String>(),
                DRAWING_CHARS["LINE_VERT"]
            ))
        }
        // Top row
        diagram.push(format!(
            "{}{}{}",
            DRAWING_CHARS["CORNER_BL_SQR"],
            iter::repeat(DRAWING_CHARS["LINE_HORZ"])
                .take(width)
                .collect::<String>(),
            DRAWING_CHARS["CORNER_BR_SQR"]
        ));

        diagram
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
