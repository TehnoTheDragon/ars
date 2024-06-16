use std::fmt::Display;

use regex;

const RESET: &str = "\x1b[0m";
const YELLOW: &str = "\x1b[33m";
const CYAN: &str = "\x1b[36m";
const BLUE: &str = "\x1b[34m";
const VALUE_DECOR: &str = "\x1b[31;3m";
const GREEN: &str = "\x1b[32m";

#[derive(Debug, Clone)]
pub struct Token {
    pub label: String,
    pub id: u32,
    pub token: TokenValue,
}

#[derive(Debug, Clone)]
pub enum TokenValue {
    Lit(String),
    Range(char, char),
    Regex(regex::Regex),
    URegex(regex::bytes::Regex),
}

impl Token {
    pub fn new(label: &str, id: u32, token: TokenValue) -> Self {
        Self {
            label: label.to_string(),
            id: id,
            token: token,
        }
    }

    pub fn new_lit(label: &str, id: u32, lit: &str) -> Self {
        Self::new(label, id, TokenValue::Lit(lit.to_string()))
    }

    pub fn new_lit_from_str(label: &str, id: u32, lit: String) -> Self {
        Self::new(label, id, TokenValue::Lit(lit))
    }

    pub fn new_range(label: &str, id: u32, start: char, end: char) -> Self {
        Self::new(label, id, TokenValue::Range(start, end))
    }

    pub fn new_uregex(label: &str, id: u32, regex: regex::bytes::Regex) -> Self {
        Self::new(label, id, TokenValue::URegex(regex))
    }

    pub fn new_uregex_from_str(label: &str, id: u32, regex: &str) -> Self {
        Self::new_uregex(label, id, regex::bytes::Regex::new(regex).unwrap())
    }

    pub fn new_regex(label: &str, id: u32, regex: regex::Regex) -> Self {
        Self::new(label, id, TokenValue::Regex(regex))
    }

    pub fn new_regex_from_str(label: &str, id: u32, regex: &str) -> Self {
        Self::new_regex(label, id, regex::Regex::new(regex).unwrap())
    }

