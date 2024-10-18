use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    EOF,
    Ident(Ident),
    StringLiteral(String),
    NumberLiteral(String),
    Assign,
    Equal,
    Plus,
    Minus,
    Semicolon,
    OpenParen,
    CloseParen,
    OpenSquirly,
    CloseSquirly,
    GreaterThan,
    LessThan,
    KeyWord(KW),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KW {
    Function,
    While,
    For,
    Loop,
    Val,
    Var,
    If,
    Else,
    In,
    Use,
    True,
    False,
    Break,
    Continue,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct Ident(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lexer {
    position: usize,
    next_pos: usize,
    input: Vec<u8>,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut out = Lexer {
            position: 0,
            next_pos: 0,
            input: input.into_bytes(),
            ch: 0,
        };
        out.next_char();
        return out;
    }

    pub fn next_token(&mut self) -> Result<Token> {
        use Token as TK;
        let mut needs_nexting = true;
        self.skip_whitespace();

        let tok = match self.ch {
            b'{' => TK::OpenSquirly,
            b'}' => TK::CloseSquirly,
            b'(' => TK::OpenParen,
            b')' => TK::CloseParen,
            b';' => TK::Semicolon,
            b'>' => TK::GreaterThan,
            b'<' => TK::LessThan,
            b'-' => TK::Minus,
            b'+' => TK::Plus,
            b'=' => {
                if self.peek() == b'=' {
                    self.next_char();
                    TK::Equal
                } else {
                    TK::Assign
                }
            }
            b'0'..=b'9' => {
                needs_nexting = false;
                TK::NumberLiteral(self.read_num())
            }
            b'"' => TK::StringLiteral(self.read_string()), // String Parsing
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                needs_nexting = false;
                let ident = self.read_ident();
                match ident.as_str() {
                    "fn" => TK::KeyWord(KW::Function),
                    "enquanto" => TK::KeyWord(KW::While),
                    "para" => TK::KeyWord(KW::For),
                    "loop" => TK::KeyWord(KW::Loop),
                    "se" => TK::KeyWord(KW::If),
                    "senao" => TK::KeyWord(KW::Else),
                    "val" => TK::KeyWord(KW::Val),
                    "var" => TK::KeyWord(KW::Var),
                    "em" => TK::KeyWord(KW::In),
                    "use" => TK::KeyWord(KW::Use),
                    "verdadeiro" => TK::KeyWord(KW::True),
                    "falso" => TK::KeyWord(KW::False),
                    "pare" => TK::KeyWord(KW::Break),
                    "continue" => TK::KeyWord(KW::Continue),
                    _ => TK::Ident(Ident(ident)),
                }
            }
            0 => TK::EOF,
            _ => unreachable!(),
        };

        if needs_nexting {
            self.next_char();
        }
        Ok(tok)
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.next_char();
        }
    }

    fn peek(&self) -> u8 {
        if self.next_pos >= self.input.len() {
            0
        } else {
            self.input[self.next_pos]
        }
    }

    fn next_char(&mut self) {
        if self.next_pos >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.next_pos]
        }

        self.position = self.next_pos;
        self.next_pos += 1;
    }

    fn read_ident(&mut self) -> String {
        let start = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.next_char();
        }

        String::from_utf8_lossy(&self.input[start..self.position]).to_string()
    }

    fn read_num(&mut self) -> String {
        let start = self.position;
        while self.ch.is_ascii_digit() {
            self.next_char();
        }

        String::from_utf8_lossy(&self.input[start..self.position]).to_string()
    }

    fn read_string(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexin() {
        let mut lexer = Lexer::new(
            "
        let x = 5;
        let y = 7;
        y + x;
        "
            .to_string(),
        );

        let mut tokens = Vec::new();

        while tokens.last() != Some(&Token::EOF) {
            tokens.push(lexer.next_token().unwrap());
        }
        use super::Token::*;
        assert_eq!(
            vec![
                TK::Ident("let".to_string()),
                TK::Ident("x".to_string()),
                TK::Assign,
                TK::NumberLiteral("5".to_string()),
                TK::Semicolon,
                TK::Ident("let".to_string()),
                TK::Ident("y".to_string()),
                TK::Assign,
                TK::NumberLiteral("7".to_string()),
                TK::Semicolon,
                TK::Ident("y".to_string()),
                TK::Plus,
                TK::Ident("x".to_string()),
                TK::Semicolon,
                TK::EOF,
            ],
            tokens
        );
    }

    #[test]
    fn lexin_but_complete() {
        let mut lexer = Lexer::new(
            "
use console;

fn principal() {
    val x = 4;
    var y = 7;
    escrevaf(x + y);
    se (x > 7) {
        enquanto (verdadeiro) {
            loop { pare }
        }
    } senao {
        para (var i = 0; i < y; i+=1) {}
    }
}
        "
            .to_string(),
        );

        let mut tokens = Vec::new();

        while tokens.last() != Some(&Token::EOF) {
            tokens.push(lexer.next_token().unwrap());
        }

        use super::{Token as TK, KW};
        assert_eq!(
            vec![
                // use console;
                TK::KeyWord(KW::Use),
                TK::Ident("console".to_string()),
                TK::Semicolon,
                // fn principal() {
                TK::KeyWord(KW::Function),
                TK::Ident("principal".to_string()),
                TK::OpenParen,
                TK::CloseParen,
                TK::OpenSquirly,
                // val x = 4;
                TK::KeyWord(KW::Val),
                TK::Ident("x".to_string()),
                TK::Assign,
                TK::NumberLiteral("4".to_string()),
                TK::Semicolon,
                // var y = 7;
                TK::KeyWord(KW::Var),
                TK::Ident("y".to_string()),
                TK::Assign,
                TK::NumberLiteral("7".to_string()),
                TK::Semicolon,
                // escrevaf(x + y);
                TK::Ident("escrevaf".to_string()),
                TK::OpenParen,
                TK::Ident("x".to_string()),
                TK::Plus,
                TK::Ident("y".to_string()),
                TK::CloseParen,
                TK::Semicolon,
                // se (x > 7) {
                TK::KeyWord(KW::If),
                TK::OpenParen,
                TK::Ident("x".to_string()),
                TK::GreaterThan,
                TK::NumberLiteral("7".to_string()),
                TK::CloseParen,
                TK::OpenSquirly,
                // enquanto (verdadeiro) {
                TK::KeyWord(While),
                TK::OpenParen,
                TK::KeyWord(True),
                TK::CloseParen,
                TK::OpenSquirly,
                // loop { pare }
                TK::KeyWord(Loop),
                TK::OpenSquirly,
                TK::KeyWord(KW::Break),
                TK::CloseSquirly,
                // }
                TK::CloseSquirly,
                // } senao {
                TK::CloseSquirly,
                TK::KeyWord(Else),
                TK::OpenSquirly,
                // para (var i = 0; i < y; i+=1) {}
                TK::KeyWord(For),
                TK::OpenParen,
                TK::KeyWord(Var),
                TK::Ident("i".to_string()),
                TK::Assign,
                TK::NumberLiteral("0".to_string()),
                TK::Semicolon,
                TK::Ident("i".to_string()),
                TK::LessThan,
                TK::Ident("y".to_string()),
                TK::Semicolon,
                TK::Ident(Ident("i".to_string())),
                TK::Plus,
                TK::Assign,
                TK::NumberLiteral("1".to_string()),
                TK::CloseParen,
                TK::OpenSquirly,
                TK::CloseSquirly,
                // }
                TK::CloseSquirly,
                // }
                TK::CloseSquirly,
                TK::EOF,
            ],
            tokens
        );
    }
}
