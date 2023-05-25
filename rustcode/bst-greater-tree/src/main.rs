
#[derive(Debug, Clone)]
struct TreeNode {
    val: u32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: u32) -> Self {
        Self {val, left: None, right: None}
    }
}

#[derive(Debug, Clone)]
struct BST {
    root: Option<Box<TreeNode>>,
}

impl BST {
    fn new(val: u32) -> Self {
        Self {
            root: Some(Box::new(TreeNode::new(val)))
        }
    }

    fn insert(&mut self, val: u32) -> () {
        
        dbg!(&self.root, val);
        let mut prev = None;
        let mut curr = self.root;
        while let Some(node_ptr) = curr.clone() {

            prev = curr;
            if node_ptr.val < val {
                curr = node_ptr.right;
            } else {
                curr = node_ptr.left;
            }
        
        }
        
        if let Some(mut node_ptr2) = prev {

            if node_ptr2.val < val {
                node_ptr2.right = Some(Box::new(TreeNode::new(val)));
            } else {
                node_ptr2.left = Some(Box::new(TreeNode::new(val)));
            }
        }
    }
}

fn main() {

    let mut bst = BST::new(4);
    bst.insert(1);
    bst.insert(6);
    bst.insert(0);
    bst.insert(2);
    bst.insert(3);
    bst.insert(5);
    bst.insert(7);
    bst.insert(8);

    println!("{:#?}", bst);
    
}

