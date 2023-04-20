
#[derive(Debug)]
pub struct TreeNode {
    val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn main() {

    let root = TreeNode {
        val: 1, 
        left: Some(Box::new(
            TreeNode {
                val: 2,
                left: Some(Box::new(
                    TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }
                )),
                right: Some(Box::new(
                    TreeNode {
                        val: 5,
                        left: None,
                        right: None,
                    }
                )),
            }
        )),
        right: Some(Box::new(
            TreeNode {
                val: 3,
                left: Some(Box::new(
                    TreeNode {
                        val: 6,
                        left: None,
                        right: None,
                    }
                )),
                right: Some(Box::new(
                    TreeNode {
                        val: 7,
                        left: None,
                        right: None,
                    }
                )),
            }
        )),
    };
    
    dbg!("{}",is_complete(Some(root)));
    // dbg!(root);
}

fn is_complete(root: Option<TreeNode>) -> bool {
    use std::collections::VecDeque;

    if let None = root {return true;}

    let mut q = VecDeque::new();
    q.push_back(root);

    while q.len() > 0 {
        if let Some(node) = q.pop_front().unwrap() {
            match node.left {
                Some(left) => q.push_back(Some(*left)),
                _ => q.push_back(None),
            };
            match node.right {
                Some(right) => q.push_back(Some(*right)),
                _ => q.push_back(None),
            }
        } else {
            // dbg!(&q);
            for val in &q {
                if val.is_some() {return false;}
            }
        }
        // dbg!(&q);
    }

    true
}
