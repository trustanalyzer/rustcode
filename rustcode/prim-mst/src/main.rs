use std::{collections::{HashMap, HashSet, BinaryHeap}, cmp::Reverse};

fn main() {
    let points = vec![[0,0],[2,2],[3,10],[5,2],[7,0]];
    let n = points.len();

    let mut adj_list = HashMap::new();
    for i in 0..n {

        let p = &points[i];
        for j in i+1..n {
            let q = &points[j];
            let cost = (p[0] as i32 - q[0]).abs() + (p[1] - q[1]).abs();

            adj_list.entry(i)
            .and_modify(|v: &mut Vec<(i32, usize)>| v.push((cost,j)))
            .or_insert(vec![(cost,j)]);

            adj_list.entry(j)
            .and_modify(|v: &mut Vec<(i32, usize)>| v.push((cost,i)))
            .or_insert(vec![(cost,i)]);
        }
    }

    let res = mst(adj_list, n);
    dbg!(res);

}

fn mst(adj_list: HashMap<usize, Vec<(i32, usize)>>, n: usize) -> i32 {

    let mut visited = HashSet::new();
    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse((0,0)));

    let mut cost = 0;
    while visited.len() < n {

        if let Some(Reverse((w,u))) = min_heap.pop() {
            
            if !visited.contains(&u) {

                cost += w;

                for (w, v) in &adj_list[&u] {
                    if !visited.contains(v) {
                        min_heap.push(Reverse((*w, *v)));
                    }

                }

                visited.insert(u);
            }
        }

    }

    cost

}
