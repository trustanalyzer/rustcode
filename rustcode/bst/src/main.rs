
use std::cmp::{PartialEq, PartialOrd};

#[derive(Debug)]
struct TreeNode<T> {
    data: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: PartialOrd + PartialEq> TreeNode<T> {
    fn new(val: T) -> TreeNode<T> {
        TreeNode { data: val, left: None, right: None }
    }

    fn add(&mut self, val: T) {
        if self.data == val {return};
        let update = if val > self.data {&mut self.right}
                        else {&mut self.left};
        match update {
            Some(update) => update.add(val),
            None => *update = Some(Box::new(TreeNode::new(val))),
        }
    }

    fn search(&self, val: T) -> Option<T> {
        if self.data == val {Some(val)}
        else if val < self.data {self.left.as_ref()?.search(val)}
        else if val > self.data {self.right.as_ref()?.search(val)}
        else {None}
    }

    fn insert(&mut self, val: T) {
        if self.data == val {return};
        let update = if self.data < val {&mut self.right} else {&mut self.left};

        match update {
            Some(node) => node.insert(val),
            None => *update = Some(Box::new(TreeNode::new(val))),
        }
    }
}

fn main() {
    let mut root = TreeNode{ data: 8, left: None, right: None };
    root.insert(4);
    root.insert(20);
    root.add(5);
    root.add(22);
    root.add(2);

    println!("{:?}", root.search(22));

}