    /// takes a string and return an index of last matched character  
    /// if not exist returns None
    pub fn check(&self, text: &str) -> Option<usize> {
        if text.len() == 0 {
            return None;
        }

        match &self.token {
            TokenValue::Lit(lit) => {
                if text.starts_with(lit) {
                    Some(lit.len() - 1)
                } else {
                    None
                }
            }
            TokenValue::Range(start, end) => {
                let mut counter = 0;
                for c in text.chars() {
                    if c >= *start && c <= *end {
                        counter += 1;
                    } else {
                        break;
                    }
                }
                if counter > 0 {
                    Some(counter - 1)
                } else {
                    None
                }
            }
            TokenValue::Regex(regex) => {
                if let Some(caps) = regex.find(text) {
                    if caps.start() != 0 {
                        return None;
                    }
                    Some(caps.end() - 1)
                } else {
                    None
                }
            }
            TokenValue::URegex(regex) => {
                if let Some(caps) = regex.find(text.as_bytes()) {
                    if caps.start() != 0 {
                        return None;
                    }
                    Some(caps.end() - 1)
                } else {
                    None
                }
            }
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{GREEN}{}{RESET}\n", self.label)?;
        write!(f, "└┬─ {VALUE_DECOR}id:{RESET} {YELLOW}{}{RESET}\n", self.id)?;
        write!(f, " └─ {CYAN}token{RESET}\n")?;
        match &self.token {
            TokenValue::Lit(lit) => write!(f, "    └─ {VALUE_DECOR}lit:{RESET} {BLUE}{}{RESET}", lit)?,
            TokenValue::Regex(regex) => write!(f, "    └─ {VALUE_DECOR}regex:{RESET} {BLUE}{}{RESET}", regex.as_str())?,
            TokenValue::URegex(regex) => write!(f, "    └─ {VALUE_DECOR}uregex:{RESET} {BLUE}{}{RESET}", regex.as_str())?,
            TokenValue::Range(start, end) => {
                write!(f, "    ├─ {VALUE_DECOR}start:{RESET} {BLUE}{}{RESET}\n", start)?;
                write!(f, "    └─ {VALUE_DECOR}end:{RESET} {BLUE}{}{RESET}", end)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenData {
    pub kind: u32,
    pub value: String,
    pub label: String,

    /// (start, end)
    pub location: (usize, usize),

    /// (line, column)
    pub span: (usize, usize),
}

impl TokenData {
    pub fn new(kind: u32, value: String, label: String, location: (usize, usize), span: (usize, usize)) -> Self {
        Self {
            kind: kind,
            label: label,
            value: value,
            location: location,
            span: span,
        }
    }

    pub fn from_str(text: &str, kind: u32) -> Self {
        Self::new(kind, text.to_string(), text.to_string(), (0, text.len()), (0, text.len()))
    }
}

pub trait TokenExt {
    fn to_token(&self, kind: u32) -> TokenData;
}

impl TokenExt for &'static str {
    fn to_token(&self, kind: u32) -> TokenData {
        TokenData::from_str(self, kind)
    }
}

impl TokenExt for String {
    fn to_token(&self, kind: u32) -> TokenData {
        TokenData::from_str(self.as_str(), kind)
    }
}

impl Display for TokenData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{GREEN}{}{RESET}\n", self.label)?;
        write!(f, "└┬─ {VALUE_DECOR}value:{RESET} {BLUE}{}{RESET}\n", self.value)?;
        write!(f, " ├─ {CYAN}location{RESET}\n")?;
        write!(f, " │  ├─ {VALUE_DECOR}line:{RESET} {YELLOW}{}{RESET}\n", self.location.0)?;
        write!(f, " │  └─ {VALUE_DECOR}column:{RESET} {YELLOW}{}{RESET}\n", self.location.1)?;
        write!(f, " └─ {CYAN}span{RESET}\n")?;
        write!(f, "    ├─ {VALUE_DECOR}start:{RESET} {YELLOW}{}{RESET}\n", self.span.0)?;
        write!(f, "    └─ {VALUE_DECOR}end:{RESET} {YELLOW}{}{RESET}\n", self.span.1)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Testing Token

    #[test]
    fn test_check_lit() {
        {
            let token = Token::new_lit("test", 0, "test");
            assert_eq!(token.check("test"), Some(3));
        }
        {
            let token = Token::new_lit("test", 0, "test");
            assert_eq!(token.check("test1"), Some(3));
        }
        {
            let token = Token::new_lit("test", 0, "test");
            assert_eq!(token.check("testtest"), Some(3));
        }
        {
            let token = Token::new_lit("test", 0, "test");
            assert_eq!(token.check("1test"), None);
        }
    }

    #[test]
    fn test_check_range() {
        {
            let token = Token::new_range("test", 0, 'a', 'z');
            assert_eq!(token.check("hello world"), Some(4));
        }
        {
            let token = Token::new_range("test", 0, 'a', 'z');
            assert_eq!(token.check("test1"), Some(3));
        }
        {
            let token = Token::new_range("test", 0, 'a', 'z');
            assert_eq!(token.check("testtest"), Some(7));
        }
        {
            let token = Token::new_range("test", 0, 'a', 'z');
            assert_eq!(token.check("1test"), None);
        }
    }

    #[test]
    fn test_check_regex() {
        {
            let token = Token::new_regex_from_str("test", 0, "\\d+");
            assert_eq!(token.check("1234"), Some(3));
        }
        {
            let token = Token::new_regex_from_str("test", 0, "[a-z]+");
            assert_eq!(token.check("test1"), Some(3));
        }
        {
            let token = Token::new_regex_from_str("test", 0, "[\\w|-]+");
            assert_eq!(token.check("test-test"), Some(8));
        }
        {
            let token = Token::new_regex_from_str("test", 0, "2");
            assert_eq!(token.check("1test"), None);
        }
    }

    #[test]
    fn test_check_uregex() {
        {
            let token = Token::new_uregex_from_str("test", 0, "🙂+");
            assert_eq!(token.check("🙂🙂🙂"), Some(11));
        }
        {
            let token = Token::new_uregex_from_str("test", 0, "\\)+");
            assert_eq!(token.check(")))"), Some(2));
        }
        {
            let token = Token::new_uregex_from_str("test", 0, "[\u{1F600}-\u{1F64F}]+");
            assert_eq!(token.check("😀😁😂"), Some(11));
        }
        {
            let token = Token::new_uregex_from_str("test", 0, "[\u{1F680}-\u{1F6FF}]+");
            assert_eq!(token.check("🚀🚐🚑"), Some(11));
        }
    }
}