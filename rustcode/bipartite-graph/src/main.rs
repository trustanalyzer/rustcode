use std::collections::HashMap;

fn main() {
    let graph = vec![[1,2,3].to_vec(), 
                                [0,2].to_vec(), 
                                [0,1,3].to_vec(), 
                                [0,2].to_vec(),
                                ];
    println!("{graph:?}");

    let res = is_bipartite(&graph);
    println!("{res:?}");
}

fn is_bipartite(graph: &Vec<Vec<i32>>) -> bool {

    let mut iterator = graph.into_iter();

    let mut neighbor_set_map = HashMap::new();
    for u in 0..graph.len() as i32 {
        if !neighbor_set_map.contains_key(&u) {
            neighbor_set_map.insert(u,1);
        }

        let u_set = neighbor_set_map.get(&u).unwrap();
        if let Some(neighbors) = iterator.next() {
            
            for v in neighbors {      
                if let Some(v_set) = neighbor_set_map.get(v) {
                    if v_set == u_set {
                        return false;
                    }
                }

                //neighbor_set_map.insert(*v, -1*u_set);
            }
        }

    }

    true
}
