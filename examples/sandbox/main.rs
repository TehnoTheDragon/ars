use ars::{lexer::{Lexer, LexerExt}, token::{Token, TokenData, TokenExt}};

fn main() {
    let pattern = vec![
        Token::new_regex_from_str("whitespace", 0, "\\s+"),
        Token::new_regex_from_str("number", 4, "\\d+(\\.\\d+)?"),
        Token::new_lit("let", 1, "let"),
        Token::new_regex_from_str("iden", 2, "[a-zA-Z_][a-zA-Z0-9_]*"),
        Token::new_lit("equal", 3, "="),
    ];
    for tok in "let x = 10.5".tokenize_all(&pattern).iter() {
        if tok.kind == 0 { continue; }
        println!("{tok}");
    }
}