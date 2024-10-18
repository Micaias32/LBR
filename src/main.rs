use std::ops::RangeBounds;

use lbr_compiler::{lexer::*, parser::ASTBuilder};

fn main() {
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
        let next = lexer.next_token().unwrap();
        println!("{:?}", next);
        tokens.push(next);
    }
    use lbr_compiler::lexer::Token::*;
    assert_eq!(
        vec![
            Ident("let".to_string()),
            Ident("x".to_string()),
            Assign,
            NumberLiteral("5".to_string()),
            Semicolon,
            Ident("let".to_string()),
            Ident("y".to_string()),
            Assign,
            NumberLiteral("7".to_string()),
            Semicolon,
            Ident("y".to_string()),
            Plus,
            Ident("x".to_string()),
            Semicolon,
            EOF,
        ],
        tokens
    );

    let mut ast_b = ASTBuilder::new(tokens);
    let ast = ast_b.get_ast().unwrap();
    dbg!(ast);
}
