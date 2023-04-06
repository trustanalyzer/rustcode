
#[derive(Debug)]
pub struct Node {
    val: i32,
    pub next: Option<Box<Node>>,
}

fn main() {
    let mut head = Node{
        val: 5, 
        next: Some(Box::new(Node{
                val: 4,
                next: Some(Box::new(Node{
                        val: 2,
                        next: Some(Box::new(Node{
                            val: 1,
                            next: None,
                    })),
                })),
            })),
        };

    dbg!(&head);

    head.next = Some(Box::new(Node{val:-4, next:None}));
    dbg!(&head);
}
