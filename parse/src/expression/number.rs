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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        let mut parser = Parser::new("42".to_string());
        let result = parser.parse_number_expr();
        assert_eq!(result, Ok(Expression::Number(42.0)));
    }

    #[test]
    fn test_parse_number_float() {
        let mut parser = Parser::new("42.2".to_string());
        let result = parser.parse_number_expr();
        assert_eq!(result, Ok(Expression::Number(42.2)));
    }

    #[test]
    fn test_parse_number_fail() {
        let mut parser = Parser::new("42.2.1".to_string());
        let result = parser.parse_number_expr();
        assert_eq!(result, Err("Expected number literal."));
    }
}
