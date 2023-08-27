use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

use crate::List::{Cons, Nil};

fn main() {
    // Reference Counting and Lisp-style lists
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    println!("a: {:?}, b: {:?}, c: {:?}", a, b, c);

    // Custom Smart Pointers implementing Drop manually
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other_stuff"),
    };

    println!("CustomSmartPointers created");

    // Strong and Weak references - Tree Data Structure

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {} weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    
    {

        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("branch strong = {} weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
        println!("leaf strong = {} weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!("leaf strong = {} weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
}
