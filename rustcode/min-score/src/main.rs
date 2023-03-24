use std::collections::{HashSet, HashMap};
use std::cmp::min;

fn main() {
    let roads: Vec<Box<[u32; 3]>> = vec![
        Box::new([1,2,9]),
        Box::new([2,3,6]),
        Box::new([2,4,5]),
        Box::new([1,4,7]),
    ];
    let n: u32 = 4;

    let mut adj_list = create_adj_list(roads);

    // println!("{:?}", adj_list);

    // let mut res: u32 = 10;
    dfs(n, &mut adj_list);

    //println!("{:?}", res);
}

fn create_adj_list(roads: Vec<Box<[u32; 3]>>) -> HashMap<u32, Vec<(u32,u32)>> {
    
    let mut adj_list = HashMap::new();
    for road in roads.into_iter() {
        // println!("{:?}", road[0]);
        adj_list
        .entry(road[0])
        .and_modify(|v: &mut Vec<(u32,u32)>| {
            v.push((road[1],road[2]));
        })
        .or_insert(vec![(road[1], road[2])]);

        adj_list
        .entry(road[1])
        .and_modify(|v: &mut Vec<(u32,u32)>| {
            v.push((road[0],road[2]));
        })
        .or_insert(vec![(road[0], road[2])]);
    };

    adj_list
}

fn dfs(n:u32, adj_list:&mut HashMap<u32, Vec<(u32, u32)>>) -> () {
    
    let mut stack: Vec<u32> = vec![];
    let mut visited : HashSet<u32>= HashSet::new();
    let mut res: u32 = u32::MAX as u32;
    let mut curr: u32 = 1;
    
    while visited.len() < n as usize || stack.len() > 0 {
        // println!("{:?}", curr);
        if let Some(mut v) = adj_list.remove(&curr) {
            while let Some(tup) = v.pop() {
                res = min(res, tup.1);
                stack.push(tup.0);
                // println!("{:?}", tup);
            };
            stack.push(curr);
            // println!("{:?}", stack);
        } else {
            visited.insert(curr);
            if let Some(val) = stack.pop() {
                curr = val;
            };
        };
    }

    println!("{:?}", res);
}
