use std::collections::{HashMap, BinaryHeap};

fn main() {
    let s = String::from("aaabbccf");
    let res = reorganise_string(s);
    println!("{res:?}");
}

fn reorganise_string(s: String) -> String {

    let mut counter = HashMap::new();
    for c in s.chars() {
        counter.entry(c)
        .and_modify(|ct| *ct+=1)
        .or_insert(1);
    }

    let mut max_heap = BinaryHeap::new();
    for (_,v) in counter.into_iter().enumerate() {
        max_heap.push((v.1,v.0));
    }

    let mut result = String::new();
    let mut unavailable = None;
    
    while let Some((num, c)) = max_heap.pop() {
        
        dbg!(&max_heap, &unavailable);
        
        if let Some((ct, chr)) = unavailable {
            max_heap.push((ct, chr));
            unavailable = None;
        }

        result.push(c);
        if num > 1 {
            unavailable = Some((num-1, c));
        }
    }

    if max_heap.is_empty() && unavailable.is_some() {
        return String::new();
    }

    result

}
