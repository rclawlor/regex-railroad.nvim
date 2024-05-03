#[derive(Eq, PartialEq, Debug)]
pub enum RegEx {
    Element(Vec<Box<RegEx>>),
    Repetition(RepetitionType, Box<RegEx>),
    Alternation(Box<RegEx>, Box<RegEx>),
    Terminal(char),
}

#[derive(Eq, PartialEq, Debug)]
pub enum RepetitionType {
    OrMore(u32),
    ZeroOrOne,
    Exactly(u32),
    Between(u32, u32),
}

pub struct RegExParser {
    text: String,
    idx: usize,
}

impl RegExParser {
    /// Create new instance of RegExParser
    pub fn new(text: &String) -> RegExParser {
        RegExParser {
            text: text.to_string(),
            idx: 0,
        }
    }

    pub fn parse(&mut self) -> Result<RegEx, String> {
        self.alternation()
    }

    fn alternation(&mut self) -> Result<RegEx, String> {
        let elem1 = self.element()?;
        // Check for OR
        if self.more() && self.peek() == '|' {
            self.consume('|').unwrap();
            let elem2 = self.element()?;
            Ok(RegEx::Alternation(Box::new(elem1), Box::new(elem2)))
        } else {
            Ok(elem1)
        }
    }

    fn element(&mut self) -> Result<RegEx, String> {
        let mut v = Vec::new();
        while self.more() && self.peek() != ')' && self.peek() != '|' {
            let r = self.repetition()?;
            v.push(Box::new(r));
        }
        Ok(RegEx::Element(v))
    }

    fn repetition(&mut self) -> Result<RegEx, String> {
        let b = self.group()?;
        if self.more() {
            match self.peek() {
                '*' => {
                    self.consume('*')?;
                    Ok(RegEx::Repetition(RepetitionType::OrMore(0), Box::new(b)))
                }
                '+' => {
                    self.consume('+')?;
                    Ok(RegEx::Repetition(RepetitionType::OrMore(1), Box::new(b)))
                }
                '?' => {
                    self.consume('?')?;
                    Ok(RegEx::Repetition(RepetitionType::ZeroOrOne, Box::new(b)))
                }
                '{' => Ok(RegEx::Repetition(self.repetition_group()?, Box::new(b))),
                _ => Ok(b),
            }
        } else {
            Ok(b)
        }
    }

    /// Find the type of repetition present
    fn repetition_group(&mut self) -> Result<RepetitionType, String> {
        self.consume('{')?;
        let mut min_count: u32 = 0;
        let mut max_count: Option<u32> = None;
        let mut two_num: bool = false;
        // Capture minimum count
        while self.more() && self.peek() != '}' {
            match self.peek() {
                num @ '0'..='9' => {
                    self.consume(num)?;
                    // Multiply by 10 to account for more than 1 digit numbers
                    min_count = min_count * 10
                        + num
                            .to_digit(10)
                            .expect("Current char already checked to be in '0'..='9'");
                }
                ',' => {
                    self.consume(',')?;
                    two_num = true;
                    break;
                }
                unknown => {
                    return Err(format!(
                        "Char '{}' not valid number for repetition group",
                        unknown
                    ));
                }
            }
        }
        // If maximum count is present, try to capture it
        while self.more() && self.peek() != '}' {
            match self.peek() {
                num @ '0'..='9' => {
                    match max_count {
                        Some(count) => {
                            max_count = Some(
                                10 * count
                                    + self
                                        .peek()
                                        .to_digit(10)
                                        .expect("Current char already checked to be in '0'..='9'"),
                            )
                        }
                        None => {
                            max_count = Some(
                                self.peek()
                                    .to_digit(10)
                                    .expect("Current char already checked to be in '0'..='9'"),
                            )
                        }
                    };
                    self.consume(num)?;
                }
                unknown => {
                    return Err(format!(
                        "Char '{}' not valid number for repetition group",
                        unknown
                    ));
                }
            }
        }

        if self.more() && self.peek() == '}' {
            self.consume('}')?;
        }
        // Return final repetition type based on numbers found
        match max_count {
            Some(max_count) => Ok(RepetitionType::Between(min_count, max_count)),
            None => {
                if two_num {
                    Ok(RepetitionType::OrMore(min_count))
                } else {
                    Ok(RepetitionType::Exactly(min_count))
                }
            }
        }
    }

    fn group(&mut self) -> Result<RegEx, String> {
        if self.peek() == '(' {
            self.consume('(').unwrap();
            let a = self.alternation()?;
            self.consume(')').unwrap();
            Ok(a)
        } else {
            Ok(RegEx::Terminal(self.next()?))
        }
    }

    /// Check what the next character is
    fn peek(&self) -> char {
        self.text.chars().nth(self.idx).unwrap()
    }

    /// 'Consume' char c from the text
    fn consume(&mut self, c: char) -> Result<(), String> {
        let p = self.peek();
        if p == c {
            self.idx += 1;
            Ok(())
        } else {
            Err(format!(
                "Character {} does not match current string {}",
                c, p
            ))
        }
    }

    /// Move to next character, consuming the current one
    fn next(&mut self) -> Result<char, String> {
        let c = self.peek();
        self.consume(c)?;
        Ok(c)
    }

    /// Returns true if the end of the string has been reached
    fn more(&self) -> bool {
        self.text.len() > self.idx
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{
        RegEx::{Alternation, Element, Repetition, Terminal},
        RegExParser, RepetitionType,
    };

    #[test]
    fn test_simple_regex() {
        let mut parser = RegExParser::new(&"a|b".to_string());
        assert_eq!(
            parser.parse().unwrap(),
            Alternation(
                Box::new(Element(vec![Box::new(Terminal('a'))])),
                Box::new(Element(vec![Box::new(Terminal('b'))]))
            )
        );

        let mut parser = RegExParser::new(&"a*".to_string());
        assert_eq!(
            parser.parse().unwrap(),
            Element(vec![Box::new(Repetition(
                RepetitionType::OrMore(0),
                Box::new(Terminal('a'))
            ))])
        );
    }

    #[test]
    fn test_moderate_regex() {
        let mut parser = RegExParser::new(&"(a|b)+".to_string());
        assert_eq!(
            parser.parse().unwrap(),
            Element(vec![Box::new(Repetition(
                RepetitionType::OrMore(1),
                Box::new(Alternation(
                    Box::new(Element(vec![Box::new(Terminal('a'))])),
                    Box::new(Element(vec![Box::new(Terminal('b'))]))
                ))
            ))])
        );
    }

    #[test]
    fn test_hard_regex() {
        let mut parser = RegExParser::new(&"a{8}".to_string());
        assert_eq!(
            parser.parse().unwrap(),
            Element(vec![Box::new(Repetition(
                RepetitionType::Exactly(8),
                Box::new(Terminal('a'))
            ))])
        );
        let mut parser = RegExParser::new(&"a{5,}".to_string());
        assert_eq!(
            parser.parse().unwrap(),
            Element(vec![Box::new(Repetition(
                RepetitionType::OrMore(5),
                Box::new(Terminal('a'))
            ))])
        );

        let mut parser = RegExParser::new(&"a{1,10}".to_string());
        assert_eq!(
            parser.parse().unwrap(),
            Element(vec![Box::new(Repetition(
                RepetitionType::Between(1, 10),
                Box::new(Terminal('a'))
            ))])
        );
    }
}
