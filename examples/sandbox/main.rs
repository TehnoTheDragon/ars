use ars::{ast::{Node, Value}, create_parser, lexer::Lexer, parser::{ParserState}, token::Token};

create_parser!(Identity, 0, |_, state: &mut ParserState| {
    state.require(vec![2]);
    let identity = state.eat();
    let node = Node::new(0, "Iden", Value::String(identity.value));
    node
});

create_parser!(Instruction, 0, |_, state: &mut ParserState| {
    state.require(vec![1]);
    let instruction = state.eat();
    let mut node = Node::new(0, "Instruction", Value::String(instruction.value));
    state.skip_until_found(vec![0]);
    node.add_child(state.parse(Identity));
    while state.is_kind(4) {
        state.eat();
        node.add_child(state.parse(Identity));
    }
    node
});

create_parser!(Label, 0, |_, state: &mut ParserState| {
    
    Node::new(0, "Instruction", Value::None)
});

fn main() {
    let mut lexer = Lexer::new(vec![
        Token::new_regex_from_str("whitespace", 0, "\\s+"),
        Token::new_lit("colon", 3, ":"),
        Token::new_lit("comma", 4, ","),
        Token::new_regex_from_str("instruction", 1, "imm|store"),
        Token::new_regex_from_str("ident", 2, "[a-zA-Z_][a-zA-Z0-9_]*"),
    ]);
    lexer.begin("store x");
    let mut parser = ParserState::new(lexer.all(), Some(vec![0]));
    println!("{}", parser.parse(Instruction));
}