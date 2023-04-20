
use std::collections::VecDeque;
use std::cmp::max;

fn bfs(list: &Vec<Option<i32>>) -> usize {
    if list.len() == 0 {return 0;}
    
    let mut queue = VecDeque::new();
    queue.push_back(0);

    let mut res = 1;
    let mut first_index = 0;
    let mut curr_level = 0;

    while let Some(index) = queue.pop_front() {
        //dbg!(&queue);
        if index / 2 + 1 > curr_level { 
            curr_level = index / 2 + 1;
            first_index = index;
        }
        //dbg!(index, first_index);
        res = max(res, index - first_index + 1);
        
        // dbg!(list.iter().nth(index*2 + 1).unwrap());

        if index*2 + 1 < list.len() {
            match list.iter().nth(index*2 + 1) {
                Some(Some(_)) => queue.push_back(index*2 + 1),
                _ => {}, 
            }
        }

        if index*2 + 2 < list.len() {
            match list.iter().nth(index*2 + 2) {
                Some(Some(_)) => queue.push_back(index*2 + 2),
                _ => {},
            }
        }
    }
    res
}



fn main() {
    let root = vec![1,3,2,5,3,0,9];
    let option_list: Vec<Option<i32>> = root.iter()
    .map(|e| {match *e {
        0 => None,
        _ => Some(*e),
    }}).collect();

    // dbg!(option_list);

    let res = bfs(&option_list);
    dbg!(res);
}


