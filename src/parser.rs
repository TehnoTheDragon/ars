use core::panic;

use crate::{ast::Node, token::TokenData};

pub trait Parser {
    fn parse(&self, state: &mut ParserState) -> Node;
}

#[derive(Debug, Clone)]
pub struct ParserState {
    pub skip_kinds: Vec<u32>,
    pub tokens: Vec<TokenData>,
    index: usize,
}

impl ParserState {
    pub fn new(tokens: Vec<TokenData>, skip_kinds: Option<Vec<u32>>) -> Self {
        Self {
            skip_kinds: skip_kinds.unwrap_or(vec![]),
            tokens: tokens,
            index: 0,
        }
    }

    pub fn parse(&mut self, parser: impl Parser) -> Node {
        self.skip_until_found(self.skip_kinds.clone());
        let mut sandbox_state = self.clone();
        let node = parser.parse(&mut sandbox_state);
        self.index = sandbox_state.index;
        return node;
    }

    pub fn require(&mut self, kinds: Vec<u32>) -> TokenData {
        let token = &self.tokens[self.index];
        if !kinds.contains(&token.kind) {
            panic!("Expected {:?} but found {:?}", kinds, token.kind);
        }
        self.eat()
    }

    pub fn is_kind(&self, kinds: Vec<u32>) -> bool {
        if self.index >= self.tokens.len() {
            return false;
        }
        kinds.contains(&self.peek().kind)
    }

    pub fn peek(&self) -> TokenData {
        self.tokens[self.index].clone()
    }

    pub fn eat(&mut self) -> TokenData {
        self.index += 1;
        self.tokens[self.index - 1].clone()
    }

    pub fn skip_until_found(&mut self, kinds: Vec<u32>) {
        while self.index < self.tokens.len() && kinds.contains(&self.peek().kind) {
            self.eat();
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.index >= self.tokens.len()
    }

}

#[macro_export]
macro_rules! create_parser {
    ( $name:ident, $kind:literal, $logic:expr ) => {
        pub struct $name;
        impl ars::parser::Parser for $name {
            fn parse(&self, _state: &mut ParserState) -> Node {($logic)(self, _state)}
        }
    };
}