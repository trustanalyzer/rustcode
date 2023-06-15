
#[derive(Debug)]
struct Node {
    val: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn add(node_opt: &mut Option<Box<Node>>, val: u32) -> () {
    
    if node_opt.is_some() {
        
        let root_val = node_opt.as_ref().unwrap().val;
        if val == root_val * 2 {
            node_opt.as_mut().unwrap().left = 
            Some(Box::new(Node{val:val, left: None, right: None}))
        } else {
            node_opt.as_mut().unwrap().right = 
            Some(Box::new(Node{val:val, left: None, right: None}))
        }
    }
}

fn main() {
    
    let mut root = Some(Box::new(Node{val: 1, left: None, right: None}));
    add(&mut root, 2);
    add(&mut root, 3);
    for num in 4..=5 {
        add(&mut root.as_mut().unwrap().left, num);
        add(&mut root.as_mut().unwrap().right, num+2);
    }

    println!("{root:#?}");
}
