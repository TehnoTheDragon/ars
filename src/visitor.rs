use std::{borrow::BorrowMut, cell::{Ref, RefCell}, collections::HashMap, fmt::{format, Display}, ops::ShrAssign, rc::Rc, sync::{Arc, Mutex, RwLock}};

use crate::ast::Node;

#[derive(Debug, Clone)]
pub enum VisitorResult {
    Compound(Vec<VisitorResult>),
    String(String),
    Integer(u64),
    Number(f64),
    Tagged(String, Rc<VisitorResult>),
    None,
}

impl VisitorResult {
    pub fn as_string(&self) -> String {
        match self {
            VisitorResult::Compound(s) => s.iter().map(|x| x.as_string()).collect(),
            VisitorResult::String(s) => s.to_string(),
            VisitorResult::Integer(s) => s.to_string(),
            VisitorResult::Number(s) => s.to_string(),
            VisitorResult::Tagged(s, value) => format!("{s} {}", value.as_string()),
            VisitorResult::None => panic!("Value is None"),
        }
    }

    pub fn add_child(&mut self, child: VisitorResult) {
        match self {
            VisitorResult::Compound(s) => s.push(child),
            _ => panic!("Value is not compound"),
        }
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
        match self {
            VisitorResult::Compound(s) => {
                write!(f, "{label_color}{}{stop}\n", "compound")?;
                for (i, child) in s.iter().enumerate() {
                    match child {
                        VisitorResult::Tagged(_, _) => write!(f, "{space}{line_color}─{stop} ")?,
                        VisitorResult::Compound(_) => write!(f, "{space}{line_color}─{stop} ")?,
                        _ => {
                            if i < s.len() - 1 {
                                write!(f, "{space}{line_color}├─{stop} ")?;
                            } else {
                                write!(f, "{space}{line_color}└─{stop} ")?;
                            }
                        }
                    }
                    child.display(f, depth + 2)?;
                }
            }
            VisitorResult::String(s) => write!(f, "{field_color}string:{stop} {string_color}{}{stop}\n", s)?,
            VisitorResult::Integer(s) => write!(f, "{field_color}integer:{stop} {number_color}{}{stop}\n", s)?,
            VisitorResult::Number(s) => write!(f, "{field_color}number:{stop} {number_color}{}{stop}\n", s)?,
            VisitorResult::Tagged(s, value) => {
                write!(f, "{label_color}{}{stop}\n", s)?;
                write!(f, "{space}{line_color}└─{stop} ")?;
                value.display(f, depth + 2)?;
            }
            VisitorResult::None => write!(f, "{none_color}none{stop}\n")?,
        }

        Ok(())
    }
}

impl Display for VisitorResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display(f, 0)
    }
}

pub type VisitorFn<T> = Box<dyn FnMut(&mut Visitor<T>, &Node) -> VisitorResult>;

pub struct Visitor<T> {
    pub visitors: RwLock<HashMap<u32, RefCell<VisitorFn<T>>>>,
    pub scope: Arc<Mutex<T>>,
}

impl<T> Visitor<T> {
    pub fn new(scope: T) -> Self {
        Self {
            visitors: RwLock::new(HashMap::new()),
            scope: Arc::new(Mutex::new(scope)),
        }
    }

    pub fn register(&mut self, kind: u32, visitor: VisitorFn<T>) {
        self.visitors.write().unwrap().insert(kind, RefCell::new(visitor));
    }

    pub fn visit(&mut self, node: &Node) -> VisitorResult {
        let visitor = match self.visitors.read().unwrap().get(&node.kind) {
            Some(visitor) => visitor.as_ptr() as *mut VisitorFn<T>,
            None => panic!("Visitor for `{}({})` not found", node.label, node.kind),
        };
        let visitor = unsafe { &mut *visitor };
        visitor(self, node)
    }

    pub fn visit_children(&mut self, node: &Node) -> VisitorResult {
        let mut compound = VisitorResult::Compound(vec![]);
        for child in node.children.iter() {
            compound.add_child(self.visit(child));
        }
        compound
    }
}