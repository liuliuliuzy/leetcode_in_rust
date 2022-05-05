#[allow(unused_imports)]
use std::{borrow::Borrow, cell::RefCell, rc::Rc};

fn main() {
    let x = Rc::new(8);
    println!("{}", x);

    println!("Enter a number to generate the coding files");
}
