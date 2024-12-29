use pest::{error::Error, iterators::Pair, Parser};

use crate::parser::{LbrParser, Rule};

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd)]
pub struct AST {
    statements: Vec<Statement>,
}
impl AST {
    pub fn new(source: &str) -> Result<Self, Box<Error<Rule>>> {
        let pairs = LbrParser::parse(Rule::grammar_rules, source)?
            .next()
            .expect("infallible")
            .into_inner();

        let mut statements = vec![];

        for pair in pairs {
            if pair.as_rule() == Rule::EOI {
                break;
            }
            let statement = Statement::from_pair(pair);
            statements.push(statement);
        }

        Ok(Self { statements })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd)]
pub enum Statement {
    Declaration {
        ident: Ident,
        value: Expression,
    },
    FunctionCall {
        func_ident: Ident,
        args: Vec<Expression>,
    },
    Expression(Expression),
}
impl Statement {
    fn from_pair(pair: Pair<Rule>) -> Statement {
        let statement = pair.into_inner().next().expect("infallible");

        match statement.as_rule() {
            Rule::declaration => Statement::from_declaration(statement),
            Rule::function_call => Statement::from_func_call(statement),
            Rule::expr => Statement::Expression(Expression::from_pair(statement)),
            _ => unreachable!("infallible"),
        }
    }

    fn from_func_call(pair: Pair<Rule>) -> Statement {
        let pairs: Vec<_> = pair.into_inner().collect();
        let (ident, arguments) = (&pairs[0], pairs.get(1).cloned());
        let mut args: Vec<_> = Vec::new();
        if let Some(arguments) = arguments {
            args = arguments
                .into_inner()
                .map(|pair| Expression::from_pair(pair))
                .collect();
        }
        Statement::FunctionCall {
            func_ident: Ident {
                name: ident.as_str().into(),
            },
            args,
        }
    }

    fn from_declaration(pair: Pair<Rule>) -> Statement {
        let pairs: Vec<_> = pair.into_inner().collect();

        let (ident, value) = (
            Ident {
                name: pairs[0].as_str().into(),
            },
            Expression::from_pair(pairs[1].to_owned()),
        );

        Statement::Declaration { ident, value }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd)]
pub enum Expression {
    UnaryExpr {
        term: Box<Expression>,
        operator: Option<Operator>,
    },
    BinaryExpr {
        lhs: Box<Expression>,
        operator: Operator,
        rhs: Box<Expression>,
    },
    NumberLiteral {
        val: String,
    },
    Identifier {
        ident: Ident,
    },
}
impl Expression {
    fn from_pair(pair: Pair<Rule>) -> Expression {
        let expr = pair.into_inner();

        let expr: Vec<_> = expr.collect();

        let term1 = Expression::from_term(expr[0].to_owned());
        if let Some(rule) = expr.get(1..=2) {
            let operator = match rule[0].as_str() {
                "+" => Operator::Plus,
                "-" => Operator::Minus,
                "*" => Operator::Multiply,
                "/" => Operator::Divide,
                _ => unreachable!("infallible"),
            };
            let term2 = Expression::from_term(rule[1].to_owned());

            Expression::BinaryExpr {
                lhs: term1.into(),
                operator,
                rhs: term2.into(),
            }
        } else {
            term1
        }
    }

    fn from_term(pair: Pair<Rule>) -> Expression {
        let pair = pair.into_inner().next().expect("infallible");
        match pair.as_rule() {
            Rule::number => Expression::NumberLiteral {
                val: pair.as_str().into(),
            },
            Rule::identifier => Expression::Identifier {
                ident: Ident {
                    name: pair.as_str().into(),
                },
            },
            Rule::expr => Expression::from_pair(pair),
            _ => unreachable!("infallible"),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd)]
pub struct Ident {
    name: String,
}
