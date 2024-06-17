use std::borrow::{Borrow, BorrowMut};

use ars::{add_node, ast::{Node, Value}, node};

fn main() {
    let mut root = node!(0, "root", Value::None);
    add_node!(root, node!(1, "a", Value::None), node!(2, "b", Value::None));
    println!("{:#?}", root);
}