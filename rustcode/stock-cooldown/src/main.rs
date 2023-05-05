use std::{collections::HashMap, cmp::max};

fn main() {
    let prices = [1,2,3,0,2].to_vec();
    let mut cache = HashMap::new();
    let res = dfs(0, true, &prices, &mut cache);
    println!("result = {res}");
}

fn dfs(i: usize, buying: bool, 
    prices: &Vec<i32>, 
    cache: &mut HashMap<(usize, bool), i32>) -> i32 {

        //dbg!(i,buying);
        if i >= prices.len() {return 0;}
        if let Some(&val) = cache.get(&(i, buying)) {return val;}

        let cooldown = dfs(i+1, buying, prices, cache);
        if buying {
            let buy = dfs(i+1, !buying, prices, cache) - prices[i];
            cache.insert((i,buying), max(buy, cooldown));
        } else {
            let sell = dfs(i+2, !buying, prices, cache) + prices[i];
            cache.insert((i,buying), max(sell, cooldown));
        }

        //dbg!(i, buying, &cache);

        *cache.get(&(i,buying)).unwrap()
}
