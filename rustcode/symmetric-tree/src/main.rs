#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn main() {
    let root = TreeNode {val: 1, 
        left: Some(Box::new(
            TreeNode {
                val: 2,
                left: Some(Box::new(
                    TreeNode {
                        val: 3,
                        left: None,
                        right: None,
                    }
                )),
                right: Some(Box::new(
                    TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }
                )),
            }
        )),
        right: Some(Box::new(
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
                        val: 3,
                        left: None,
                        right: None,
                    }
                )),
            }
        )),
    };

    // dbg!(&root);
    let res = is_symmetric(root.left, root.right);
    dbg!(res);
}

fn is_symmetric(left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> bool {
    if left.is_none() && right.is_none() {return true;}
    if left.is_none() ^ right.is_none() {return false;}

    let left = left.unwrap();
    let right = right.unwrap();

    // dbg!(&left, &right);

    let res = left.val == right.val 
    && is_symmetric(left.left, right.right) 
    && is_symmetric(left.right, right.left);
    // dbg!(&res);
    res
}
