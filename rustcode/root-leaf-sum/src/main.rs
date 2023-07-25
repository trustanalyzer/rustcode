use std::collections::VecDeque;

#[derive(Debug,Clone)]
struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}


fn main() {
    let root = Some(Box::new(
        Node {
            val: 1, 
            left: Some(Box::new(Node {val: 2, left: None, right: None})),
            right: Some(Box::new(Node {val: 3, left: None, right: None}))
        }
    ));

    let res = root_leaf_sum(root);
    println!("{res:?}");
}

fn root_leaf_sum(root: Option<Box<Node>>) -> Option<i32> {
    if root.is_none() {return None;}

    let mut res = 0;
    let mut queue: VecDeque<(Node, i32)> = VecDeque::new();
    
    queue.push_back((*root.unwrap(), 0));
    while let Some((node, mut curr_sum)) = queue.pop_front() {

        dbg!(&node, &curr_sum);

        curr_sum += node.val;
        if node.left.is_none() && node.right.is_none() {
            res += curr_sum;
        };
        if node.left.is_some() {
            queue.push_back((*node.left.unwrap(), curr_sum*10))
        };
        if node.right.is_some() {
            queue.push_back((*node.right.unwrap(), curr_sum*10))
        };
    }

    Some(res)

}


