#[derive(Debug, Clone)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>
}

impl TreeNode {

    fn insert(&mut self, val: i32, on_left: bool) -> () {
        if on_left {
            self.left = Some(Box::new(TreeNode {
                val: val, left: None, right: None,
            }));
        } else {
            self.right = Some(Box::new(TreeNode {
                val: val, left: None, right: None
            }));
        }
    } 
}

#[derive(Debug)]
struct BSTree {
    root: Option<Box<TreeNode>>
}

impl BSTree {
    fn new() -> Self {
        Self {root: None}
    }

    fn insert(&mut self, key: i32) -> () {
        if self.root.is_none() {
            self.root = Some(Box::new(TreeNode {val:key, left: None, right: None}));
            return ();
        } 

        let mut curr = self.root.as_mut();
        while let Some(node) = curr {

            dbg!(&node);
            
            if key <= node.val {
                match &node.left {
                    None => {
                        node.insert(key, true);
                        return ();
                    },
                    Some(_) => {curr = node.left.as_mut();}
                }

            } else {
                match node.right {
                    None => {
                        node.insert(key, false);
                        return ();
                    },
                    Some(_) => {curr = node.right.as_mut();}
                }
            }
        }
    }
}

fn main() {
    let list = [4,7,2,5,1,3];
    let mut bstree = BSTree::new();
    for key in list {
        bstree.insert(key);
    }

    dbg!(bstree);
}
