use std::usize;
use std::iter;
use tracing::info;

use crate::{
    error::Error,
    parser::{AnchorType, RegEx, RepetitionType},
    railroad::sym,
    railroad::draw::{Draw, DrawGroup}
};

const H_PADDING: usize = 2;


// Repeat character n times
fn repeat(character: char, n: usize) -> String {
    iter::repeat(character).take(n).collect::<String>()
}


/// A horizontal sequence of railroad diagram elements
#[derive(Debug, Default)]
pub struct Sequence<N> {
    children: Vec<N>,
}

impl<N> Sequence<N> {
    #[must_use]
    pub fn new(children: Vec<N>) -> Self {
        Self {
            children
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
        let mut exit_height: usize = 0;
        for (n, child) in self.children.iter().enumerate() {
            let mut node = child.draw();

            for (a, b) in node.iter().enumerate() {
                info!("Node {} {}: {}", a, b.chars().count(), b);
            }

            // Ensure exit of previous node aligns with entry of new node
            match child.entry_height() {
                child_height if exit_height < child_height => {
                    let empty = repeat(' ', diagram[0].chars().count());
                    for _ in 0..(child.entry_height() - exit_height) {
                        diagram.insert(0, empty.clone());
                    }
                    exit_height = child.entry_height();
                },
                child_height if child_height < exit_height => {
                    let empty = repeat(' ', node[0].chars().count());
                    for _ in 0..(exit_height - child.entry_height()) {
                        node.insert(0, empty.clone());
                    }
                },
                _ => ()
            }

            // Add necessary padding to align new node
            match diagram.len() {
                diagram_len if node.len() < diagram_len => {
                    let empty = repeat(' ', node[0].chars().count());
                    for _ in 0..(diagram_len - node.len()) {
                        node.push(empty.clone());
                    }
                },
                diagram_len if diagram_len < node.len() => {
                    let empty = repeat(' ', diagram[0].chars().count());
                    for _ in 0..(node.len() - diagram_len) {
                        diagram.push(empty.clone());
                    }
                },
                _ => ()
            }

            if n > 0 {
                // Add padding
                let empty = repeat(' ', H_PADDING);
                let line = repeat(sym::L_HORZ, H_PADDING);
                for (i, d) in diagram.iter_mut().enumerate() {
                    if i == exit_height {
                        *d = format!("{}{}", d, line);
                    } else {
                        *d = format!("{}{}", *d, empty);
                    }
                }
                info!("Added padding");
            }

            // Append new node
            info!("Node {} - Diagram {}", node.len(), diagram.len());
            for i in 0..diagram.len() {
                diagram[i] = format!("{}{}", diagram[i], node[i]);
                info!("Diagram {}: {}", i, diagram[i]);
            }

            info!("Finished node {}", n);
        }

        diagram
    }
}

/// The `Start` of a railroad diagram
#[derive(Debug, Default)]
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
        vec![sym::START.to_string()]
    }
}

/// The `End` of a railroad diagram
#[derive(Debug, Default)]
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
        vec![sym::END.to_string()]
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
        self.text.chars().count() + 4
    }

    fn draw(&self) -> Vec<String> {
        let mut diagram = Vec::new();
        // Top row
        diagram.push(format!(
            "{}{}{}",
            sym::C_TL_SQR,
            repeat(sym::L_HORZ, self.width() - 2),
            sym::C_TR_SQR
        ));
        // Text row
        diagram.push(format!(
            "{} {} {}",
            sym::J_LEFT, self.text, sym::J_RIGHT
        ));
        // Top row
        diagram.push(format!(
            "{}{}{}",
            sym::C_BL_SQR,
            repeat(sym::L_HORZ, self.width() - 2),
            sym::C_BR_SQR
        ));

        diagram
    }
}

/// An `Anchor` node
#[derive(Debug)]
pub struct Anchor {
    text: String,
}

impl Anchor {
    #[must_use]
    pub fn new(text: String) -> Self {
        Anchor { text }
    }
}

