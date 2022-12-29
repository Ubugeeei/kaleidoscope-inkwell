pub mod token;

pub struct Lexer {
    pub input_chars: Vec<char>,
    pub current_char: char,

    pub current_position: usize,
    pub read_position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input_chars: input.chars().collect(),
            current_position: 0,
            read_position: 0,
            current_char: '\0',
        };
        l.consume_char();
        l
    }
}

impl Lexer {
    pub fn next(&mut self) -> token::Token {
        self.consume_whitespace();
        let tok = match self.current_char {
            // symbols
            ',' => {
                self.consume_char();
                token::Token::Comma
            }
            '(' => {
                self.consume_char();
                token::Token::LParen
            }
            ')' => {
                self.consume_char();
                token::Token::RParen
            }
            '#' => {
                self.consume_char();
                token::Token::Comment
            }
            // ops
            '+' => {
                self.consume_char();
                token::Token::Op('+')
            }
            '-' => {
                self.consume_char();
                token::Token::Op('-')
            }
            '*' => {
                self.consume_char();
                token::Token::Op('*')
            }
            '/' => {
                self.consume_char();
                token::Token::Op('/')
            }
            '<' => {
                self.consume_char();
                token::Token::Op('<')
            }
            '>' => {
                self.consume_char();
                token::Token::Op('>')
            }
            '=' => {
                self.consume_char();
                token::Token::Op('=')
            }
            '\0' => {
                self.consume_char();
                token::Token::EOF
            }
            // multi-char
            _ => {
                if self.current_char.is_alphabetic() {
                    self.lex_word()
                } else if self.current_char.is_numeric() {
                    self.lex_number()
                } else {
                    token::Token::Illegal(self.current_char.to_string())
                }
            }
        };

        tok
    }
}

// lex word
impl Lexer {
    fn lex_word(&mut self) -> token::Token {
        let word = self.consume_word();
        self.ward_into_token(word)
    }

    fn consume_word(&mut self) -> String {
        let mut word = String::new();
        while self.current_char.is_alphanumeric() || self.current_char == '_' {
            word.push(self.current_char);
            self.consume_char();
        }
        word
    }

    fn ward_into_token(&mut self, word: String) -> token::Token {
        match word.as_str() {
            "binary" => token::Token::Binary,
            "unary" => token::Token::Unary,
            "def" => token::Token::Def,
            "var" => token::Token::Var,
            "extern" => token::Token::Extern,
            "if" => token::Token::If,
            "then" => token::Token::Then,
            "else" => token::Token::Else,
            "for" => token::Token::For,
            "in" => token::Token::In,
            _ => token::Token::Identifier(word),
        }
    }
}

// lex number
impl Lexer {
    fn lex_number(&mut self) -> token::Token {
        let mut number = String::new();
        while self.current_char.is_numeric() || self.current_char == '.' {
            number.push(self.current_char);
            self.consume_char();
        }
        match number.parse::<f64>() {
            Ok(n) => token::Token::Number(n),
            Err(_) => token::Token::Illegal(number),
        }
    }
}

impl Lexer {
    fn consume_char(&mut self) {
        if self.read_position >= self.input_chars.len() {
            self.current_char = '\0';
        } else {
            self.current_char = self.input_chars[self.read_position];
        }
        self.current_position = self.read_position;
        self.read_position += 1;
    }

