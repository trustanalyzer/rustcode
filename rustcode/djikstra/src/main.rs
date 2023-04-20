fn main() {
    let time = [[2,1,1], [2,3,1], [3,4,1], [4,1,5]];

    use std::collections::{HashSet, BinaryHeap, HashMap};
    use std::cmp::{max, Reverse};

    let mut adj_list = HashMap::new();
    for [u,v,w] in time {
        adj_list
        .entry(u)
        .and_modify(|vec: &mut Vec<[i32; 2]>| {vec.push([v,w])})
        .or_insert(vec![[v,w]]);
    }

    dbg!(&adj_list);
    let n = 4;
    let k = 2;

    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0usize,k)));
    
    let mut visited = HashSet::new();
    let mut res = 0;
    while queue.len() > 0 && visited.len() < n {
        
        dbg!(&queue);
        match queue.pop() {
            Some(Reverse((dist, node))) => {
                //dbg!(&node);
                if let Some(_) = &adj_list.get(&node) {
                    for [v,w] in &adj_list[&node]{
                        if !visited.contains(v) {
                            queue.push(Reverse((*w as usize+dist, *v)));
                        }
                    }
                }
                res = max(res, dist);
                visited.insert(node);
        }
            _ => {},
        }
    }

    dbg!(res);

}