impl Draw for Anchor {
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
            sym::C_TL_SQR_B,
            repeat(sym::L_HORZ_B, self.width() - 2),
            sym::C_TR_SQR_B
        ));
        // Text row
        diagram.push(format!(
            "{}{}{}",
            sym::J_LEFT_B, self.text, sym::J_RIGHT_B
        ));
        // Top row
        diagram.push(format!(
            "{}{}{}",
            sym::C_BL_SQR_B,
            repeat(sym::L_HORZ_B, self.width() - 2),
            sym::C_BR_SQR_B
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
        self.inner.height() + 1
    }

    fn width(&self) -> usize {
        self.inner.width() + 4
    }

    fn draw(&self) -> Vec<String> {
        let mut diagram = self.inner.draw();
        info!("Entry height: {}", self.entry_height());
        // Iterate through inner node
        for (i, d) in diagram.iter_mut().enumerate() {
            match self.entry_height() {
                height if height == i => {
                    *d = format!("{}{}{}{}{}",
                        sym::J_DOWN, sym::L_HORZ, *d, sym::L_HORZ, sym::J_DOWN
                    );
                },
                height if height < i => {
                    *d = format!("{} {} {}", sym::L_VERT, *d, sym::L_VERT);
                },
                _ => *d = format!("  {}  ", *d)
            }
        }

        for (i, n) in diagram.iter().enumerate() {
            info!("Repetition {}: {}", i, n);
        }

        // Description of how many repeats
        let desciption = match self.repetition {
            RepetitionType::OrMore(n) => format!(" {}+ ", n),
            RepetitionType::Exactly(n) => format!(" {} ", n),
            RepetitionType::Between(n, m) => format!(" {}-{} ", n, m),
            RepetitionType::ZeroOrOne => panic!("RepetitionType::ZeroOrOne should be parsed as Optional")
        };
        let padding = (diagram[0].chars().count() - desciption.chars().count()).saturating_sub(2);

        // Bottom loop
        diagram.push(format!("{}{}{}{}",
            sym::C_BL_RND,
            desciption,
            repeat(sym::L_HORZ, padding),
            sym::C_BR_RND
        ));

        for (i, n) in diagram.iter().enumerate() {
            info!("Repetition {}: {}", i, n);
        }

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
        self.inner.entry_height() + 1
    }

    fn height(&self) -> usize {
        self.inner.height() + 1
    }

    fn width(&self) -> usize {
        self.inner.width() + 2
    }

    fn draw(&self) -> Vec<String> {
        let mut diagram = self.inner.draw();
        let height = diagram.len();
        for (i, d) in diagram.iter_mut().enumerate() {
            match i {
                _ if i == height / 2 => {
                    *d = format!("{}{}{}{}{}",
                        sym::J_UP, sym::L_HORZ, *d, sym::L_HORZ, sym::J_UP
                    );   
                },
                _ if i < height / 2 => {
                    *d = format!("{} {} {}", sym::L_VERT, *d, sym::L_VERT);
                },
                _ => *d = format!("  {}  ", *d)
            }
        }

        // Top loop
        let len_full = diagram[0].chars().count() - 2;
        diagram.insert(0, format!("{}{}{}",
            sym::C_TL_RND,
            repeat(sym::L_HORZ, len_full),
            sym::C_TR_RND
        ));

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
        (self.inner.iter().total_height() - 1) / 2
    }

    fn height(&self) -> usize {
        self.inner.iter().total_height()
    }

    fn width(&self) -> usize {
        self.inner.iter().max_width()
    }

    fn draw(&self) -> Vec<String> {
        let mut diagram: Vec<String> = Vec::new();
        let choices = self.inner.len();
        let odd = choices % 2 == 1;
        // Zero-indexed midpoint
        let midpoint = (self.inner.iter().total_height() + 1) / 2 - 1;
        let width = self.inner.iter().max_width();
        info!("{} {} {}", choices, midpoint, odd);

        // Stack all choices vertically
        for (i, node) in self.inner.iter().enumerate() { 
            let sub_diagram = node.draw();
            let sub_len = sub_diagram[0].chars().count();

            // Ensure all nodes have the same width
            let left_pad = (width - sub_len) / 2;
            let right_pad = usize::div_ceil(width - sub_len, 2);
            info!("W{} S{} L{} R{} H{}", width, sub_len, left_pad, right_pad, node.entry_height());

            for (x, y) in sub_diagram.iter().enumerate() {
                info!("Sub {}: {}", x, y);
            }
            for (n, line) in sub_diagram.iter().enumerate() {
                // Draw connection...
                if n == node.entry_height() {
                    info!("Midpoint {}", line);
                    let (left_sym, right_sym) = if i == 0 {
                        (sym::C_TL_RND, sym::C_TR_RND)
                    } else if diagram.len() == midpoint {
                        (sym::CROSS, sym::CROSS)
                    } else if i == choices - 1 {
                        (sym::C_BL_RND, sym::C_BR_RND)
                    } else {
                        (sym::J_RIGHT, sym::J_LEFT)
                    };
                    diagram.push(format!("{}{}{}{}{}",
                        left_sym,
                        repeat(sym::L_HORZ, left_pad),
                        line,
                        repeat(sym::L_HORZ, right_pad),
                        right_sym
                    ));
                }
                else if diagram.len() == midpoint {
                    diagram.push(format!("{}{}{}",
                        sym::J_LEFT,
                        line,
                        sym::J_RIGHT
                    ));
                }
                // ...if first node and top or last row and bottom...
                else if (n < node.entry_height() && i == 0) || (n > node.entry_height() && i == choices - 1) {
                    diagram.push(format!(" {}{}{} ", repeat(' ', left_pad), line, repeat(' ', right_pad)));
                }
                // ...otherwise add vertical line
                else {
                    diagram.push(format!("{}{}{}{}{}",
                        sym::L_VERT,
                        repeat(' ', left_pad),
                        line,
                        repeat(' ', right_pad),
                        sym::L_VERT
                    ));
                }
            }
        }

        diagram
    }
}

