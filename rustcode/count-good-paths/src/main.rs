use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug)]
struct UF {
    parent: HashMap<i32, i32>,
    rank: HashMap<i32, i32>,
}

impl UF {
    fn new(n: i32) -> UF {
        let mut parent = HashMap::new();
        let mut rank = HashMap::new();
        for i in 0..n {
            parent.insert(i,i);
            rank.insert(i,1);
        }

        UF {parent: parent, rank: rank}
    }

    fn find(&mut self, mut i: i32) -> i32 {

        while i != *self.parent.get(&i).unwrap() {
            let parent = *self.parent.get(&i).unwrap();
            self.parent.insert(parent, *self.parent.get(&parent).unwrap());
            i = parent;
        }

        i

    }

    fn union(&mut self, u: i32, v: i32) -> bool {
        
        let u_root = self.find(u);
        let v_root = self.find(v);

        if u_root == v_root {return false;}

        let u_rank = *self.rank.get(&u_root).unwrap();
        let v_rank = *self.rank.get(&u_root).unwrap();

        if u_rank > v_rank {
            self.parent.insert(v, u);
            *self.rank.get_mut(&u).unwrap() += v_rank;

        } else {

            self.parent.insert(u, v);
            *self.rank.get_mut(&v).unwrap() += u_rank;

        }

        true
    }
}

fn main() {
    let n = 5;
    let mut uf = UF::new(n);

    //println!("{uf:?}");

    let vals = vec![1,3,2,1,3];
    let mut val_to_indexes = HashMap::new();
    for (i, val) in vals.iter().enumerate() {
        val_to_indexes.entry(val)
        .and_modify(|vect: &mut Vec<usize>| vect.push(i))
        .or_insert(vec![i]);
    }

    let sorted_keys = val_to_indexes.keys().sorted();
    // println!("{sorted_keys:?}");
    
    let edges = vec![(0,1), (0,2), (2,3), (2,4)];
    let mut adj_list = HashMap::new();
    for (u,v) in edges {
        adj_list.entry(u)
        .and_modify(|vect: &mut Vec<i32>| vect.push(v))
        .or_insert(vec![v]);

        adj_list.entry(v)
        .and_modify(|vect: &mut Vec<i32>| vect.push(u))
        .or_insert(vec![u]);
    }

    // println!("{adj_list:?}");

    let mut paths = 0;
    for key in sorted_keys {
        for idx in val_to_indexes.get(key).unwrap() {
            for nei in adj_list.get(&(*idx as i32)).unwrap() {
                if vals[*nei as usize] <= vals[*idx] {
                    uf.union(*idx as i32, *nei);
                }
            }
        }
    }

    println!("{uf:#?}");
}


