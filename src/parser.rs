use crate::lexer::Token;
use anyhow::Result;

#[derive(Debug, PartialEq)]
pub struct ASTBuilder {
    tokens: Vec<Token>,
    index: usize,
}

impl ASTBuilder {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    pub fn get_ast(&mut self) -> Result<AST> {
        let mut ast = AST::new();
        while self.tokens[self.index] != Token::EOF {
            ast.statements.push(self.get_statement().unwrap());
        }
        Ok(ast)
    }

    fn get_statement(&mut self) -> Option<Statement> {
        todo!()
    }

    fn get_block(&mut self) -> Result<Statement> {
        todo!()
    }

    fn get_function_declaration(&mut self) -> Result<Statement, String> {
        let ident = match self.tokens[self.index + 1] {
            Token::Ident(s) => s,
            _ => return Err("Syntax Error".to_string()),
        };
        let func = Statement::FunctionDeclaration {
            ident,
            body: Box::new(self.get_block().unwrap()),
            ret_type: Type {
                name: "i32".to_string(),
                size: 4,
            },
        };
        Ok(func)
    }
}

#[derive(Debug, PartialEq)]
pub struct AST {
    statements: Vec<Statement>,
}

impl AST {
    fn new() -> Self {
        Self { statements: vec![] }
    }
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    FunctionDeclaration {
        ident: Ident,
        body: Box<Statement>,
        ret_type: Type,
    },
    Use {
        path: String,
    },
    Call,
    Binding,
    Expression(Expression),
    CodeBlock(Vec<Statement>),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    MathExpr {
        op: Operator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Division,
}

#[derive(Debug, PartialEq)]
pub struct Type {
    name: String,
    size: usize,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn build_ast() {
        let mut ast_b = ASTBuilder::new(vec![Token::EOF]);
        let _ = ast_b.get_ast();
        assert_eq!(true, true);
    }
}
