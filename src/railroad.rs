use lazy_static::lazy_static;
use std::{collections::HashMap, usize};
use std::iter;
use tracing::info;

use crate::{
    error::Error,
    parser::{RegEx, RepetitionType},
};

const H_PADDING: usize = 2;

lazy_static! {
    static ref SYM: HashMap<&'static str, char> = [
        ("START", '╟'),
        ("END", '╢'),
        ("CROSS", '┼'),
        ("J_LEFT", '┤'),
        ("J_RIGHT", '├'),
        ("J_UP", '┴'),
        ("J_DOWN", '┬'),
        ("A_DOWN", '▽'),
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


// Repeat character n times
fn repeat(character: char, n: usize) -> String {
    iter::repeat(character).take(n).collect::<String>()
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
        let mut diagram: Vec<String> = vec![String::new()];
        for (n, child) in self.children.iter().enumerate() {
            let mut node = child.draw();
            let length = node.len();

            for (a, b) in node.iter().enumerate() {
                info!("Node {}: {}", a, b);
            }

            // Add extra lines to top/bottom of diagram if necessary...
            if length > diagram.len() {
                info!("Node > Diagram");
                let len = diagram[0].chars().count();
                let empty = repeat(' ', len);
                info!("Diagram length: {}", diagram.len());
                for i in 0..((length - diagram.len()) / 2) {
                    diagram.insert(i, empty.clone());
                    diagram.push(empty.clone());
                    info!("Diagram length: {}", diagram.len());
                }
            }
            // ...or extra lines to top/bottom of new node
            else if length < diagram.len() {
                info!("Diagram > Node");
                let len = node[0].chars().count();
                let empty = repeat(' ', len);
                for _i in 0..((diagram.len() - length) / 2) {
                    node.insert(0, empty.clone());
                    node.push(empty.clone());
                }
            }

            if n > 0 {
                // Add padding
                let height = diagram.len();
                let empty = repeat(' ', H_PADDING);
                let line = repeat(SYM["L_HORZ"], H_PADDING);
                for i in 0..height {
                    if i == (height - 1) / 2 {
                        diagram[i] = format!("{}{}", diagram[i], line);
                    } else {
                        diagram[i] = format!("{}{}", diagram[i], empty);
                    }
                }
                info!("Added padding");
            }

            // Append new node
            info!("Node {} - Diagram {}", node.len(), diagram.len());
            for i in 0..diagram.len() {
                diagram[i] = format!("{}{}", diagram[i], node[i]);
            }

            info!("Finished node {}", n);
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
        vec![SYM["START"].to_string()]
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
        vec![SYM["END"].to_string()]
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
        1
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
            SYM["C_TL_SQR"],
            repeat(SYM["L_HORZ"], self.width()),
            SYM["C_TR_SQR"]
        ));
        // Text row
        diagram.push(format!(
            "{} {} {}",
            SYM["J_LEFT"], self.text, SYM["J_RIGHT"]
        ));
        // Top row
        diagram.push(format!(
            "{}{}{}",
            SYM["C_BL_SQR"],
            repeat(SYM["L_HORZ"], self.width()),
            SYM["C_BR_SQR"]
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
                diagram[i] = format!("{}{}{}{}{}",
                    SYM["J_DOWN"], SYM["L_HORZ"], diagram[i], SYM["L_HORZ"], SYM["J_DOWN"]
                );
            }
            else if i > height / 2 {
                diagram[i] = format!("{} {} {}", SYM["L_VERT"], diagram[i], SYM["L_VERT"]);
            }
            else {
                diagram[i] = format!("  {}  ", diagram[i])
            }
        }

        // Top padding
        let len_empty = diagram[0].chars().count();
        diagram.insert(0, repeat(' ', len_empty));
        diagram.insert(0, repeat(' ', len_empty));

        // Bottom loop
        let len_full = diagram[0].chars().count() - 2;
        diagram.push(format!("{}{}{}",
            SYM["C_BL_RND"],
            repeat(SYM["L_HORZ"], len_full),
            SYM["C_BR_RND"]
        ));

        // Description of how many repeats
        let desciption = match self.repetition {
            RepetitionType::OrMore(n) => format!("{} or more", n),
            RepetitionType::Exactly(n) => format!("Exactly {}", n),
            RepetitionType::Between(n, m) => format!("{} to {}", n, m),
            RepetitionType::ZeroOrOne => panic!("RepetitionType::ZeroOrOne should be parsed as Optional")
        };
        let padding_start = (diagram[0].len() - desciption.len()) / 2;
        let padding_end = diagram[0].len() - padding_start;
        diagram.push(format!("{}{}{}",
            repeat(' ', padding_start),
            desciption,
            repeat(' ', padding_end)
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
        let mut diagram = self.inner.draw();
        let height = diagram.len();
        for i in 0..height {
            if i == height / 2 {
                diagram[i] = format!("{}{}{}{}{}",
                    SYM["J_UP"], SYM["L_HORZ"], diagram[i], SYM["L_HORZ"], SYM["J_UP"]
                );
            }
            else if i < height / 2 {
                diagram[i] = format!("{} {} {}", SYM["L_VERT"], diagram[i], SYM["L_VERT"]);
            }
            else {
                diagram[i] = format!("  {}  ", diagram[i])
            }
        }

        // Top loop
        let len_full = diagram[0].chars().count() - 2;
        diagram.insert(0, format!("{}{}{}",
            SYM["C_TL_RND"],
            repeat(SYM["L_HORZ"], len_full),
            SYM["C_TR_RND"]
        ));

        // Bottom padding
        let len_empty = diagram[0].chars().count();
        diagram.push(repeat(' ', len_empty));

        diagram
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
        self.inner.iter().total_height() / 2
    }

    fn height(&self) -> usize {
        self.inner.iter().max_height()
    }

    fn width(&self) -> usize {
        self.inner.iter().max_width()
    }

    fn draw(&self) -> Vec<String> {
        let mut diagram: Vec<String> = Vec::new();
        let choices = self.inner.len();
        let odd = choices % 2 == 1;
        let midpoint = choices / 2;
        info!("{} {} {}", choices, midpoint, odd);

        // Stack all choices vertically
        for (i, node) in self.inner.iter().enumerate() { 
            let sub_diagram = node.draw();
            for x in &sub_diagram {
                info!("{}", x);
            }
            for line in 0..sub_diagram.len() {
                // Draw connection...
                if line == node.entry_height() {
                    info!("Midpoint {}", line);
                    let (left_sym, right_sym) = if i == 0 {
                        (SYM["C_TL_RND"], SYM["C_TR_RND"])
                    } else if i == midpoint && odd {
                        (SYM["CROSS"], SYM["CROSS"])
                    } else if i == choices - 1 {
                        (SYM["C_BL_RND"], SYM["C_BR_RND"])
                    } else {
                        (SYM["J_RIGHT"], SYM["J_LEFT"])
                    };
                    diagram.push(format!("{}{}{}",
                        left_sym,
                        sub_diagram[line],
                        right_sym
                    ));
                }
                // ...if first node and top or last row and bottom...
                else if (line < node.entry_height() && i == 0) || (line > node.entry_height() && i == choices - 1) {
                    diagram.push(format!(" {} ", sub_diagram[line]));
                }
                // ...otherwise add vertical line
                else {
                    diagram.push(format!("{}{}{}",
                        SYM["L_VERT"],
                        sub_diagram[line],
                        SYM["L_VERT"]
                    ));
                }
            }
                
            // Add vertical spacing if not the final node
            if i != choices - 1 && !odd {
                if i == (choices / 2) - 1 {
                    diagram.push(format!("{}{}{}",
                        SYM["J_LEFT"],
                        repeat(' ', sub_diagram[0].chars().count()),
                        SYM["J_RIGHT"]
                    ));
                }
            }
        }

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

