use super::token::Token;

pub struct Lexer {
    pub input_chars: Vec<char>,
    pub current_char: char,

    pub current_position: usize,
    pub read_position: usize,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input_chars: input.chars().collect(),
            current_position: 0,
            read_position: 0,
            current_char: '\0',
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input_chars.len() {
            self.current_char = '\0';
        } else {
            self.current_char = self.input_chars[self.read_position];
        }
        self.current_position = self.read_position;
        self.read_position += 1;
    }
}

impl Lexer {
    fn next(&mut self) -> Token {
        let tok = match self.current_char {
            ',' => Token::Comma,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '#' => Token::Comment,
            '\0' => Token::EOF,
            _ => Token::Illegal,
        };
        self.read_char();
        tok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        let input = String::from(",()#");
        let mut l = Lexer::new(input);
        assert_eq!(l.next(), Token::Comma);
        assert_eq!(l.next(), Token::LParen);
        assert_eq!(l.next(), Token::RParen);
        assert_eq!(l.next(), Token::Comment);
        assert_eq!(l.next(), Token::EOF);
    }
}
