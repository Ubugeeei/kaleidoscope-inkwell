use parse::Parser;

fn main() {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut parser = Parser::new(input);
        let result = parser.parse();
        println!("{:?}", result);
    }
}
