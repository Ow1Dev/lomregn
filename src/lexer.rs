use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum Kind {
    Number(u32),
}

pub struct Lexer {
    source: VecDeque<char>,
}

pub fn new(source: &str) -> Lexer {
    Lexer {
        // trim is not nessary here as my iterator does it. but i have it here for performece
        // reason.
        source: source.trim().chars().collect(),
    }
}

impl Iterator for Lexer {
    type Item = Kind;

    fn next(&mut self) -> Option<Kind> {
        let mut val = Vec::new();
        while let Some(v) = self.source.pop_front() {
            if v.is_whitespace() {
                if !val.is_empty() {
                    break;
                }
                continue;
            }

            val.push(v)
        }

        if val.is_empty() {
            return None;
        }

        if val.iter().all(|c| c.is_digit(10)) {
            return Some(Kind::Number(
                val.into_iter().collect::<String>().parse().unwrap(),
            ));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexes_integer_literals() {
        let cases = [
            ("1", Kind::Number(1)),
            ("16", Kind::Number(16)),
            ("103", Kind::Number(103)),
        ];

        for (input, expected) in cases {
            let mut lexer = self::new(input);
            assert_eq!(lexer.next(), Some(expected));
        }
    }

    #[test]
    fn ignores_whitespace() {
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
    fn lexes_multiple_integer_literals() {
        let mut lexer = self::new("42  5");
        assert_eq!(lexer.next(), Some(Kind::Number(42)));
        assert_eq!(lexer.next(), Some(Kind::Number(5)));
    }
}
