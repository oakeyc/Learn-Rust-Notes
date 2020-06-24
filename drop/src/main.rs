struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("dropping {}", self.data);
    }
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn main() {
    let c = CustomSmartPointer {
        data: String::from("foo bar"),
    };

    let d = CustomSmartPointer {
        data: String::from("other baz"),
    };
    println!("Created pointers");
    drop(c);

    println!("dropped c on the ground :(");

    //    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    //    println!("count after a = {}", Rc::strong_count(&a));
    //    let b = Cons(3, Rc::clone(&a));
    //    println!("count after b = {}", Rc::strong_count(&a));
    //    {
    //        let c = Cons(4, Rc::clone(&a));
    //        println!("count after c = {}", Rc::strong_count(&a));
    //    }
    //    println!("count after c is out of scope = {}", Rc::strong_count(&a));

    let x = 10;
    //    let y &mut x;

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
