use crate::{error::Error, extract::{Language, STRING_FORMAT}};
use lazy_static::lazy_static;
use tracing::info;

lazy_static! {
    static ref SPECIAL_CHARS: Vec<char> = vec!['(', ')', '[', ']', '+', '*', '$', '|', '^', '{', '}'];
}

#[derive(Eq, PartialEq, Debug)]
pub enum RegEx {
    Element(Vec<Box<RegEx>>),
    Repetition(RepetitionType, Box<RegEx>),
    Alternation(Vec<Box<RegEx>>),
    Character(CharacterType),
    Anchor(AnchorType),
    Terminal(String),
    Capture(Option<String>, usize, Box<RegEx>)
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum RepetitionType {
    OrMore(u32),
    ZeroOrOne,
    Exactly(u32),
    Between(u32, u32),
}

#[derive(Eq, PartialEq, Debug)]
pub enum CharacterType {
    Any(Vec<Box<CharacterType>>),
    Not(Vec<Box<CharacterType>>),
    Between(Box<CharacterType>, Box<CharacterType>),
    Terminal(char),
    Meta(MetaCharacter)
}

#[derive(Eq, PartialEq, Debug)]
pub enum MetaCharacter {
    Word(bool),
    Digit(bool),
    Whitespace(bool),
    Any
}

#[derive(Eq, PartialEq, Debug)]
pub enum AnchorType {
    Start,
    End,
    WordBoundary,
    NotWordBoundary
}

pub struct RegExParser {
    language: Language,
    text: String,
    idx: usize,
    capture_group: usize
}

impl RegExParser {
    /// Create new instance of RegExParser
    pub fn new(language: Language, text: &String) -> RegExParser {
        RegExParser {
            language,
            text: text.to_string(),
            idx: 0,
            capture_group: 0
        }
    }

    pub fn parse(&mut self) -> Result<RegEx, Error> {
        self.alternation()
    }

    fn alternation(&mut self) -> Result<RegEx, Error> {
        let elem1 = self.element()?;
        if !self.more() || self.peek() != '|' {
            Ok(elem1)
        } else {
            // Check for OR
            let mut v = vec![Box::new(elem1)];
            while self.more() && self.peek() == '|' {
                self.consume('|').unwrap();
                v.push(Box::new(self.element()?));
            }
            Ok(RegEx::Alternation(v))
        }
    }

    fn element(&mut self) -> Result<RegEx, Error> {
        let mut v = Vec::new();
        while self.more() && self.peek() != ')' && self.peek() != '|' {
            let r = self.repetition()?;
            v.push(Box::new(r));
        }
        Ok(RegEx::Element(v))
    }

