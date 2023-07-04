
#[derive(Debug, Default)]
struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Option<Box<Node>> {
        Some(Box::new(Node {val: val, left: None, right: None}))
    }

    fn insert(&mut self, val: i32) -> () {
        
        match val {
            val if val < self.val => {
                match self.left {
                    None => {self.left = Some(Box::new(Node {val: val, left: None, right: None}));},
                    _ => {self.left.as_mut().unwrap().insert(val);}
                }
            }
            _ => {
                match self.right {
                    None => {self.right = Some(Box::new(Node {val: val, left: None, right: None}));},
                    _ => {self.right.as_mut().unwrap().insert(val);}
                }
            }
        }   
    }
}

fn main() {
    let mut root = Node::new(4).unwrap();
    for num in [2,1,3,6] {
        root.insert(num);
    }
    println!("{root:#?}");
}
