use std::{borrow::BorrowMut, cell::Cell, sync::{Arc, RwLock}};

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    None,
}

#[derive(Debug)]
pub struct Node {
    pub kind: u32,
    pub label: String,
    pub value: RwLock<Value>,
    pub children: Vec<Arc<Node>>,
}

impl Node {
    pub fn new(kind: u32, label: &str, value: Value) -> Node {
        Node {
            kind,
            label: label.to_string(),
            value: RwLock::new(value),
            children: vec![],
        }
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(Arc::new(child));
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