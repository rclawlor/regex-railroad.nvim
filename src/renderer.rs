use crate::parser::RegEx;

pub struct RegExRenderer {
    tree: RegEx,
}

impl RegExRenderer {
    pub fn new(tree: RegEx) -> RegExRenderer {
        RegExRenderer { tree }
    }

    pub fn render(&self) {}
}
