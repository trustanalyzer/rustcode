use std::collections::HashMap;
use std::cmp::max;

fn main() {
    let trees = vec![1,1,2,1,3];
    let k = 2usize;

    let res = max_trees(&trees, k);
    println!("{res:?}");
}

fn max_trees(trees: &Vec<i32>, k: usize) -> usize {

    let mut l = 0usize;
    let mut max_window = 0usize;
    let mut fruit_tree_ct = HashMap::new();
    for r in 0..trees.len() {
        
        fruit_tree_ct.entry(trees[r])
        .and_modify(|ct| *ct+=1)
        .or_insert(1usize);

        while fruit_tree_ct.len() > k {

            fruit_tree_ct.entry(trees[l])
            .and_modify(|ct| *ct -= 1);
            
            if let Some(&0) = fruit_tree_ct.get(&trees[l]) {
                fruit_tree_ct.remove(&trees[l]);
            }

            l += 1;
        }

        max_window = max(max_window, r-l+1);
    }

    max_window
}