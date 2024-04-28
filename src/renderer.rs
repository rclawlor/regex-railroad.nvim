use crate::parser::RegEx;

pub struct RegExRenderer {
    _tree: RegEx,
}

impl RegExRenderer {
    pub fn new(tree: RegEx) -> RegExRenderer {
        RegExRenderer { _tree: tree }
    }
}
