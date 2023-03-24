fn main() {
    let v = vec!['A', 'B', 'A', 'A', 'B', 'C', 'C'];

    let ans = least_processing_time(&v, 3);
    println!("least time is {:?}", ans);
}

use std::collections::{VecDeque, HashMap, BinaryHeap};

fn least_processing_time(tasks: &Vec<char>, n: usize) -> usize {
    
    let mut counter:HashMap<&char, usize> = HashMap::new();
    for c in tasks {
        counter
        .entry(c)
        .and_modify(|c| {*c += 1})
        .or_insert(1);
    }

    // println!("{:?}", counter.values());
    let mut max_heap = BinaryHeap::new();

    for val in counter.values() {
        max_heap.push(*val);
    };

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    let mut time = 0;
    while !max_heap.is_empty() || !queue.is_empty() {
        time += 1;
        // println!("{:?}", max_heap);

        if !max_heap.is_empty() {
            let count = max_heap.pop().unwrap() - 1;

            if count != 0 {
                queue.push_back((count, time + n));
            }
        }

        if let Some((count, start_time)) = queue.pop_front() {
            if start_time == time {
                max_heap.push(count);
                //println!("{:?}", count);
            } else {
                queue.push_front((count, start_time));
            }
        }
        // println!("{:?}", queue);
    }

    time

}
