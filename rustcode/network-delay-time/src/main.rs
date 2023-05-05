use std::{collections::{HashMap, BinaryHeap, HashSet}, cmp::{max, Reverse}};

fn main() {
    let input = vec![[2u32,1,1], [2,3,1], [3,4,1]];
    let n = 4u32;
    let k = 2u32;

    let max_dist = farthest_node(&input, n, k);
    dbg!(max_dist);
}

fn farthest_node(input: &Vec<[u32;3]>, n: u32, k: u32) -> u32 {
    
    let mut adj_list = HashMap::new();
    for [u,v,w] in input.iter() {
        adj_list.entry(*u)
        .and_modify(|vec: &mut Vec<[u32;2]>| vec.push([*v,*w]))
        .or_insert(vec![[*v,*w]]);
    }

    dbg!(&adj_list);

    let mut max_dist = 0;
    let mut min_heap= BinaryHeap::new();
    min_heap.push(Reverse([0u32,k]));
    
    let mut visited = HashSet::new();

    while !min_heap.is_empty() && visited.len() < n as usize {

        if let Some(Reverse([dist, node])) = min_heap.pop() {
            
            dbg!(dist,node);
            if !visited.contains(&node) {
                
                max_dist = max(max_dist, dist);
                if let Some(vect) = adj_list.get(&node){
                    
                    
                    for [v,w] in vect.into_iter() {
                        if !visited.contains(v) {
                            min_heap.push(Reverse([w+dist, *v]));
                        }
                    }
                }
                visited.insert(node);
            }
        }

    }

    if visited.len() == n as usize {return max_dist;}
    else {u32::MAX}
}
