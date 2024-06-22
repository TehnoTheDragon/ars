use std::{borrow::{Borrow, BorrowMut}, cell::Cell, fmt::{Display, Write}, ops::Deref, sync::{Arc, RwLock, RwLockReadGuard}};

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    None,
}

impl Value {
    pub fn as_string(&self) -> String {
        match self {
            Value::String(s) => s.to_string(),
            Value::None => panic!("Value is None"),
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub kind: u32,
    pub label: String,
    pub value: Value,
    pub children: Vec<Arc<Node>>,
}

impl Node {
    pub fn new(kind: u32, label: &str, value: Value) -> Node {
        Node {
            kind,
            label: label.to_string(),
            value: value,
            children: vec![],
        }
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(Arc::new(child));
    }

    pub fn display(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) -> std::fmt::Result {
        let line_color = format!("\x1b[38;2;{};{};{}m", 120, 120, 120);
        let number_color = format!("\x1b[38;2;{};{};{}m", 240, 140, 30);
        let label_color = format!("\x1b[38;2;{};{};{}m", 200, 30, 200);
        let field_color = format!("\x1b[38;2;{};{};{}m", 150, 150, 170);
        let string_color = format!("\x1b[38;2;{};{};{}m", 60, 180, 100);
        let none_color = format!("\x1b[38;2;{};{};{}m", 255, 100, 100);
        let stop = "\x1b[0m";

        let space = " ".repeat(depth * 2);
        write!(f, "{label_color}{}{stop}\n", self.label)?;
        write!(f, "{space}{line_color}└┬─{stop} {field_color}value:{field_color} ")?;
        match &self.value {
            Value::String(s) => write!(f, "{string_color}{}{stop}\n", s)?,
            Value::None => write!(f, "{none_color}none{stop}\n")?,
        }
        write!(f, "{space} {line_color}└─{stop} {field_color}kind:{field_color} {number_color}{}{stop}\n", self.kind)?;
        for child in &self.children {
            write!(f, "{space}    {line_color}┌─{stop} ")?;
            child.display(f, depth + 2)?;
        }
        Ok(())
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display(f, 0)
    }
}

#[macro_export]
macro_rules! node {
    ( $kind:literal, $label:literal, $value:expr ) => {
        Node::new($kind, $label, $value)
    };
}

#[macro_export]
macro_rules! add_node {
    ( $root:expr, $node:expr, $( $rest:expr ),* ) => {
        $root.add_child($node);
        $( $root.add_child($rest) )*
    };

    ( $root:expr, $node:expr ) => {
        $root.add_child($node)
    };

    ( $root:expr ) => {}
}