    fn repetition(&mut self) -> Result<RegEx, Error> {
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
    fn repetition_group(&mut self) -> Result<RepetitionType, Error> {
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
                    return Err(Error::RepetitionValue(unknown));
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
                    return Err(Error::RepetitionValue(unknown));
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

    fn group(&mut self) -> Result<RegEx, Error> {
        if self.peek() == '(' {
            self.consume('(')?;
            let a = match self.peek() {
                '?' => {
                    self.consume('?')?;
                    if self.peek() == ':' {
                        // Unnamed capture group
                        self.consume(':')?;
                        self.capture_group += 1;
                        RegEx::Capture(None, self.capture_group, Box::new(self.alternation()?))
                    }
                    else if self.peek() == '<' {
                        // Named capture group
                        self.consume('<')?;
                        let mut name = String::new();
                        while self.more() && self.peek() != '>' {
                            name = format!("{}{}", name, self.next()?);
                        }
                        self.consume('>')?;
                        self.capture_group += 1;
                        RegEx::Capture(Some(name), self.capture_group, Box::new(self.alternation()?))
                    }
                    else {
                        return Err(Error::InvalidCharacter('?', self.idx))
                    }
                },
                _ => self.alternation()?
            };
            self.consume(')').unwrap();
            Ok(a)
        } else if self.peek() == '[' {
            self.consume('[').unwrap();
            let a = self.character()?;
            self.consume(']').unwrap();
            Ok(RegEx::Character(a))
        } else if self.peek() == '\\' {
            self.consume('\\')?;
            let character_type = match self.next()? {
                'w' => CharacterType::Meta(MetaCharacter::Word(true)),
                'W' => CharacterType::Meta(MetaCharacter::Word(false)),
                'd' => CharacterType::Meta(MetaCharacter::Digit(true)),
                'D' => CharacterType::Meta(MetaCharacter::Digit(false)),
                's' => CharacterType::Meta(MetaCharacter::Whitespace(true)),
                'S' => CharacterType::Meta(MetaCharacter::Whitespace(false)),
                other => return Ok(RegEx::Terminal(other.to_string()))
            };
            Ok(RegEx::Character(character_type))
        } else if self.peek() == '^' || self.peek() == '$' {
            match self.peek() {
                '^' => {
                    self.consume('^')?;
                    Ok(RegEx::Anchor(AnchorType::Start))
                },
                '$' => {
                    self.consume('$')?;
                    Ok(RegEx::Anchor(AnchorType::End))
                },
                _ => Ok(RegEx::Terminal(String::from("")))
            }
        } else if self.peek() == '.' {
            self.consume('.')?;
            Ok(RegEx::Character(CharacterType::Meta(MetaCharacter::Any)))
        } else {
            let mut string = String::from("");
            while self.more() && !SPECIAL_CHARS.contains(&self.peek()) {
                let fmt = STRING_FORMAT.get(&self.language).expect("Language is supported");
                if self.peek() == fmt.escape_char() {
                    self.consume(fmt.escape_char())?;
                }
                string = format!("{}{}", string, self.next()?);
            }
            Ok(RegEx::Terminal(string))
        }
    }

    fn character(&mut self) -> Result<CharacterType, Error> {
        let mut match_char = true;
        if self.peek() == '^' {
            self.consume('^').unwrap();
            match_char = false;
        }
        let mut v = Vec::new();
        while self.more() && self.peek() != ']' {
            let c = self.next_character()?;
            v.push(c);
        }
        if match_char {
            Ok(CharacterType::Any(v))
        } else {
            Ok(CharacterType::Not(v))
        }
    }

    fn next_character(&mut self) -> Result<Box<CharacterType>, Error> {
        let c = match self.peek() {
            digit_a @ '0'..='9' => {
                self.consume(digit_a).unwrap();
                if self.peek() == '-' {
                    self.consume('-').unwrap();
                    match self.peek() {
                        digit_b @ '0'..='9' => {
                            self.consume(digit_b).unwrap();
                            CharacterType::Between(
                                Box::new(CharacterType::Terminal(digit_a)),
                                Box::new(CharacterType::Terminal(digit_b)),
                            )
                        }
                        other => return Err(Error::CharacterRange(digit_a, other)),
                    }
                } else {
                    CharacterType::Terminal(digit_a)
                }
            }
            letter_a @ 'a'..='z' => {
                self.consume(letter_a).unwrap();
                if self.peek() == '-' {
                    self.consume('-').unwrap();
                    match self.peek() {
                        letter_b @ 'a'..='z' => {
                            self.consume(letter_b).unwrap();
                            CharacterType::Between(
                                Box::new(CharacterType::Terminal(letter_a)),
                                Box::new(CharacterType::Terminal(letter_b)),
                            )
                        }
                        other => return Err(Error::CharacterRange(letter_a, other)),
                    }
                } else {
                    CharacterType::Terminal(letter_a)
                }
            }
            capital_a @ 'A'..='Z' => {
                self.consume(capital_a).unwrap();
                if self.peek() == '-' {
                    self.consume('-').unwrap();
                    match self.peek() {
                        capital_b @ 'A'..='Z' => {
                            self.consume(capital_b).unwrap();
                            CharacterType::Between(
                                Box::new(CharacterType::Terminal(capital_a)),
                                Box::new(CharacterType::Terminal(capital_b)),
                            )
                        }
                        other => return Err(Error::CharacterRange(capital_a, other)),
                    }
                } else {
                    CharacterType::Terminal(capital_a)
                }
            },
            '\\' => {
                self.consume('\\')?;
                match self.peek() {
                    'w' => {
                        self.consume('w')?;
                        CharacterType::Meta(MetaCharacter::Word(true))
                    },
                    'W' => {
                        self.consume('W')?;
                        CharacterType::Meta(MetaCharacter::Word(false))
                    },
                    'd' => {
                        self.consume('d')?;
                        CharacterType::Meta(MetaCharacter::Digit(true))
                    },
                    'D' => {
                        self.consume('D')?;
                        CharacterType::Meta(MetaCharacter::Digit(false))
                    },
                    's' => {
                        self.consume('s')?;
                        CharacterType::Meta(MetaCharacter::Whitespace(true))
                    },
                    'S' => {
                        self.consume('S')?;
                        CharacterType::Meta(MetaCharacter::Whitespace(false))
                    },
                    _ => CharacterType::Terminal('\\')
                }
            }
            other => {
                info!("Character {}", other);
                self.consume(other)?;
                CharacterType::Terminal(other)
            }
        };
        if self.peek() == '-' {
            if self.peek_n(1) == Some(']') {
                Ok(Box::new(c))
            } else {
                Ok(Box::new(CharacterType::Between(
                    Box::new(c),
                    self.next_character()?,
                )))
            }
        } else {
            Ok(Box::new(c))
        }
    }

    /// Check what the next character is
    fn peek(&self) -> char {
        self.text.chars().nth(self.idx).unwrap()
    }

    /// Check n characters ahead
    fn peek_n(&self, n: usize) -> Option<char> {
        self.text.chars().nth(self.idx + n)
    }

    /// 'Consume' char c from the text
    fn consume(&mut self, c: char) -> Result<(), Error> {
        let p = self.peek();
        if p == c {
            self.idx += 1;
            Ok(())
        } else {
            Err(Error::StringIterator(c, p))
        }
    }

    /// Move to next character, consuming the current one
    fn next(&mut self) -> Result<char, Error> {
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
    use crate::{extract::Language, parser::{
        RegEx::{Alternation, Element, Repetition, Terminal},
        RegExParser, RepetitionType,
    }};

    #[test]
    fn test_simple_regex() {
        let mut parser = RegExParser::new(Language::Rust, &"a|b".to_string());
        assert_eq!(
            parser.parse().unwrap(),
            Alternation(vec![
                Box::new(Element(vec![Box::new(Terminal('a'.to_string()))])),
                Box::new(Element(vec![Box::new(Terminal('b'.to_string()))]))
            ])
        );

        let mut parser = RegExParser::new(Language::Rust, &"a*".to_string());
        assert_eq!(
            parser.parse().unwrap(),
            Element(vec![Box::new(Repetition(
                RepetitionType::OrMore(0),
                Box::new(Terminal('a'.to_string()))
            ))])
        );
    }

    #[test]
    fn test_moderate_regex() {
        let mut parser = RegExParser::new(Language::Rust, &"(a|b)+".to_string());
        assert_eq!(
            parser.parse().unwrap(),
            Element(vec![Box::new(Repetition(
                RepetitionType::OrMore(1),
                Box::new(Alternation(vec![
                    Box::new(Element(vec![Box::new(Terminal('a'.to_string()))])),
                    Box::new(Element(vec![Box::new(Terminal('b'.to_string()))]))
                ]))
            ))])
        );
    }

    #[test]
    fn test_hard_regex() {
        let mut parser = RegExParser::new(Language::Rust, &"a{8}".to_string());
        assert_eq!(
            parser.parse().unwrap(),
            Element(vec![Box::new(Repetition(
                RepetitionType::Exactly(8),
                Box::new(Terminal('a'.to_string()))
            ))])
        );
        let mut parser = RegExParser::new(Language::Rust, &"a{5,}".to_string());
        assert_eq!(
            parser.parse().unwrap(),
            Element(vec![Box::new(Repetition(
                RepetitionType::OrMore(5),
                Box::new(Terminal('a'.to_string()))
            ))])
        );

        let mut parser = RegExParser::new(Language::Rust, &"a{1,10}".to_string());
        assert_eq!(
            parser.parse().unwrap(),
            Element(vec![Box::new(Repetition(
                RepetitionType::Between(1, 10),
                Box::new(Terminal('a'.to_string()))
            ))])
        );
    }
}
