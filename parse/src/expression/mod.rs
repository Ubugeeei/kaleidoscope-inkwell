use ast::{Expression, Function, Prototype};
use lex::token::Token;

use crate::Parser;

mod for_;
mod if_;
mod number;
mod ops;
mod paren;
mod var;

impl Parser {
    pub(super) fn parse_toplevel_expr(&mut self) -> Result<Function, &'static str> {
        match self.parse_expr() {
            Ok(expr) => Ok(Function {
                prototype: Prototype {
                    name: "anonymous".to_string(),
                    args: vec![],
                    is_op: false,
                    prec: 0,
                },
                body: Some(expr),
                is_anon: true,
            }),

            Err(err) => Err(err),
        }
    }

    pub(super) fn parse_expr(&mut self) -> Result<Expression, &'static str> {
        match self.parse_unary_expr() {
            Ok(left) => self.parse_binary_expr(0, left),
            err => err,
        }
    }

    pub(super) fn parse_primary(&mut self) -> Result<Expression, &'static str> {
        match self.current_token {
            Token::Identifier(_) => self.parse_id_expr(),
            Token::Number(_) => self.parse_number_expr(),
            Token::LParen => self.parse_paren_expr(),
            Token::If => self.parse_conditional_expr(),
            Token::For => self.parse_for_expr(),
            Token::Var => self.parse_var_expr(),
            _ => Err("Unknown expression."),
        }
    }
}
