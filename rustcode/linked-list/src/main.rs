use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn new(data: i32) -> List {
        Cons(data, Box::new(Nil))
    }

    fn add(data:i32, head: List) -> List {
        match head {
            Nil => {Self::new(data)},
            _ => {Cons(data, Box::new(head))},
        }
    }
}

fn main() {
    let list = List::new(1);
    println!("{:?}", list);

    let head = Nil;
    let list = List::add(22, head);
    println!("{:?}", &list);
}