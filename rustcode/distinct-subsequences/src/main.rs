use std::collections::HashMap;

fn main() {
    let mut cache = HashMap::new();
    let s = String::from("rabbbit");
    let t = String::from("rabbit");

    let res = dfs(0, 0, &s, &t, &mut cache);
    println!("result = {res:?}");
    //dbg!(cache);
}

fn dfs(i: usize, j: usize, s: &String, t: &String,
    cache: &mut HashMap<(usize, usize), u32>) -> u32 {

        if j == t.len() {return 1;}

        if i == s.len() {return 0;}

        if let None = cache.get(&(i,j)) {
            
            let mut val = dfs(i+1usize, j, s, t, cache);
            if s.chars().nth(i) == t.chars().nth(j) {
                val += dfs(i+1usize, j+1usize, s, t, cache);
            }

            cache.insert((i,j), val);
        }

        *cache.get(&(i, j)).unwrap()

    }
