use lazy_static::lazy_static;
use std::collections::HashMap;
use std::iter;
use tracing::info;

use crate::{
    error::Error,
    parser::{RegEx, RepetitionType},
};

const H_PADDING: usize = 2;

lazy_static! {
    static ref SYMBOL: HashMap<&'static str, char> = [
        ("START", '╟'),
        ("END", '╢'),
        ("CROSS", '┼'),
        ("J_LEFT", '┤'),
        ("J_RIGHT", '├'),
        ("J_UP", '┴'),
        ("J_DOWN", '┬'),
        ("L_HORZ", '─'),
        ("L_VERT", '│'),
        ("C_TL_SQR", '┌'),
        ("C_TR_SQR", '┐'),
        ("C_BL_SQR", '└'),
        ("C_BR_SQR", '┘'),
        ("C_TL_RND", '╭'),
        ("C_TR_RND", '╮'),
        ("C_BL_RND", '╰'),
        ("C_BR_RND", '╯')
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

impl<'a, N> Draw for &'a N
where
    N: Draw + ?Sized,
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

impl<N> Draw for Box<N>
where
    N: Draw + ?Sized,
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

impl<'a, N> Draw for &'a mut N
where
    N: Draw + ?Sized,
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
    children: Vec<N>,
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
            children: Vec::new(),
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
    N: Draw + std::fmt::Debug
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
        let mut diagram = vec![String::new()];
        for (n, child) in self.children.iter().enumerate() {
            let node = child.draw();
            let length = node.len();

            // Add extra lines to top/bottom of diagram if necessary
            if length > diagram.len() {
                let len = diagram[0].chars().count();
                let empty = iter::repeat(' ').take(len).collect::<String>();
                for i in 0..(length / 2) {
                    diagram.insert(i, empty.clone());
                    diagram.push(empty.clone());
                }
            }

            if n > 0 {
                // Add padding
                let height = diagram.len();
                let empty = iter::repeat(' ').take(H_PADDING).collect::<String>();
                let line = iter::repeat(SYMBOL["L_HORZ"]).take(H_PADDING).collect::<String>();
                for i in 0..height {
                    if i == (height - 1) / 2 {
                        diagram[i] = format!("{}{}", diagram[i], line);
                    } else {
                        diagram[i] = format!("{}{}", diagram[i], empty);
                    }
                }
            }

            // Append new node
            for i in 0..length {
                diagram[i] = format!("{}{}", diagram[i], node[i]);
            }
        }

        diagram
    }
}

/// The `Start` of a railroad diagram
#[derive(Debug)]
pub struct Start {}

impl Start {
    #[must_use]
    pub fn new() -> Self {
        Start {}
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
        vec![SYMBOL["START"].to_string()]
    }
}

/// The `End` of a railroad diagram
#[derive(Debug)]
pub struct End {}

impl End {
    #[must_use]
    pub fn new() -> Self {
        End {}
    }
}

impl Draw for End {
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
        vec![SYMBOL["END"].to_string()]
    }
}

/// A `Terminal` node
#[derive(Debug)]
pub struct Terminal {
    text: String,
}

impl Terminal {
    #[must_use]
    pub fn new(text: String) -> Self {
        Terminal { text }
    }
}

impl Draw for Terminal {
    fn entry_height(&self) -> usize {
        2
    }

    fn height(&self) -> usize {
        3
    }

    fn width(&self) -> usize {
        self.text.chars().count() + 2
    }

    fn draw(&self) -> Vec<String> {
        let mut diagram = Vec::new();
        // Top row
        diagram.push(format!(
            "{}{}{}",
            SYMBOL["C_TL_SQR"],
            iter::repeat(SYMBOL["L_HORZ"])
                .take(self.width())
                .collect::<String>(),
            SYMBOL["C_TR_SQR"]
        ));
        // Text row
        diagram.push(format!(
            "{} {} {}",
            SYMBOL["J_LEFT"], self.text, SYMBOL["J_RIGHT"]
        ));
        // Top row
        diagram.push(format!(
            "{}{}{}",
            SYMBOL["C_BL_SQR"],
            iter::repeat(SYMBOL["L_HORZ"])
                .take(self.width())
                .collect::<String>(),
            SYMBOL["C_BR_SQR"]
        ));

        diagram
    }
}

/// A `Repetition` of a node
pub struct Repetition<N> {
    inner: N,
    repetition: RepetitionType,
}

impl<N> Repetition<N> {
    pub fn new(inner: N, repetition: RepetitionType) -> Self {
        Self { inner, repetition }
    }

    pub fn into_inner(self) -> N {
        self.inner
    }
}

