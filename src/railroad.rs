use lazy_static::lazy_static;
use std::collections::HashMap;
use std::{iter, ops::Deref};
use tracing::{error, info};

use crate::{
    error::Error,
    parser::{CharacterType, RegEx, RepetitionType},
};

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

pub struct RailroadRenderer {
    _diagram: Vec<String>,
}

impl Default for RailroadRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl RailroadRenderer {
    pub fn new() -> RailroadRenderer {
        RailroadRenderer {
            _diagram: vec![String::new()],
        }
    }

    pub fn render_diagram(tree: &RegEx) -> Result<Vec<Vec<String>>, Error> {
        let mut diagram = Vec::new();
        match tree {
            RegEx::Element(a) => {
                for i in a.iter() {
                    match **i {
                        RegEx::Terminal(_) => {
                            diagram.push(Self::render_diagram_element(i)?);
                        },
                        _ => {
                            diagram.push(RailroadRenderer::render_diagram_element(i)?);
                        }
                    }
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
        match tree {
            RegEx::Terminal(a) => Ok(RailroadRenderer::draw_box(a)),
            _ => Ok(vec![String::new()])
        }
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
}

