use std::collections::HashMap;

use ast::Function;
use lex::{token::Token, Lexer};

mod expression;
mod extern_;
mod function;

pub struct Parser {
    lx: Lexer,
    current_token: Token,
    prec_map: HashMap<char, i32>,
}

impl Parser {
    pub fn new(input: String) -> Self {
        let mut lx = Lexer::new(input);
        let current_token = lx.next();

        Parser {
            lx,
            current_token,
            prec_map: {
                let mut p = HashMap::with_capacity(6);
                p.insert('=', 100);
                p.insert('<', 200);
                p.insert('>', 200);
                p.insert('+', 300);
                p.insert('-', 300);
                p.insert('*', 400);
                p.insert('/', 400);
                p
            },
        }
    }
}

impl Parser {
    pub fn parse(&mut self) -> Result<Function, &'static str> {
        let result = match self.current_token.clone() {
            Token::Def => self.parse_def(),
            Token::Extern => self.parse_extern(),
            _ => self.parse_toplevel_expr(),
        };

        match result {
            Ok(result) => {
                if self.current_token == Token::EOF {
                    Err("Unexpected token after parsed expression.")
                } else {
                    Ok(result)
                }
            }
            err => err,
        }
    }
}

impl Parser {
    fn next(&mut self) {
        self.current_token = self.lx.next();
    }

    fn get_tok_precedence(&self) -> i32 {
        if let Token::Op(op) = self.current_token {
            *self.prec_map.get(&op).unwrap_or(&100)
        } else {
            -1
        }
    }
}
