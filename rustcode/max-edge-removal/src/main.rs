
use std::collections::{BinaryHeap,HashMap};

#[derive(Debug, Clone)]
struct ConnectedComponents {
    V: u32,
    parent: HashMap<u32,u32>,
    rank: HashMap<u32, u32>,
}

impl ConnectedComponents {
    fn new(vertices: u32) -> Self {

        let mut parent = HashMap::new();
        let mut rank = HashMap::new();
        for v in 1..vertices+1 {
            parent.insert(v,v);
            rank.insert(v,1);
        }

        Self {
            V: vertices,
            parent: parent,
            rank: rank,
        }

    }

    fn find(&mut self, mut u: u32) -> u32 {
        
        //dbg!(u);
        while u != *self.parent.get(&u).unwrap() {
            
            let par = *self.parent.get(&u).unwrap();
            let grand_par = *self.parent.get(&par).unwrap();
            self.parent.entry(u).and_modify(|p| *p = grand_par);
            u = grand_par;
        }

        u   
    }

    fn union(&mut self, u: u32, v: u32) -> bool {
        
        let u_root = self.find(u);
        let v_root = self.find(v);

        if u_root == v_root {return false;}
        let u_rank = *self.rank.get(&v_root).unwrap();
        let v_rank = *self.rank.get(&v_root).unwrap();

        if u_rank >= v_rank {
            self.parent.entry(v_root).and_modify(|p| *p = u_root);
            self.rank.entry(u_root).and_modify(|r| *r += v_rank);
        } else {
            self.parent.entry(u_root).and_modify(|p| *p = v_root);
            self.rank.entry(v_root).and_modify(|r| *r += u_rank);
        }

        true
    }
}


fn main() {
    let edges = vec![[1u32, 1, 2], [1, 2, 4], [1, 1, 3], 
        [2, 3, 4],[3, 1, 2], [3, 2, 4]];

    let mut max_heap: BinaryHeap<[u32;3]> = edges.into();
    //dbg!(max_heap);

    let vertices = 4u32;

    let mut cc_a = ConnectedComponents::new(vertices);
    let mut cc_b = cc_a.clone();
    let (mut edge_ct_a, mut edge_ct_b) = (0u32, 0u32);
    
    while let Some([t, u, v]) = max_heap.pop() {
        if t == 3 {
            edge_ct_a += cc_a.union(u,v) as u32;
            edge_ct_b = edge_ct_a;
            cc_b = cc_a.clone();

        } else if t == 1 {
            edge_ct_a += cc_a.union(u,v) as u32;

        } else {
            edge_ct_b += cc_b.union(u,v) as u32;
            
        }

        if edge_ct_a == cc_a.V-1 && edge_ct_b == cc_b.V-1 {break;}
    }

    println!("{edge_ct_a:?}");
}


