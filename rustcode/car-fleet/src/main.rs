use std::collections::BinaryHeap;

fn main() {

    let target = 12;
    let position = vec![10,8,0,5,3];
    let speed = vec![2,4,1,1,3];

    let mut max_heap = BinaryHeap::new();
    for (i, p) in position.iter().enumerate() {
        max_heap.push((p, i));
    }

    let mut stack = Vec::new();
    while let Some((p, i)) = max_heap.pop() {
        if let Some(s) = speed.iter().nth(i) {
            
            let time = (target as f64 - *p as f64) / *s as f64;
            println!("{time:?}");
            if stack.is_empty() {
                stack.push(time);
            } else if time > *stack.last().unwrap() {
                stack.push(time);
            }
        }
        
    }

    println!("{stack:?}");

}
