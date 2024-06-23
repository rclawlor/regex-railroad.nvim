use std::iter;
use tracing::info;

use crate::parser::{CharacterType, MetaCharacter};
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
///
///   ┌───┐  ┌───┐  ┌───┐
///   │ A ├──┤ B ├──┤ C │
///   └───┘  └───┘  └───┘
///
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
///
///   START╟───
///
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
        6
    }

    fn draw(&self) -> Vec<String> {
        vec![format!("START{}", sym::START.to_string())]
    }
}

/// The `End` of a railroad diagram
///
///   ───╢END
///
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
        vec![format!("{}END", sym::END.to_string())]
    }
}

/// A `Terminal` node
///
///   ┌──────────┐
///   ┤ Terminal ├
///   └──────────┘
///
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
///
///   ┏━━━━━━━━┓
///   ┨ Anchor ┠
///   ┗━━━━━━━━┛
///
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
///
///     ┌────────────┐
///   ┬─┤ Repetition ├─┬
///   │ └────────────┘ │
///   ╰─N──────────────╯
///
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
///
///   ╭──────────────╮
///   │ ┌──────────┐ │
///   ┴─┤ Optional ├─┴
///     └──────────┘
///
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
        self.inner.width() + 4
    }

    fn draw(&self) -> Vec<String> {
        let mut diagram = self.inner.draw();
        for (i, d) in diagram.iter_mut().enumerate() {
            match self.entry_height() {
                height if height - 1 == i => {
                    *d = format!("{}{}{}{}{}",
                        sym::J_UP, sym::L_HORZ, *d, sym::L_HORZ, sym::J_UP
                    );
                },
                height if i < height => {
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
///
///     ┌───┐
///   ╭─┤ A ├─╮
///   │ └───┘ │
///   ┤       ├
///   │ ┌───┐ │
///   ╰─┤ B ├─╯
///     └───┘
///
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

/// A `Stack` of options
///
///    ┌───┐
///    │'A'│
///    ┤'B'├
///    │'C'│
///    └───┘
///
pub struct Stack {
    invert: bool,
    characters: Vec<String>
}

impl Stack {
    pub fn new(invert: bool, characters: Vec<String>) -> Self {
        Stack { invert, characters }
    }
}

impl Draw for Stack {
    fn entry_height(&self) -> usize {
        (self.characters.len() + 3) / 2
    }

    fn height(&self) -> usize {
        self.characters.len() + 3
    }

    fn width(&self) -> usize {
        std::cmp::max(
            self.characters.iter()
                .map(|x| x.chars().count())
                .max()
                .unwrap_or(0) + 2,
            9
        )
    }
    
    fn draw(&self) -> Vec<String> {
        let mut diagram = Vec::new();
        let width = self.width();
        let entry_height = self.entry_height();
        // Description
        if self.invert {
            diagram.push(format!("None of:{}", repeat(' ', width - 8)));
        } else {
            diagram.push(format!("One of:{}", repeat(' ', width - 7)));
        }
        // Top row
        diagram.push(format!(
            "{}{}{}",
            sym::C_TL_SQR,
            repeat(sym::L_HORZ, width - 2),
            sym::C_TR_SQR
        ));
        // Characters
        for character in self.characters.iter() {
            let sub_len = character.chars().count();
            let left_pad = (width - 2 - sub_len) / 2;
            let right_pad = usize::div_ceil(width - 2 - sub_len, 2);
            let (left_char, right_char) = match diagram.len() {
                a if a == entry_height => (sym::J_LEFT, sym::J_RIGHT),
                _ => (sym::L_VERT, sym::L_VERT)
            };
            diagram.push(format!(
                "{}{}{}{}{}",
                left_char,
                repeat(' ', left_pad),
                character, 
                repeat(' ', right_pad),
                right_char
            ));
        }
        // Bottom row
        diagram.push(format!(
            "{}{}{}",
            sym::C_BL_SQR,
            repeat(sym::L_HORZ, self.width() - 2),
            sym::C_BR_SQR
        ));

        diagram

    }
}

/// A 'Capture' group
///
///  ╭╌╌╌╌ Name ╌╌╌╌╮
///  ┆ ┌──────────┐ ┆
///  ┼─┤   Node   ├─┼
///  ┆ └──────────┘ ┆
///  ╰╌╌╌╌╌╌╌╌╌╌╌╌╌╌╯
///
#[derive(Debug)]
pub struct Capture<N> {
    inner: N,
    name: String
}

impl<N> Capture<N> {
    pub fn new(inner: N, name: String) -> Self {
        Self { inner, name }
    }
}

impl<N> Draw for Capture<N>
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
        self.inner.width() + 4
    }

    fn draw(&self) -> Vec<String> {
        let mut diagram = self.inner.draw();
        // Iterate through inner node
        for (i, d) in diagram.iter_mut().enumerate() {
            match self.entry_height() {
                height if height == i + 1 => {
                    *d = format!("{}{}{}{}{}",
                        sym::CROSS, sym::L_HORZ, *d, sym::L_HORZ, sym::CROSS
                    );
                },
                _ => {
                    *d = format!("{} {} {}", sym::L_VERT_D, *d, sym::L_VERT_D);
                }
            }
        }
        let len_full = diagram[0].chars().count() - 2;
        let len_name = self.name.chars().count();
        let left_pad = (len_full - len_name) / 2;
        let right_pad = len_full - len_name - left_pad;
        diagram.insert(0, format!("{}{}{}{}{}",
            sym::C_TL_RND,
            repeat(sym::L_HORZ_D, left_pad),
            self.name,
            repeat(sym::L_HORZ_D, right_pad),
            sym::C_TR_RND
        ));

        diagram.push(format!("{}{}{}",
            sym::C_BL_RND,
            repeat(sym::L_HORZ_D, len_full),
            sym::C_BR_RND
        ));

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
            RegEx::Character(a) => {
                let mut invert = false;
                let b = match a {
                    CharacterType::Any(b) => b,
                    CharacterType::Not(b) => {
                        invert = true;
                        b
                    },
                    CharacterType::Meta(_) => {
                        return Ok(Box::new(Anchor { text: Self::render_character(a)? }))
                    }
                    _ => return Err(Error::InvalidParsing)
                };
                let mut characters: Vec<String> = Vec::new();
                for character in b.iter() {
                    characters.push(Self::render_character(character)?);
                }
                Ok(Box::new(Stack { invert, characters }))
            },
            RegEx::Capture(name, group, a) => Ok(
                Box::new(
                    Capture {
                        inner: Self::generate_diagram_element(a)?,
                        name: if let Some(n) = name {
                            n.clone()
                        } else {
                            format!("Group {}", group)
                        }
                    }
                )
            )
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
            CharacterType::Meta(a) => {
                match a {
                    MetaCharacter::Word(m) => Ok(format!("{}Word", if *m { "" } else { "Non-" })),
                    MetaCharacter::Digit(m) => Ok(format!("{}Digit", if *m { "" } else { "Non-" })),
                    MetaCharacter::Whitespace(m) => Ok(format!("{}Whitespace", if *m { "" } else { "Non-" })),
                    MetaCharacter::Any => Ok(String::from("Any"))
                }
            }
            _ => Err(Error::InvalidParsing),
        }
    }

    pub fn render_diagram(diagram: &Sequence<Box<dyn Draw>>) -> Result<Vec<String>, Error> {
        Ok(diagram.draw())
    }
}