    fn consume_whitespace(&mut self) {
        while self.current_char.is_whitespace() {
            self.consume_char();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        let input = String::from("def fib(n) if n < 2 then n else fib(n - 1) + fib(n - 2)");
        let mut l = Lexer::new(input);
        assert_eq!(l.next(), token::Token::Def);
        assert_eq!(l.next(), token::Token::Identifier(String::from("fib")));
        assert_eq!(l.next(), token::Token::LParen);
        assert_eq!(l.next(), token::Token::Identifier(String::from("n")));
        assert_eq!(l.next(), token::Token::RParen);
        assert_eq!(l.next(), token::Token::If);
        assert_eq!(l.next(), token::Token::Identifier(String::from("n")));
        assert_eq!(l.next(), token::Token::Op('<'));
        assert_eq!(l.next(), token::Token::Number(2.0));
        assert_eq!(l.next(), token::Token::Then);
        assert_eq!(l.next(), token::Token::Identifier(String::from("n")));
        assert_eq!(l.next(), token::Token::Else);
        assert_eq!(l.next(), token::Token::Identifier(String::from("fib")));
        assert_eq!(l.next(), token::Token::LParen);
        assert_eq!(l.next(), token::Token::Identifier(String::from("n")));
        assert_eq!(l.next(), token::Token::Op('-'));
        assert_eq!(l.next(), token::Token::Number(1.0));
        assert_eq!(l.next(), token::Token::RParen);
        assert_eq!(l.next(), token::Token::Op('+'));
        assert_eq!(l.next(), token::Token::Identifier(String::from("fib")));
        assert_eq!(l.next(), token::Token::LParen);
        assert_eq!(l.next(), token::Token::Identifier(String::from("n")));
        assert_eq!(l.next(), token::Token::Op('-'));
        assert_eq!(l.next(), token::Token::Number(2.0));
        assert_eq!(l.next(), token::Token::RParen);
        assert_eq!(l.next(), token::Token::EOF);
    }

    #[test]
    fn test_lex_symbol() {
        let input = String::from(",()#");
        let mut l = Lexer::new(input);
        assert_eq!(l.next(), token::Token::Comma);
        assert_eq!(l.next(), token::Token::LParen);
        assert_eq!(l.next(), token::Token::RParen);
        assert_eq!(l.next(), token::Token::Comment);
        assert_eq!(l.next(), token::Token::EOF);
    }

    #[test]
    fn test_lex_op() {
        let input = String::from("+-*/<>=");
        let mut l = Lexer::new(input);
        assert_eq!(l.next(), token::Token::Op('+'));
        assert_eq!(l.next(), token::Token::Op('-'));
        assert_eq!(l.next(), token::Token::Op('*'));
        assert_eq!(l.next(), token::Token::Op('/'));
        assert_eq!(l.next(), token::Token::Op('<'));
        assert_eq!(l.next(), token::Token::Op('>'));
        assert_eq!(l.next(), token::Token::Op('='));
        assert_eq!(l.next(), token::Token::EOF);
    }

    #[test]
    fn test_lex_word() {
        let input = String::from("hoge binary unary def var extern if then else for in");
        let mut l = Lexer::new(input);
        assert_eq!(l.next(), token::Token::Identifier(String::from("hoge")));
        assert_eq!(l.next(), token::Token::Binary);
        assert_eq!(l.next(), token::Token::Unary);
        assert_eq!(l.next(), token::Token::Def);
        assert_eq!(l.next(), token::Token::Var);
        assert_eq!(l.next(), token::Token::Extern);
        assert_eq!(l.next(), token::Token::If);
        assert_eq!(l.next(), token::Token::Then);
        assert_eq!(l.next(), token::Token::Else);
        assert_eq!(l.next(), token::Token::For);
        assert_eq!(l.next(), token::Token::In);
        assert_eq!(l.next(), token::Token::EOF);
    }

    #[test]
    fn test_lex_number() {
        let input = String::from("1 1.0 1.1 1.1.1");
        let mut l = Lexer::new(input);
        assert_eq!(l.next(), token::Token::Number(1.0));
        assert_eq!(l.next(), token::Token::Number(1.0));
        assert_eq!(l.next(), token::Token::Number(1.1));
        assert_eq!(l.next(), token::Token::Illegal(String::from("1.1.1")));
        assert_eq!(l.next(), token::Token::EOF);
    }

    #[test]
    fn test_whitespace() {
        let input = String::from(
            r#"
        
        
        
        , 
                (
                  
                  "#,
        );
        let mut l = Lexer::new(input);
        assert_eq!(l.next(), token::Token::Comma);
        assert_eq!(l.next(), token::Token::LParen);
    }
}