impl<N> Draw for Repetition<N>
where
    N: Draw,
{
    fn entry_height(&self) -> usize {
        self.inner.entry_height()
    }

    fn height(&self) -> usize {
        self.inner.height() + 2
    }

    fn width(&self) -> usize {
        self.inner.width()
    }

    fn draw(&self) -> Vec<String> {
        let mut diagram = self.inner.draw();
        let height = diagram.len();
        for i in 0..height {
            if i == height / 2 {
                diagram[i] = format!("{}{}{}", SYMBOL["J_DOWN"], diagram[i], SYMBOL["J_DOWN"]);
            }
            else if i > height / 2 {
                diagram[i] = format!("{}{}{}", SYMBOL["L_VERT"], diagram[i], SYMBOL["L_VERT"]);
            }
            else {
                diagram[i] = format!(" {} ", diagram[i])
            }
        }

        let len_empty = diagram[0].chars().count();
        diagram.insert(0, iter::repeat(' ').take(len_empty).collect::<String>());

        let len_full = diagram[0].chars().count() - 2;
        diagram.push(format!("{}{}{}",
            SYMBOL["C_BL_RND"],
            iter::repeat(SYMBOL["L_HORZ"]).take(len_full).collect::<String>(),
            SYMBOL["C_BR_RND"]
        ));

        diagram
    }
}

/// An `Optional` node
pub struct Optional<N> {
    inner: N,
}

impl<N> Optional<N> {
    pub fn new(inner: N) -> Self {
        Optional { inner }
    }
}

impl<N> Draw for Optional<N>
where
    N: Draw,
{
    fn entry_height(&self) -> usize {
        self.inner.entry_height()
    }

    fn height(&self) -> usize {
        self.inner.height()
    }

    fn width(&self) -> usize {
        self.inner.width()
    }

    fn draw(&self) -> Vec<String> {
        // TODO: write function
        vec![]
    }
}

/// A `Choice` of nodes
pub struct Choice<N> {
    inner: Vec<N>
}

impl<N> Choice<N> {
    pub fn new(inner: Vec<N>) -> Self {
        Choice { inner }
    }
}

impl<N> Draw for Choice<N>
where
    N: Draw
{
    fn entry_height(&self) -> usize {
        self.inner.iter().max_entry_height()
    }

    fn height(&self) -> usize {
        self.inner.iter().max_height()
    }

    fn width(&self) -> usize {
        self.inner.iter().max_width()
    }

    fn draw(&self) -> Vec<String> {
        vec![]
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

    pub fn generate_diagram(tree: &RegEx) -> Result<Sequence<Box<dyn Draw>>, Error> {
        let mut diagram = Sequence::new(vec![Box::new(Start {}) as Box<dyn Draw>]);
        match tree {
            RegEx::Element(a) => {
                for i in a.iter() {
                    let new_elem = Self::generate_diagram_element(&*i, &mut diagram)?;
                    diagram.push(new_elem);
                }
            },
            _ => {
                let new_elem = Self::generate_diagram_element(tree, &mut diagram)?;
                diagram.push(new_elem);
            }
        }
        diagram.push(Box::new(End {}));
        Ok(diagram)
    }

    pub fn generate_diagram_element(
        tree: &RegEx,
        diagram: &mut Sequence<Box<dyn Draw>>,
    ) -> Result<Box<dyn Draw>, Error> {
        match tree {
            RegEx::Terminal(a) => Ok(Box::new(Terminal {
                text: a.to_string(),
            })),
            RegEx::Repetition(repetition, a) => match repetition {
                RepetitionType::ZeroOrOne => Ok(Box::new(Optional::<Box<dyn Draw>> {
                    inner: Self::generate_diagram_element(a, diagram)?,
                })),
                _ => Ok(Box::new(Repetition::<Box<dyn Draw>> {
                    inner: Self::generate_diagram_element(a, diagram)?,
                    repetition: *repetition,
                })),
            },
            RegEx::Alternation(a) => Ok(Box::new(Choice::<Box<dyn Draw>> {
                inner: a.iter().map(|x| Self::generate_diagram_element(x, diagram).unwrap()).collect()
            })),
            RegEx::Element(a) => {
                let mut seq = Vec::new();
                for i in a.iter() {
                    let new_elem = Self::generate_diagram_element(i, diagram)?;
                    seq.push(new_elem);
                }
                Ok(Box::new(Sequence::<Box<dyn Draw>>::new(seq)))
            }
            other => {
                info!("Other {:?}", other);
                Ok(Box::new(Terminal { text: String::new() }))
            },
        }
    }

    pub fn render_diagram(diagram: &Sequence<Box<dyn Draw>>) -> Result<Vec<String>, Error> {
        Ok(diagram.draw())
    }
}

