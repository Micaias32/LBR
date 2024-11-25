use std::{collections::HashSet, rc::Rc};

use crate::parser::*;
use pest::{error::Error, iterators::Pair, Parser};

pub struct Ast {
    nodes: Vec<Node>,
    idents: HashSet<Rc<str>>,
}

pub enum Node {
    Assignment {
        ident: Rc<str>,
        assignee: Box<Node>,
    },
    Declaration {
        ident: Rc<str>,
        assignee: Box<Node>,
        mutability: Mutability,
    },
    FunctionCall {
        ident: Rc<str>,
        arguments: Vec<Node>,
    },
    UnaryExpr {
        operator: MathOperator,
        term: Box<Node>,
    },
    BinaryExpr {
        lhs: Box<Node>,
        operator: MathOperator,
        rhs: Box<Node>,
    },
}

pub enum Mutability {
    Constant,
    Value,
    Variable,
}

pub enum MathOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl Default for Ast {
    fn default() -> Self {
        Self::new()
    }
}

impl Ast {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            idents: HashSet::new(),
        }
    }

    pub fn parse(&mut self, source: &str) -> Result<(), Box<Error<Rule>>> {
        let mut nodes = vec![];

        let pairs = match LbrParser::parse(Rule::grammar_rules, source) {
            Ok(p) => p,
            Err(e) => return Err(e.into()),
        };

        for pair in pairs {
            if let Rule::statement = pair.as_rule() {
                let node = self.get_node_from_statement(pair);
                nodes.push(node);
            }
        }

        Ok(())
    }

    fn get_node_from_statement(&mut self, pair: Pair<Rule>) -> Node {
        let nested = pair.into_inner().next().unwrap();
        match (nested.as_rule(), nested) {
            (Rule::function_call, pair) => self.get_function_call(pair),
            (Rule::var_decl, pair) => self.get_val_declaration(pair),
            _ => unreachable!(),
        }
    }

    fn get_function_call(&mut self, pair: Pair<Rule>) -> Node {
        let pairs = pair.into_inner();
    }

    fn get_val_declaration(&self, pair: Pair<Rule>) -> Node {
        todo!()
    }
}
