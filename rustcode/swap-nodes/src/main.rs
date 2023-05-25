
#[derive(Debug, Clone)]
struct Node {
    val: i32,
    next: Option<Box<Node>>
}

fn main() {
    let head = Some(Box::new(Node{
        val: 1,
        next: Some(Box::new(Node {
            val: 2,
            next: Some(Box::new(Node{
                val: 3,
                next: Some(Box::new(Node { 
                    val: 4, 
                    next: Some(Box::new(Node { val: 5, next: None })) }))
            }))
        })),
    }));

    swap_nodes(head, 2);
}

fn swap_nodes(head: Option<Box<Node>>, k: usize) -> () {
    
    let mut curr = head.clone();
    for _ in 0..k-1 {
        //println!("{:?}", curr.clone());
        curr = curr.unwrap().next;
    }

    dbg!(&curr);
    
    let mut right = head;
    while let Some(_) = curr.clone() {
        curr = curr.unwrap().next;
        right = right.unwrap().next;
    }

    dbg!(right);


}
