use std::collections::{HashMap, HashSet, VecDeque};

fn main() {

    let connections = vec![[0,1], [1,3], [2,3], [4, 0], [4, 5]];
    let mut adj_list = HashMap::new();

    for [u, v] in connections {
        adj_list.entry(u).and_modify(|vtr: &mut Vec<i32>| vtr.push(v))
        .or_insert(vec![v]);

        adj_list.entry(v).and_modify(|vtr: &mut Vec<i32>| vtr.push(-u))
        .or_insert(vec![-u]);
    }

    let res = bfs(adj_list);
    println!("{res:?}");

}

fn bfs(adj_list: HashMap<i32, Vec<i32>>) -> i32 {

    //dbg!(&adj_list);
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(0);

    let mut changes = 0;

    while !queue.is_empty() {

        //dbg!(&queue, &visited);

        if let Some(mut u) = queue.pop_front() {

            if u > 0 {
                changes += 1;
            } else {
                u *= -1;
            }


            if let Some(vtr) = adj_list.get(&u) {
                for v in vtr {
                    if !visited.contains(&v.abs()) {
                        queue.push_back(*v);
                    }
                }
            }

            visited.insert(u);

        }

    }

    changes

}