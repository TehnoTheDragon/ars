use crate::token::{Token, TokenData};

#[derive(Debug, Clone)]
pub struct Lexer {
    // inputs

    input: String,
    patterns: Vec<Token>,

    /// last position before tokenizing
    start: usize,
    /// current position of the cursor
    current: usize,

    // metadata

    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(patterns: Vec<Token>) -> Self {
        Self {
            input: String::new(),
            patterns,
            start: 0,
            current: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn begin(&mut self, input: &str) {
        self.input = input.to_string();
        self.start = 0;
        self.current = 0;
        self.line = 1;
        self.column = 1;
    }

    pub fn all(&mut self) -> Vec<TokenData> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next() {
            tokens.push(token);
        }
        tokens
    }

    pub fn next(&mut self) -> Option<TokenData> {
        self.start = self.current;
        self.tokenize()
    }

    fn tokenize(&mut self) -> Option<TokenData> {
        if self.current >= self.input.len() {
            return None;
        }

        for pattern in self.patterns.iter() {
            if let Some(matched) = pattern.check(&self.input[self.current..]) {
                self.current += matched + 1;
                self.column += matched + 1;
                let token_data = TokenData::new(
                    pattern.id,
                    // we add 1 because check return index but slice starts from index 1 (probably)
                    self.input[self.start..self.start + matched + 1].to_string(),
                    pattern.label.clone(),
                    (self.line, self.column),
                    (self.start, self.current));
                return Some(token_data);
            }
        }

        panic!("\x1b[31;1mInvalid character\x1b[0m {:?}\n - line {}\n - column: {}", self.input.chars().nth(self.start).unwrap(), self.line, self.column);
    }
}

pub trait LexerExt {
    fn tokenize_all(&self, patterns: &Vec<Token>) -> Vec<TokenData>;
}

impl LexerExt for &'static str {
    fn tokenize_all(&self, patterns: &Vec<Token>) -> Vec<TokenData> {
        let mut lexer = Lexer::new(patterns.clone());
        lexer.begin(self);
        lexer.all()
    }
}

impl LexerExt for String {
    fn tokenize_all(&self, patterns: &Vec<Token>) -> Vec<TokenData> {
        let mut lexer = Lexer::new(patterns.clone());
        lexer.begin(self);
        lexer.all()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new(vec![
            Token::new_regex_from_str("whitespace", 0, "\\s+"),
            Token::new_lit("let", 1, "let"),
        ]);
        lexer.begin("let\nlet");
        assert_eq!(lexer.next().unwrap().label, "let");
        assert_eq!(lexer.next().unwrap().label, "whitespace");
        assert_eq!(lexer.next().unwrap().label, "let");
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_lexer_error() {
        let mut lexer = Lexer::new(vec![
            Token::new_regex_from_str("whitespace", 0, "\\s+"),
            Token::new_lit("let", 1, "let"),
        ]);
        lexer.begin("let\nlet\n");
        assert_eq!(lexer.next().unwrap().label, "let");
        assert_eq!(lexer.next().unwrap().label, "whitespace");
        assert_eq!(lexer.next().unwrap().label, "let");
        assert_eq!(lexer.next().unwrap().label, "whitespace");
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_variable_example() {
        let mut lexer = Lexer::new(vec![
            Token::new_regex_from_str("whitespace", 0, "\\s+"),
            Token::new_regex_from_str("number", 1, "\\d+(\\.\\d+)?"),
            Token::new_lit("let", 2, "let"),
            Token::new_regex_from_str("iden", 3, "[a-zA-Z_][a-zA-Z0-9_]*"),
            Token::new_lit("equal", 4, "="),
        ]);
        lexer.begin("let x = 10.5");
        assert_eq!(lexer.next().unwrap().label, "let");
        assert_eq!(lexer.next().unwrap().label, "whitespace");
        assert_eq!(lexer.next().unwrap().label, "iden");
        assert_eq!(lexer.next().unwrap().label, "whitespace");
        assert_eq!(lexer.next().unwrap().label, "equal");
        assert_eq!(lexer.next().unwrap().label, "whitespace");
        assert_eq!(lexer.next().unwrap().label, "number");
        assert_eq!(lexer.next(), None);
    }
}