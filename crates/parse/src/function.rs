use ast::{Function, Prototype};
use lex::token::Token;

use crate::Parser;

impl Parser {
    pub(super) fn parse_def(&mut self) -> Result<Function, &'static str> {
        self.next();

        let proto = self.parse_prototype()?;

        let body = self.parse_expr()?;

        Ok(Function {
            prototype: proto,
            body: Some(body),
            is_anon: false,
        })
    }

    pub(super) fn parse_prototype(&mut self) -> Result<Prototype, &'static str> {
        let (id, is_operator, precedence) = match self.current_token.clone() {
            Token::Identifier(id) => {
                self.next();
                (id, false, 0)
            }

            Token::Binary => {
                self.next();

                let op = match self.current_token {
                    Token::Op(ch) => ch,
                    _ => return Err("Expected operator in custom operator declaration."),
                };

                self.next();

                let mut name = String::from("binary");

                name.push(op);

                let prec = if let Token::Number(prec) = self.current_token {
                    self.next();
                    prec as usize
                } else {
                    0
                };

                self.prec_map.insert(op, prec as i32);

                (name, true, prec)
            }

            Token::Unary => {
                self.next();

                let op = match self.current_token {
                    Token::Op(ch) => ch,
                    _ => return Err("Expected operator in custom operator declaration."),
                };

                let mut name = String::from("unary");

                name.push(op);

                self.next();

                (name, true, 0)
            }

            _ => return Err("Expected identifier in prototype declaration."),
        };

        match self.current_token.clone() {
            Token::LParen => (),
            _ => return Err("Expected '(' character in prototype declaration."),
        }

        self.next();

        if let Token::RParen = self.current_token {
            self.next();

            return Ok(Prototype {
                name: id,
                args: vec![],
                is_op: is_operator,
                prec: precedence,
            });
        }

        let mut args = vec![];

        loop {
            match self.current_token.clone() {
                Token::Identifier(name) => args.push(name),
                _ => return Err("Expected identifier in parameter declaration."),
            }

            self.next();

            match self.current_token {
                Token::RParen => {
                    self.next();
                    break;
                }
                Token::Comma => {
                    self.next();
                }
                _ => return Err("Expected ',' or ')' character in prototype declaration."),
            }
        }

        Ok(Prototype {
            name: id,
            args,
            is_op: is_operator,
            prec: precedence,
        })
    }
}
