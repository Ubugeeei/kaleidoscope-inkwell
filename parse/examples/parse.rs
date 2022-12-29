use std::io::Write;

use parse::Parser;

fn main() {
    loop {
        print!("> ");
        let _ = std::io::stdout().flush();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut parser = Parser::new(input);
        let result = parser.parse();
        println!("{:?}", result);
    }
}
