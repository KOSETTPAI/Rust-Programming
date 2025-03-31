/// This module reads characters in arithmetic expression and converts them to tokens.
/// The allowed tokens are defined in ast module.
// Standard lib
use std::iter::Peekable;
use std::str::Chars;

//Other internal modules
use super::token::Token;

// Other structs

// Tokenizer struct contains a Peekable iterator on the arithmetic expression
pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>,
}

// Constructs a new instance of Tokenizer
impl<'a> Tokenizer<'a> {
    pub fn new(new_expr: &'a str) -> Self {
        Tokenizer {
            expr: new_expr.chars().peekable(),
        }
    }

    /// Extracts a numeric string from the expression.
    /// Handles integers and decimal numbers.
    pub fn parse_number(&mut self) -> String {
        let mut num = String::new();

        // Peek and consume characters while they are part of a number
        while let Some(&ch) = self.expr.peek() {
            if ch.is_numeric() || ch == '.' {
                num.push(ch);
                self.expr.next(); // Consume the character
            } else {
                break;
            }
        }

        num
    }
}

// Implement Iterator trait for Tokenizer struct.
// With this, we can use next() method on tokenizer to retrieve the next token from arithmetic expression
impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let next_char = self.expr.next();

        match next_char {
            Some('0'..='9') => {
                let mut num = String::new();
                num.push(next_char.unwrap());
                num.push_str(&self.parse_number());
                match num.parse::<f64>() {
                    Ok(num) => Some(Token::Num(num)),
                    Err(_) => None,
                }
            }

            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Subtract),
            Some('*') => Some(Token::Multiply),
            Some('/') => Some(Token::Divide),
            Some('^') => Some(Token::Caret),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            Some('&') => Some(Token::And),
            Some('|') => Some(Token::Or),

            None => Some(Token::EOF),
            Some(_) => None,
        }
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_integer() {
        let mut tokenizer = Tokenizer::new("34");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(34.0))
    }
    #[test]
    fn test_decimal_number() {
        let mut tokenizer = Tokenizer::new("34.5");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(34.5))
    }
    #[test]
    fn test_invalid_char() {
        let mut tokenizer = Tokenizer::new("#$%");
        assert_eq!(tokenizer.next(), None);
    }
}