pub const CODE: &str = "\
val a = 1;
val b = 2;
escreval(a + b);
";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Val,
    Ident(String),
    NumberLiteral(String),
    OpenParen,
    CloseParen,
    Semicolon,
    Assign,
    EndOfFile,
    Plus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tokenizer {
    code: Vec<u8>,
    tokens: Vec<Token>,
    pos: usize,
}

impl Tokenizer {
    pub fn new(code: String) -> Self {
        Self {
            code: code.into(),
            tokens: vec![],
            pos: 0,
        }
    }

    pub fn tokens(mut self) -> Result<Vec<Token>, String> {
        while self.pos != self.code.len() {
            let err = self.skip_whitespace();
            if let Err(()) = err {
                break;
            }
            let tok = match self.code[self.pos] {
                b'(' => Token::OpenParen,
                b')' => Token::CloseParen,
                b';' => Token::Semicolon,
                b'+' => Token::Plus,
                b'=' => Token::Assign,
                b'0'..=b'9' => self.get_num_literal(),
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    let ident = self.get_ident();
                    match ident.as_str() {
                        "val" => Token::Val,
                        _ => Token::Ident(ident),
                    }
                }
                _ => return Err("Unexpexted Token".to_string()),
            };
            self.tokens.push(tok);
            self.pos += 1;
        }
        self.tokens.push(Token::EndOfFile);
        Ok(self.tokens)
    }

    fn get_num_literal(&mut self) -> Token {
        let mut tok = String::from(self.code[self.pos] as char);
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                tok.push(c.into());
            } else {
                break;
            }
            self.pos += 1
        }
        Token::NumberLiteral(tok)
    }

    fn peek(&self) -> Option<u8> {
        if self.pos < self.code.len() + 1 {
            Some(self.code[self.pos + 1])
        } else {
            None
        }
    }

    fn get_ident(&mut self) -> String {
        let mut ident = String::from(self.code[self.pos] as char);
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == b'_' {
                ident.push(c.into());
            } else {
                break;
            }
            self.pos += 1;
        }
        ident
    }

    fn skip_whitespace(&mut self) -> Result<(), ()> {
        while self.code[self.pos].is_ascii_whitespace() {
            self.pos += 1;
            if self.pos >= self.code.len() {
                return Err(());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokeniz_this_mf() {
        let tokenizer = Tokenizer::new(CODE.to_string());
        let tokens = tokenizer.tokens().unwrap();
        use Token as TK;
        assert_eq!(
            tokens,
            vec![
                TK::Val,
                TK::Ident("a".to_string()),
                TK::Assign,
                TK::NumberLiteral("1".to_string()),
                TK::Semicolon,
                TK::Val,
                TK::Ident("b".to_string()),
                TK::Assign,
                TK::NumberLiteral("2".to_string()),
                TK::Semicolon,
                TK::Ident("escreval".to_string()),
                TK::OpenParen,
                TK::Ident("a".to_string()),
                TK::Plus,
                TK::Ident("b".to_string()),
                TK::CloseParen,
                TK::Semicolon,
                TK::EndOfFile,
            ]
        );
    }
}
