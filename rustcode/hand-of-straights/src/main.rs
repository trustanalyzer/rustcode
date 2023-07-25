use std::collections::{HashMap, BinaryHeap};

fn main() {
    let hand = vec![1,2,3,6,2,3,4,7,8];

    assert!(is_divisible(&hand, 3));

    let hand_2 = vec![1,2,3,6,2,3,4,7,9];

    assert_ne!(is_divisible(&hand_2, 3), true);

    let hand_3 = vec![1,1,3,6,2,3,4,7,8];

    assert_eq!(is_divisible(&hand_3, 3), false);
}

fn is_divisible(hand: &Vec<i32>, group_size: usize) -> bool {

    if hand.len() % group_size > 0 {return false;}

    let mut counter = HashMap::new();
    for key in hand {
        counter.entry(key)
        .and_modify(|ct| *ct+=1)
        .or_insert(1);
    }

    let mut max_heap = BinaryHeap::new();
    for (k,v) in counter.into_iter() {
        max_heap.push((*k,v));
    }

    while let Some((mut key, count)) = max_heap.pop() {

        //dbg!(key, count);
        let mut remaining = Vec::new();
        for _ in 1..group_size {
            
            //dbg!(&max_heap);
            match max_heap.pop() {
                Some((next_key, next_ct)) if next_key == key - 1 && next_ct >= count => {
                    key = next_key;
                    if next_ct > count {
                        remaining.push((next_key, next_ct - count))
                    }
                }
                _ => {return false;}
            }
        }

        for (k,c) in remaining {
            max_heap.push((k,c))
        }
    }

    true
}