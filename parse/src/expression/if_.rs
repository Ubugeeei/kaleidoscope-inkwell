use ast::Expression;
use lex::token::Token;

use crate::Parser;

impl Parser {
    pub(super) fn parse_conditional_expr(&mut self) -> Result<Expression, &'static str> {
        self.next();

        let cond = self.parse_expr()?;

        match self.current_token {
            Token::Then => self.next(),
            _ => return Err("Expected 'then' keyword."),
        }

        let then = self.parse_expr()?;

        match self.current_token {
            Token::Else => self.next(),
            _ => return Err("Expected 'else' keyword."),
        }

        let otherwise = self.parse_expr()?;

        Ok(Expression::Conditional {
            cond: Box::new(cond),
            consequence: Box::new(then),
            alternative: Box::new(otherwise),
        })
    }
}
