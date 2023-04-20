use std::rc::Rc;
use std::cell::RefCell;

type Node_Ptr<T> = Rc<RefCell<ListNode<T>>>;

#[derive(Debug, Clone, PartialEq)]
struct ListNode<T> {
    val: T,
    prev: Option<Node_Ptr<T>>,
    next: Option<Node_Ptr<T>>,
}

impl<T> ListNode<T> {
    fn new(val: T) -> Self {
        Self {val: val, prev: None, next: None}
    }

    fn set_next(&mut self, node_ptr: Option<Node_Ptr<T>>) {
        self.next = node_ptr;
    }

    fn set_prev(&mut self, node_ptr: Option<Node_Ptr<T>>) {
        self.prev = node_ptr;
    }

    fn get_next(&self) -> Option<Node_Ptr<T>> {
        if let Some(ptr) = &self.next {
            return Some(ptr.clone());
        }
        None
    }
}

fn main() {
    let mut head = ListNode::new(0);
    let mut tail = ListNode::new(1);

    head.next = Some(Rc::new(RefCell::new(tail.clone())));
    tail.prev = Some(Rc::new(RefCell::new(head.clone())));

    dbg!(&head);
    dbg!(&tail);
}


