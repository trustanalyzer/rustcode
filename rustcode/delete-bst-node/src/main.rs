
#[derive(Debug)]
struct Node {
    val: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(val: u32) -> Self {
        Self {
            val: val,
            left: None,
            right: None,
        }
    }

    fn push(&mut self, val: u32) -> () {
        
        if val <= self.val {
            match self.left {
                None => {
                    self.left = Some(Box::new(Node::new(val)));
                },
                _ => {
                    self.left.as_mut().unwrap().push(val);
                }
            }
        } else {
            match self.right {
                None => {
                    self.right = Some(Box::new(Node::new(val)));
                },
                _ => {
                    self.right.as_mut().unwrap().push(val);
                }
            }
        }
    }

    fn delete(&mut self, val: u32) -> () {
        
        if val == self.val {
            match (&self.left, &self.right) {
                (None, None) => {
                    dbg!(val, self.val);
                },
                (None, Some(_)) => {
                    let new_val = self.right.as_ref().unwrap().val;
                    self.val = new_val;
                    self.right.as_mut().unwrap().delete(new_val);
                },
                (Some(_), None) => {
                    let new_val = self.left.as_ref().unwrap().val;
                    self.val = new_val;
                    self.left.as_mut().unwrap().delete(new_val);
                },
                (_, _) => {
                    let mut curr = self.right.as_mut().unwrap();
                    while curr.left.is_some() {
                        curr = curr.left.as_mut().unwrap();
                    }
                    self.val = curr.val;
                    curr.delete(curr.val);
                }
            }

        } else if val < self.val {
            //dbg!(val, self.val);
            self.left.as_mut().unwrap().delete(val);
        } else {
            //dbg!(val, self.val);
            self.right.as_mut().unwrap().delete(val);
        }



    }
}


fn main() {
    
    let mut root = Node::new(5);
    root.push(2);
    root.push(4);
    root.push(3);
    root.push(1);
    root.push(8);
    root.push(6);
    root.push(7);

    //dbg!(&root);
    root.delete(8);
    dbg!(&root);
}


