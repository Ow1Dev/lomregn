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
        source: source.chars().collect(),
    }
}

impl Iterator for Lexer {
    type Item = Kind;

    fn next(&mut self) -> Option<Kind> {
        let mut val = Vec::new();
        while let Some(v) = self.source.pop_front() {
            if !v.is_whitespace() {
                val.push(v)
            }
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
            ("  16", Kind::Number(16)),
            ("42  ", Kind::Number(42)),
            ("  93  ", Kind::Number(93)),
            (" 103  ", Kind::Number(103)),
        ];

        for (input, expected) in cases {
            let mut lexer = self::new(input);
            let val = lexer.next().expect("got nothing here");
            assert_eq!(val, expected)
        }
    }
}
