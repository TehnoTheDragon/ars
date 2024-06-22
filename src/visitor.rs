use std::{borrow::BorrowMut, collections::HashMap, sync::Arc};

use crate::ast::Node;

pub type VisitorFn = Box<dyn Fn(&Visitor, &Node) -> VisitorResult>;

#[derive(Debug, Clone)]
pub enum VisitorResult {
    String(String),
    None,
}

impl VisitorResult {
    pub fn as_string(&self) -> String {
        match self {
            VisitorResult::String(s) => s.to_string(),
            VisitorResult::None => panic!("Value is None"),
        }
    }
}

pub struct Visitor {
    pub visitors: HashMap<u32, VisitorFn>,
}

impl Visitor {
    pub fn new(visitors: Vec<(u32, VisitorFn)>) -> Self {
        let mut map = HashMap::new();
        for (id, visitor) in visitors {
            map.insert(id, visitor);
        }
        Self {
            visitors: map,
        }
    }

    pub fn visit(&self, node: &Node) -> VisitorResult {
        if let Some(visitor) = self.visitors.get(&node.kind) {
            return visitor(self, node);
        }
        panic!("Unknown node kind({}): {}", node.kind, node.label);
    }

    pub fn visit_child(&self, node: &Arc<Node>) -> VisitorResult {
        if let Some(visitor) = self.visitors.get(&node.kind) {
            return visitor(self, node);
        }
        panic!("Unknown node kind({}): {}", node.kind, node.label);
    }

    pub fn visit_children(&self, children: &Vec<Arc<Node>>) -> VisitorResult {
        for child in children {
            self.visit_child(&child);
        }
        VisitorResult::None
    }
}