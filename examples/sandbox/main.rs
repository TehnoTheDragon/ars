use std::{borrow::Borrow, fmt::Write, io::Write as IoWrite, rc::Rc, sync::{Arc, Mutex, RwLock}, thread::Scope, vec};

use ars::{ast::{Node, Value}, create_parser, lexer::Lexer, parser::ParserState, token::Token, visitor::{Visitor, VisitorResult}};

fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}

create_parser!(Register, 4, |_, state: &mut ParserState| {
    let register = state.eat();
    Node::new(4, "Register", Value::String(register.value))
});

create_parser!(Constant, 3, |_, state: &mut ParserState| {
    let constant = state.eat();
    Node::new(3, "Constant", Value::String(constant.value))
});

create_parser!(Identity, 2, |_, state: &mut ParserState| {
    match state.peek().kind {
        5 => { state.eat(); state.parse(Register) },
        7 => { state.eat(); state.parse(Constant) },
        _ => {
            let identity = state.require(vec![2]);
            let node = Node::new(2, "Ident", Value::String(identity.value));
            node
        },
    }
});

create_parser!(Instruction, 1, |_, state: &mut ParserState| {
    let instruction = state.require(vec![1]);
    let mut node = Node::new(1, "Instruction", Value::String(instruction.value));
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

struct MyScope {
    pub data: std::fs::File,
}

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
    let mut my_scope = MyScope {
        data: std::fs::File::create("assets/tests/pseudo-assembly.nasm").unwrap(),
    };
    let mut visitor = Visitor::new(&mut my_scope);
    
    visitor.register(0, Box::new(|visitor, node| {
        visitor.scope.lock().unwrap().data.write_fmt(format_args!(".{}:\n", node.value.as_string())).unwrap();
        visitor.visit_children(node);
        VisitorResult::None
    }));

    visitor.register(1, Box::new(|visitor, node| {
        visitor.scope.lock().unwrap().data.write_fmt(format_args!("\t{} ", node.value.as_string())).unwrap();
        for (i, child) in node.children.iter().enumerate() {
            visitor.visit(child);
            if i < node.children.len() - 1 {
                visitor.scope.lock().unwrap().data.write_fmt(format_args!(", ")).unwrap();
            }
        }
        visitor.scope.lock().unwrap().data.write_fmt(format_args!("\n")).unwrap();
        VisitorResult::None
    }));

    visitor.register(2, Box::new(|visitor, node| {
        visitor.scope.lock().unwrap().data.write_fmt(format_args!("[{}]", node.value.as_string())).unwrap();
        VisitorResult::None
    }));

    visitor.register(3, Box::new(|visitor, node| {
        visitor.scope.lock().unwrap().data.write_fmt(format_args!("#{}", node.value.as_string())).unwrap();
        VisitorResult::None
    }));

    visitor.register(4, Box::new(|visitor, node| {
        visitor.scope.lock().unwrap().data.write_fmt(format_args!("r{}", node.value.as_string())).unwrap();
        VisitorResult::None
    }));

    visitor.visit(&parser.parse(Label));
}