#[derive(Default)]
pub struct RailroadRenderer {
    _diagram: Vec<String>,
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
                    let new_elem = Self::generate_diagram_element(i)?;
                    diagram.push(new_elem);
                }
            },
            _ => {
                let new_elem = Self::generate_diagram_element(tree)?;
                diagram.push(new_elem);
            }
        }
        diagram.push(Box::new(End {}));
        Ok(diagram)
    }

    pub fn generate_diagram_element(
        tree: &RegEx
    ) -> Result<Box<dyn Draw>, Error> {
        match tree {
            RegEx::Terminal(a) => Ok(Box::new(Terminal {
                text: a.to_string(),
            })),
            RegEx::Repetition(repetition, a) => match repetition {
                RepetitionType::ZeroOrOne => Ok(Box::new(Optional::<Box<dyn Draw>> {
                    inner: Self::generate_diagram_element(a)?,
                })),
                _ => Ok(Box::new(Repetition::<Box<dyn Draw>> {
                    inner: Self::generate_diagram_element(a)?,
                    repetition: *repetition,
                })),
            },
            RegEx::Alternation(a) => Ok(Box::new(Choice::<Box<dyn Draw>> {
                inner: a.iter().map(|x| Self::generate_diagram_element(x).unwrap()).collect()
            })),
            RegEx::Element(a) => {
                let mut seq = Vec::new();
                for i in a.iter() {
                    let new_elem = Self::generate_diagram_element(i)?;
                    seq.push(new_elem);
                }
                Ok(Box::new(Sequence::<Box<dyn Draw>>::new(seq)))
            },
            RegEx::Anchor(a) => {
                match a {
                    AnchorType::Start => {
                        Ok(Box::new(Anchor { text: String::from("LINE START")}))
                    },
                    AnchorType::End => {
                        Ok(Box::new(Anchor { text: String::from("LINE END")}))
                    },
                    _ => {
                        Ok(Box::new(Anchor { text: String::from("")}))
                    }
                }
            },
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

