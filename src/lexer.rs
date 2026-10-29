use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq)]
pub enum Kind {
    Number(u32),
    Plus,
    Minus,
    Slash,
    Star,
    Unknown,
}

pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
}

pub fn new(source: &str) -> Lexer<'_> {
    Lexer {
        // trim is not nessary here as my iterator does it. but i have it here for performece
        // reason.
        source: source.trim().chars().peekable(),
    }
}

impl Iterator for Lexer<'_> {
    type Item = Kind;

    fn next(&mut self) -> Option<Kind> {
        let c = self.source.find(|c| !c.is_ascii_whitespace())?;

        match c {
            '+' => Some(Kind::Plus),
            '-' => Some(Kind::Minus),
            '/' => Some(Kind::Slash),
            '*' => Some(Kind::Star),
            c if c.is_ascii_digit() => {
                let mut digit = Vec::new();
                digit.push(c);

                while let Some(v) = self.source.peek() {
                    if !v.is_ascii_digit() {
                        break;
                    }
                    digit.push(self.source.next()?)
                }

                Some(Kind::Number(
                    digit
                        .into_iter()
                        .collect::<String>()
                        .parse()
                        .expect("Something ws not a digit"),
                ))
            }
            _ => Some(Kind::Unknown),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_if_empty_return_none() {
        let mut lexer = self::new("");
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn lexer_kind_literals() {
        let cases = [
            ("1", Kind::Number(1)),
            ("16", Kind::Number(16)),
            ("103", Kind::Number(103)),
            ("+", Kind::Plus),
            ("/", Kind::Slash),
            ("*", Kind::Star),
            ("-", Kind::Minus),
        ];

        for (input, expected) in cases {
            let mut lexer = self::new(input);
            assert_eq!(lexer.next(), Some(expected));
        }
    }

    #[test]
    fn lexer_ignores_whitespace() {
        let cases = [
            ("  16", Kind::Number(16)),
            ("42  ", Kind::Number(42)),
            ("  93  ", Kind::Number(93)),
        ];

        for (input, expected) in cases {
            let mut lexer = self::new(input);
            assert_eq!(lexer.next(), Some(expected));
        }
    }
    #[test]
    fn lexer_multiple_kind_literals() {
        let mut lexer = self::new("42  + 5");
        assert_eq!(lexer.next(), Some(Kind::Number(42)));
        assert_eq!(lexer.next(), Some(Kind::Plus));
        assert_eq!(lexer.next(), Some(Kind::Number(5)));
    }
}
