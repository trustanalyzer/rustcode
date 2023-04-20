use std::collections::HashMap;

fn main() {
    let edges = vec![[1,2], [1,3], [2,3]];
    let (mut parent, mut rank) = (HashMap::new(), HashMap::new());
    for i in 1..=edges.len() {
        parent.insert(i as i32,i as i32);
        rank.insert(i as i32,1);
    }

    for [u,v] in edges.into_iter() {
        dbg!(&parent);
        dbg!(&rank);

        if !union(u,v, &mut parent, &mut rank) {
            dbg!(u, v);
        }
    }

}

fn find(u: i32, 
    parent: &mut HashMap<i32, i32>, 
    rank: &mut HashMap<i32, i32>) -> i32 {

    let mut p = u;
    while p != *parent.get(&p).unwrap() {
        // parent.get_mut(&p).map(|v| *v = *parent.get(parent.get(&p).unwrap()).unwrap());
        p = *parent.get(&p).unwrap();
    }

    p
}

fn union(u: i32, v: i32, 
    parent: &mut HashMap<i32, i32>, 
    rank: &mut HashMap<i32, i32>) -> bool {
    let (u_root, v_root) = (find(u, parent, rank), find(v, parent, rank));
    
    if u_root == v_root {return false;}

    let (u_rank, v_rank) = (*rank.get(&u_root).unwrap(), *rank.get(&v_root).unwrap());
    
    if u_rank > v_rank {

        parent.get_mut(&v_root).map(|v| *v = u_root);
        rank.get_mut(&u_root).map(|v| *v += v_rank);

    } else {

        parent.get_mut(&u_root).map(|v| *v = v_root);
        rank.get_mut(&v_root).map(|v| *v += u_rank);

    }

    true
}
