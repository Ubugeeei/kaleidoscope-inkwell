use ast::Expression;
use lex::token::Token;

use crate::Parser;

impl Parser {
    pub(super) fn parse_paren_expr(&mut self) -> Result<Expression, &'static str> {
        match self.current_token.clone() {
            Token::LParen => (),
            _ => return Err("Expected '(' character at start of parenthesized expression."),
        }

        self.next();

        let expr = self.parse_expr()?;

        match self.current_token {
            Token::RParen => (),
            _ => return Err("Expected ')' character at end of parenthesized expression."),
        }

        self.next();

        Ok(expr)
    }
}
