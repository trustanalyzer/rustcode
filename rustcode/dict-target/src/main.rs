use std::collections::HashMap;

fn main() {
    // let modulus = 10_i32.pow(9) + 7;
    let words = ["acca", "bbbb", "caca"].to_vec();
    let target = String::from("aba");

    let counts = pre_compute(&words);
    // dbg!(&counts);

    let mut cache = HashMap::new();
    let res = dfs(0,0, &target, &words, &counts, &mut cache);
    // dbg!(&cache);
    dbg!(res);
}

fn pre_compute(words: &Vec<&str>) -> HashMap<(u32,char), u32> {
    let mut counter = HashMap::new();
    for word in words {
        for (i,c) in word.chars().enumerate() {
            counter.entry((i.try_into().unwrap(), c))
            .and_modify(|cnt| *cnt += 1)
            .or_insert(1);
        }
    }

    counter
}

fn dfs(i: u32, 
    k: u32, 
    target: &String, 
    words: &Vec<&str>,
    counts: &HashMap<(u32, char), u32>,
    cache: &mut HashMap<(u32, u32), u32>,
) -> u32 {
    if i == target.len().try_into().unwrap() {return 1;}
    if k == words[0].len().try_into().unwrap() {return 0;}
    if cache.contains_key(&(i,k)) {return *cache.get(&(i,k)).unwrap();}

    let without = dfs(i ,k+1, target, words, counts, cache);
    // dbg!(without);

    let c = target.chars().nth(i.try_into().unwrap()).unwrap();
    let count = *counts.get(&(k,c)).unwrap_or(&0); 
    // dbg!(count, k, c);
    let with = count * dfs(i+1, k+1, target, words, counts, cache);
    // dbg!(i,k,with);

    cache.entry((i,k))
    .and_modify(|cnt| *cnt += with)
    .or_insert(without);

    *cache.get(&(i,k)).unwrap()
}


