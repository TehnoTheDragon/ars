use std::vec;

use ars::{ast::{Node, Value}, create_parser, lexer::Lexer, parser::ParserState, token::Token};

fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}

create_parser!(Register, 0, |_, state: &mut ParserState| {
    let register = state.eat();
    Node::new(0, "Register", Value::String(register.value))
});

create_parser!(Constant, 0, |_, state: &mut ParserState| {
    let constant = state.eat();
    Node::new(0, "Constant", Value::String(constant.value))
});

create_parser!(Identity, 0, |_, state: &mut ParserState| {
    match state.peek().kind {
        5 => { state.eat(); state.parse(Register) },
        7 => { state.eat(); state.parse(Constant) },
        _ => {
            let identity = state.require(vec![2]);
            let node = Node::new(0, "Ident", Value::String(identity.value));
            node
        },
    }
});

create_parser!(Instruction, 0, |_, state: &mut ParserState| {
    let instruction = state.require(vec![1]);
    let mut node = Node::new(0, "Instruction", Value::String(instruction.value));
    state.skip_until_found(vec![0]);
    node.add_child(state.parse(Identity));
    while state.is_kind(vec![4]) {
        state.eat();
        node.add_child(state.parse(Identity));
    }
    node
});

create_parser!(Label, 0, |_, state: &mut ParserState| {
    let identity = state.parse(Identity);
    state.require(vec![3]);
    state.eat();
    let mut label = Node::new(0, "Label", Value::String(identity.value.as_string()));
    while state.is_kind(vec![1, 2]) {
        match state.peek().kind {
            1 => label.add_child(state.parse(Instruction)),
            2 => label.add_child(state.parse(Label)),
            _ => unreachable!(),
        }
        state.skip_until_found(vec![0]);
    }
    label
});

fn main() {
    let mut lexer = Lexer::new(vec![
        Token::new_regex_from_str("whitespace", 0, "\\s+"),
        Token::new_lit("colon", 3, ":"),
        Token::new_lit("comma", 4, ","),
        Token::new_lit("percent", 5, "%"),
        Token::new_lit("hashtag", 7, "#"),
        Token::new_regex_from_str("instruction", 1, "imm|store|load|add|sub|mul|div|mod|and|or|xor|shl|shr|not"),
        Token::new_regex_from_str("ident", 2, "[a-zA-Z_][a-zA-Z0-9_]*"),
        Token::new_regex_from_str("number", 6, "\\d+(\\.\\d+)?"),
    ]);
    lexer.begin(read_file("assets/tests/pseudo-assembly.pasm").as_str());
    let tokens = lexer.all();
    let mut parser = ParserState::new(tokens, Some(vec![0]));
    println!("{}", parser.parse(Label));
}