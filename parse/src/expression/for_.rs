use ast::Expression;
use lex::token::Token;

use crate::Parser;

impl Parser {
    pub(super) fn parse_for_expr(&mut self) -> Result<Expression, &'static str> {
        self.next();

        let name = match self.current_token.clone() {
            Token::Identifier(n) => n,
            _ => return Err("Expected identifier in for loop."),
        };

        self.next();

        match self.current_token {
            Token::Op('=') => self.next(),
            _ => return Err("Expected '=' character in for loop."),
        }

        let start = self.parse_expr()?;

        match self.current_token.clone() {
            Token::Comma => self.next(),
            _ => return Err("Expected ',' character in for loop."),
        }

        let end = self.parse_expr()?;

        let step = match self.current_token {
            Token::Comma => {
                self.next();

                Some(self.parse_expr()?)
            }

            _ => None,
        };

        match self.current_token {
            Token::In => self.next(),
            _ => return Err("Expected 'in' keyword in for loop."),
        }

        let body = self.parse_expr()?;

        Ok(Expression::For {
            var_name: name,
            start: Box::new(start),
            end: Box::new(end),
            step: step.map(Box::new),
            body: Box::new(body),
        })
    }
}
