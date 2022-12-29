use ast::Expression;
use lex::token::Token;

use crate::Parser;

impl Parser {
    pub(super) fn parse_unary_expr(&mut self) -> Result<Expression, &'static str> {
        let op = match self.current_token {
            Token::Op(ch) => {
                self.next();
                ch
            }
            _ => return self.parse_primary(),
        };

        let mut name = String::from("unary");

        name.push(op);

        Ok(Expression::Call {
            fn_name: name,
            args: vec![self.parse_unary_expr()?],
        })
    }

    pub(super) fn parse_binary_expr(
        &mut self,
        prec: i32,
        mut left: Expression,
    ) -> Result<Expression, &'static str> {
        loop {
            let current_prec = self.get_tok_precedence();

            if current_prec < prec || self.current_token == Token::EOF {
                return Ok(left);
            }

            let op = match self.current_token {
                Token::Op(op) => op,
                _ => return Err("Invalid operator."),
            };

            self.next();

            let mut right = self.parse_unary_expr()?;

            let next_prec = self.get_tok_precedence();

            if current_prec < next_prec {
                right = self.parse_binary_expr(current_prec + 1, right)?;
            }

            left = Expression::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
    }
}
