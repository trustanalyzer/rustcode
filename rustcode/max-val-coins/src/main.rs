use std::collections::HashMap;
use std::cmp::min;

fn main() {
    let piles = vec![[1,100,3].to_vec(), [7,8,9].to_vec()];
    let k = 2;
    let mut cache = HashMap::new();

    let max_value = dfs(0, k, &piles, &mut cache);
    dbg!(max_value);
    //dbg!(cache);

}

fn dfs(i: i32, k: i32, 
    piles: &Vec<Vec<i32>>, 
    cache: &mut HashMap<(i32, i32), i32>) -> i32 {

    if i == piles.len() as i32 {return 0;}
    if cache.contains_key(&(i,k)) {return *cache.get(&(i,k)).unwrap();}

    let without = dfs(i+1, k, piles, cache);
    cache.insert((i,k), without);

    let mut curr_pile = 0;
    let curr_vec = piles.iter().nth(i as usize).unwrap();
    for j in 0..min(k, curr_vec.len() as i32) {
        curr_pile += curr_vec.iter().nth(j as usize).unwrap_or(&0);
        let switch_pile = curr_pile + dfs(i+1, k-j-1, piles, cache);
        
        // dbg!(i, k);
        if *cache.get(&(i,k)).unwrap() < switch_pile {
            cache.entry((i,k))
            .and_modify(|v| *v = switch_pile);
        }
    }

    *cache.get(&(i,k)).unwrap()
}


