use ast::Expression;
use lex::token::Token;

use crate::Parser;

impl Parser {
    pub(super) fn parse_id_expr(&mut self) -> Result<Expression, &'static str> {
        let id = match self.current_token.clone() {
            Token::Identifier(id) => id,
            _ => return Err("Expected identifier."),
        };

        self.next();

        match self.current_token {
            Token::LParen => {
                self.next();

                if let Token::RParen = self.current_token {
                    return Ok(Expression::Call {
                        fn_name: id,
                        args: vec![],
                    });
                }

                let mut args = vec![];

                loop {
                    args.push(self.parse_expr()?);

                    match self.current_token {
                        Token::Comma => (),
                        Token::RParen => break,
                        _ => return Err("Expected ',' character in function call."),
                    }

                    self.next();
                }

                self.next();

                Ok(Expression::Call { fn_name: id, args })
            }

            _ => Ok(Expression::Variable(id)),
        }
    }

    pub(super) fn parse_var_expr(&mut self) -> Result<Expression, &'static str> {
        self.next();

        let mut variables = Vec::new();

        loop {
            let name = match self.current_token.clone() {
                Token::Identifier(name) => name,
                _ => return Err("Expected identifier in 'var..in' declaration."),
            };

            self.next();

            let initializer = match self.current_token {
                Token::Op('=') => Some({
                    self.next();
                    self.parse_expr()?
                }),

                _ => None,
            };

            variables.push((name, initializer));

            match self.current_token.clone() {
                Token::Comma => {
                    self.next();
                }
                Token::In => {
                    self.next();
                    break;
                }
                _ => return Err("Expected comma or 'in' keyword in variable declaration."),
            }
        }

        let body = self.parse_expr()?;

        Ok(Expression::VarIn {
            variables,
            body: Box::new(body),
        })
    }
}
