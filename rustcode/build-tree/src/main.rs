#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}



fn main() {
    let preorder = vec![3,9,20,15,7];
    let inorder = vec![9,3,15,20,7];

    let root = build_tree(preorder, inorder);
    println!("{:?}", root);

    //let mut root = TreeNode{val:3,left:None,right:None};
    //root.left = Some(&TreeNode{val:9,left:None,right:None});
    //root.right = Some(&TreeNode{val:20,left:None,right:None});

    //println!("{:?}", root.left);
}

fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Box<TreeNode>> {
        if preorder.len() == 0 || inorder.len() == 0 {
            return None;
        }

        let mut node = Box::new(TreeNode{val: preorder[0], left: None, right: None});
        let index = &inorder.iter().position(|e| e == &preorder[0]).unwrap();
        //dbg!(&node, index);
        node.left = build_tree(preorder[1..*index+1].to_owned(), inorder[0..*index].to_owned());
        node.right = build_tree(preorder[*index+1..].to_owned(), inorder[*index+1..].to_owned());

        //dbg!(&node, index);
        //println!("{:?}", node);
        Some(node)
    }



