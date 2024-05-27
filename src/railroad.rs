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


pub trait Draw {
    /// The number of lines from this element's top to where the entering,
    /// connecting path is drawn.
    fn entry_height(&self) -> usize;

    /// This primitives's total height.
    fn height(&self) -> usize;

    /// This primitive's total width.
    fn width(&self) -> usize;

    /// Draw this element.
    fn draw(&self) -> Vec<String>;
}

impl std::fmt::Debug for dyn Draw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Draw")
            .field("entry_height", &self.entry_height())
            .field("height", &self.height())
            .field("width", &self.width())
            .finish()
    }
}

impl <'a, N> Draw for &'a N
where
    N: Draw + ?Sized
{
    fn entry_height(&self) -> usize {
        (**self).entry_height()
    }

    fn height(&self) -> usize {
        (**self).height()
    }

    fn width(&self) -> usize {
        (**self).width()
    }

    fn draw(&self) -> Vec<String> {
        (**self).draw()
    }
}

impl <N> Draw for Box<N>
where
    N: Draw + ?Sized
{
    fn entry_height(&self) -> usize {
        (**self).entry_height()
    }

    fn height(&self) -> usize {
        (**self).height()
    }

    fn width(&self) -> usize {
        (**self).width()
    }

    fn draw(&self) -> Vec<String> {
        (**self).draw()
    }
}

impl <'a, N> Draw for &'a mut N
where
    N: Draw + ?Sized
{
    fn entry_height(&self) -> usize {
        (**self).entry_height()
    }

    fn height(&self) -> usize {
        (**self).height()
    }

    fn width(&self) -> usize {
        (**self).width()
    }

    fn draw(&self) -> Vec<String> {
        (**self).draw()
    }
}

pub trait DrawGroup {
    /// The maximum `entry_height()`-value.
    fn max_entry_height(self) -> usize;

    /// The maximum `height()`-value.
    fn max_height(self) -> usize;

    /// The maximum `width()`-value.
    fn max_width(self) -> usize;

    /// The sum of all `width()`-values.
    fn total_width(self) -> usize;

    /// The sum of all `height()`-values.
    fn total_height(self) -> usize;
}

impl<I, N> DrawGroup for I
where
    I: IntoIterator<Item = N>,
    N: Draw,
{
    fn max_entry_height(self) -> usize {
        self.into_iter()
            .map(|n| n.entry_height())
            .max()
            .unwrap_or_default()
    }

    fn max_height(self) -> usize {
        self.into_iter()
            .map(|n| n.height())
            .max()
            .unwrap_or_default()
    }

    fn max_width(self) -> usize {
        self.into_iter()
            .map(|n| n.width())
            .max()
            .unwrap_or_default()
    }

    fn total_width(self) -> usize {
        self.into_iter().map(|n| n.width()).sum()
    }

    fn total_height(self) -> usize {
        self.into_iter().map(|n| n.height()).sum()
    }
}

/// A horizontal sequence of railroad diagram elements
#[derive(Debug)]
pub struct Sequence<N> {
    children: Vec<N>
}

impl<N> Sequence<N> {
    #[must_use]
    pub fn new(children: Vec<N>) -> Self {
        Self {
            children,
            ..Self::default()
        }
    }

    pub fn push(&mut self, child: N) {
        self.children.push(child);
    }

    #[must_use]
    pub fn into_inner(self) -> Vec<N> {
        self.children
    }
}

impl<N> Default for Sequence<N> {
    fn default() -> Self {
        Self {
            children: Vec::new()
        }
    }
}

impl<N> iter::FromIterator<N> for Sequence<N> {
    fn from_iter<T: IntoIterator<Item = N>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl<N> Draw for Sequence<N>
where
    N: Draw
{
    fn entry_height(&self) -> usize {
        self.children.iter().max_entry_height()
    }

    fn height(&self) -> usize {
        self.children.iter().max_height()
    }

    fn width(&self) -> usize {
        self.children.iter().max_width()
    }

    fn draw(&self) -> Vec<String> {
        vec![]
    }
}

/// The `Start` of a railroad diagram
#[derive(Debug)]
pub struct Start {}

impl Start {
    #[must_use]
    pub fn new() -> Self {
        Start { }
    }
}

impl Draw for Start {
    fn entry_height(&self) -> usize {
        0
    }

    fn height(&self) -> usize {
        1
    }

    fn width(&self) -> usize {
        1
    }

    fn draw(&self) -> Vec<String> {
        vec![DRAWING_CHARS["START"].to_string()]
    }
}

/// A `Terminal` node
#[derive(Debug)]
pub struct Terminal {
    text: String
}

impl Terminal {
    #[must_use]
    pub fn new(text: String) -> Self {
        Terminal { text }
    }
}

impl Draw for Terminal {
    fn entry_height(&self) -> usize {
        3
    }

    fn height(&self) -> usize {
        5
    }

    fn width(&self) -> usize {
        self.text.len()
    }

    fn draw(&self) -> Vec<String> {
        let mut diagram = Vec::new();
        // Top row
        diagram.push(format!(
            "{}{}{}",
            DRAWING_CHARS["CORNER_TL_SQR"],
            iter::repeat(DRAWING_CHARS["LINE_HORZ"])
                .take(self.width())
                .collect::<String>(),
            DRAWING_CHARS["CORNER_TR_SQR"]
        ));
        // Padding rows
        for _ in 0..self.height() / 2 {
            diagram.push(format!(
                "{}{}{}",
                DRAWING_CHARS["LINE_VERT"],
                iter::repeat(' ').take(self.width()).collect::<String>(),
                DRAWING_CHARS["LINE_VERT"]
            ))
        }
        // Text row
        diagram.push(format!(
            "{} {} {}",
            DRAWING_CHARS["LINE_VERT"], self.text, DRAWING_CHARS["LINE_VERT"]
        ));
        // Padding rows
        for _ in 0..self.height() / 2 {
            diagram.push(format!(
                "{}{}{}",
                DRAWING_CHARS["LINE_VERT"],
                iter::repeat(' ').take(self.width()).collect::<String>(),
                DRAWING_CHARS["LINE_VERT"]
            ))
        }
        // Top row
        diagram.push(format!(
            "{}{}{}",
            DRAWING_CHARS["CORNER_BL_SQR"],
            iter::repeat(DRAWING_CHARS["LINE_HORZ"])
                .take(self.width())
                .collect::<String>(),
            DRAWING_CHARS["CORNER_BR_SQR"]
        ));
        
        diagram
    }
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

    pub fn generate_diagram(tree: &RegEx) {
        let mut diagram = Sequence::new(vec![Box::new(Start {}) as Box<dyn Draw>]);
        match tree {
            RegEx::Element(a) => {
                for i in a.iter() {
                    Self::generate_diagram_element::<Box<dyn Draw>>(&*i, &mut diagram)
                }
            }
            _ => ()
        }
        info!("{:?}", diagram);
    }

    pub fn generate_diagram_element<N: Draw + ?Sized>(tree: &RegEx, diagram: &mut Sequence<Box<dyn Draw>>) {
        match tree {
            RegEx::Terminal(a) => diagram.push(Box::new(Terminal { text: a.to_string() })),
            _ => ()
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

