use std::collections::{HashMap, VecDeque, HashSet};

fn main() {
    let n = 4;
    let redEdges = [[0,1], [0,2], [2,3], [1,3]].to_vec();
    let blueEdges = [[1,2]].to_vec();

    let res = shortest_path(&redEdges, &blueEdges);
    println!("{res:?}");
}

fn shortest_path(reds: &Vec<[i32;2]>, blues: &Vec<[i32;2]>) -> HashMap<i32, i32> {

    let mut red_adj_list: HashMap<i32, Vec<i32>> = HashMap::new();
    for [src, dst] in reds {
        red_adj_list.entry(*src)
        .and_modify(|vtr| vtr.push(*dst))
        .or_insert(vec![*dst]);
    }

    let mut blue_adj_list: HashMap<i32, Vec<i32>> = HashMap::new();
    for [src, dst] in blues {
        blue_adj_list.entry(*src)
        .and_modify(|vtr| vtr.push(*dst))
        .or_insert(vec![*dst]);
    }

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((0,0,true)); // is_red bool = true
    queue.push_back((0,0,false));

    let mut res = HashMap::new();
    while let Some((node, dist, in_color)) = queue.pop_front() {

        visited.insert((node, in_color));
        if let None = res.get(&node) {
            res.insert(node, dist);
        }
        
        if let Some(red_vtr) = red_adj_list.get(&node) {
        for nei in red_vtr {
            if !visited.contains(&(*nei, true)) && in_color == false {
                queue.push_back((*nei, dist+ 1, true));
                }
            }
        };

        if let Some(blue_vtr) = blue_adj_list.get(&node) {
        for nei in blue_vtr {
            if !visited.contains(&(*nei, false)) && in_color == true {
                queue.push_back((*nei, dist+1, false));
                }
            }
        };


    }
    res

}


