use std::collections::{HashMap, HashSet, BinaryHeap};


fn main() {
    let n = 7;
    let edges = [[0,1], [0,2], [1,4], [1,5], [2,3], [2,6]].to_vec();
    let has_apple = [false, false, true, false, true, true, false].to_vec();

    let mut adj_list = HashMap::new();
    for [u,v] in edges {
        let dist = -2*(has_apple[v as usize] as i32);
        adj_list.entry(u).and_modify(|vtr: &mut Vec<(i32, i32)>| vtr.push((v, dist)))
        .or_insert(vec![(v, dist)]);
    };
    //println!("{adj_list:?}");

    let mut visited = HashSet::new();
    visited.insert(0);

    let mut max_heap = BinaryHeap::new(); 
    if let Some(vtr) = adj_list.get(&0) {
        for (v, d) in vtr {
            if !visited.contains(&v) {
                max_heap.push((d,v));
            }
        }
    };

    let mut time = 0;
    while let Some((d, u)) = max_heap.pop() {
        time += *d;
        visited.insert(*u);

        if let Some(vtr) = adj_list.get(u) {
            for (v, dist) in vtr {
                if !visited.contains(&v) {
                    max_heap.push((dist,v));
                }
            }
        };

        if visited.len() == n {
            println!("{:?}", time.abs());
            break;
        }
    }
}
