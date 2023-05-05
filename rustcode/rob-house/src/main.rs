use std::cmp::max;


#[derive(Debug, Clone)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}



fn main() {
    let root = Some(Box::new(TreeNode {
        val: 3,
        left: Some(Box::new(TreeNode {
            val: 20, 
            left: Some(Box::new(TreeNode {val: 100, left: None, right: None})),
            right: None,
        })),
        right: Some(Box::new(TreeNode { 
            val: 5, 
            left: Some(Box::new(TreeNode {val: 1, left: None, right: None})), 
            right: None, 
        }))

    }));

    let res = dfs(root);
    dbg!(res);
}

fn dfs(root: Option<Box<TreeNode>>) -> (i32, i32) {
    if root.is_none() {return (0,0);}

    let left_tup = match root {
        None => (0,0),
        Some(ref ptr) => dfs(ptr.left.clone()),
    };
    let right_tup = match root {
        None => (0,0),
        Some(ref ptr) => dfs(ptr.right.clone()),
    };

    let mut with_root = 0;
    let mut without_root = 0;
    if let Some(ptr) = root {
        with_root = ptr.val + left_tup.1 + right_tup.1;
        without_root = max(left_tup.0, left_tup.1) + max(right_tup.0, right_tup.1);
    }

    (with_root, without_root)

}
 