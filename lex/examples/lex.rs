use std::io::Write;

use lex::{token::Token, Lexer};

fn main() {
    loop {
        print!("> ");
        let _ = std::io::stdout().flush();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let mut lexer = Lexer::new(input);
        let mut t = lexer.next();

        while t != Token::EOF {
            println!("{:?}", t);
            t = lexer.next();
        }
    }
}
