use ast::Expression;
use lex::token::Token;

use crate::Parser;

impl Parser {
    pub(super) fn parse_number_expr(&mut self) -> Result<Expression, &'static str> {
        match self.current_token {
            Token::Number(nb) => {
                self.next();
                Ok(Expression::Number(nb))
            }
            _ => Err("Expected number literal."),
        }
    }
}
