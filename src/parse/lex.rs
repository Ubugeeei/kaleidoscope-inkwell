use super::token::Token;

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
    pub fn next(&mut self) -> Token {
        self.consume_whitespace();
        let tok = match self.current_char {
            ',' => Token::Comma,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '#' => Token::Comment,
            '+' => Token::Op(String::from("+")),
            '-' => Token::Op(String::from("-")),
            '*' => Token::Op(String::from("*")),
            '/' => Token::Op(String::from("/")),
            '<' => Token::Op(String::from("<")),
            '>' => Token::Op(String::from(">")),
            '=' => Token::Op(String::from("=")),
            '\0' => Token::EOF,
            _ => {
                if self.current_char.is_alphabetic() {
                    self.lex_word()
                } else if self.current_char.is_numeric() {
                    self.lex_number()
                } else {
                    Token::Illegal(self.current_char.to_string())
                }
            }
        };
        self.consume_char();
        tok
    }
}

// lex word
impl Lexer {
    fn lex_word(&mut self) -> Token {
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

    fn ward_into_token(&mut self, word: String) -> Token {
        match word.as_str() {
            "binary" => Token::Binary,
            "unary" => Token::Unary,
            "def" => Token::Def,
            "var" => Token::Var,
            "extern" => Token::Extern,
            "if" => Token::If,
            "then" => Token::Then,
            "else" => Token::Else,
            "for" => Token::For,
            "in" => Token::In,
            _ => Token::Identifier(word),
        }
    }
}

// lex number
impl Lexer {
    fn lex_number(&mut self) -> Token {
        let mut number = String::new();
        while self.current_char.is_numeric() || self.current_char == '.' {
            number.push(self.current_char);
            self.consume_char();
        }
        match number.parse::<f64>() {
            Ok(n) => Token::Number(n),
            Err(_) => Token::Illegal(number),
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
        assert_eq!(l.next(), Token::Def);
        assert_eq!(l.next(), Token::Identifier(String::from("fib")));
        assert_eq!(l.next(), Token::LParen);
        assert_eq!(l.next(), Token::Identifier(String::from("n")));
        assert_eq!(l.next(), Token::RParen);
        assert_eq!(l.next(), Token::If);
        assert_eq!(l.next(), Token::Identifier(String::from("n")));
        assert_eq!(l.next(), Token::Op(String::from("<")));
        assert_eq!(l.next(), Token::Number(2.0));
        assert_eq!(l.next(), Token::Then);
        assert_eq!(l.next(), Token::Identifier(String::from("n")));
        assert_eq!(l.next(), Token::Else);
        assert_eq!(l.next(), Token::Identifier(String::from("fib")));
        assert_eq!(l.next(), Token::LParen);
        assert_eq!(l.next(), Token::Identifier(String::from("n")));
        assert_eq!(l.next(), Token::Op(String::from("-")));
        assert_eq!(l.next(), Token::Number(1.0));
        assert_eq!(l.next(), Token::RParen);
        assert_eq!(l.next(), Token::Op(String::from("+")));
        assert_eq!(l.next(), Token::Identifier(String::from("fib")));
        assert_eq!(l.next(), Token::LParen);
        assert_eq!(l.next(), Token::Identifier(String::from("n")));
        assert_eq!(l.next(), Token::Op(String::from("-")));
        assert_eq!(l.next(), Token::Number(2.0));
        assert_eq!(l.next(), Token::RParen);
        assert_eq!(l.next(), Token::EOF);
    }

    #[test]
    fn test_lex_symbol() {
        let input = String::from(",()#");
        let mut l = Lexer::new(input);
        assert_eq!(l.next(), Token::Comma);
        assert_eq!(l.next(), Token::LParen);
        assert_eq!(l.next(), Token::RParen);
        assert_eq!(l.next(), Token::Comment);
        assert_eq!(l.next(), Token::EOF);
    }

    #[test]
    fn test_lex_op() {
        let input = String::from("+-*/<>=");
        let mut l = Lexer::new(input);
        assert_eq!(l.next(), Token::Op(String::from("+")));
        assert_eq!(l.next(), Token::Op(String::from("-")));
        assert_eq!(l.next(), Token::Op(String::from("*")));
        assert_eq!(l.next(), Token::Op(String::from("/")));
        assert_eq!(l.next(), Token::Op(String::from("<")));
        assert_eq!(l.next(), Token::Op(String::from(">")));
        assert_eq!(l.next(), Token::Op(String::from("=")));
        assert_eq!(l.next(), Token::EOF);
    }

    #[test]
    fn test_lex_word() {
        let input = String::from("hoge binary unary def var extern if then else for in");
        let mut l = Lexer::new(input);
        assert_eq!(l.next(), Token::Identifier(String::from("hoge")));
        assert_eq!(l.next(), Token::Binary);
        assert_eq!(l.next(), Token::Unary);
        assert_eq!(l.next(), Token::Def);
        assert_eq!(l.next(), Token::Var);
        assert_eq!(l.next(), Token::Extern);
        assert_eq!(l.next(), Token::If);
        assert_eq!(l.next(), Token::Then);
        assert_eq!(l.next(), Token::Else);
        assert_eq!(l.next(), Token::For);
        assert_eq!(l.next(), Token::In);
        assert_eq!(l.next(), Token::EOF);
    }

    #[test]
    fn test_lex_number() {
        let input = String::from("1 1.0 1.1 1.1.1");
        let mut l = Lexer::new(input);
        assert_eq!(l.next(), Token::Number(1.0));
        assert_eq!(l.next(), Token::Number(1.0));
        assert_eq!(l.next(), Token::Number(1.1));
        assert_eq!(l.next(), Token::Illegal(String::from("1.1.1")));
        assert_eq!(l.next(), Token::EOF);
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
        assert_eq!(l.next(), Token::Comma);
        assert_eq!(l.next(), Token::LParen);
    }
}
