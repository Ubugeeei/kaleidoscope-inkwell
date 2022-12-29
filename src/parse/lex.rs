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

#[cfg(test)]
mod tests {
    use super::*;
}
