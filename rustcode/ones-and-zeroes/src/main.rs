use std::cmp::max;
use std::collections::HashMap;

fn main() {
    let strs: Vec<String> = vec!["10".into(), "0001".into(), 
    String::from("110011"), "1".to_string(), "0".to_owned()];

    let (m, n) = (5,3);
    let mut cache: HashMap<(i32,i32,i32),i32> = HashMap::new();

    let res = dfs(&strs, m, n, 0, &mut cache);
    dbg!(res);
}

fn dfs(strs: &Vec<String>, m: i32, n: i32, i: i32, 
    cache: &mut HashMap<(i32,i32,i32),i32>) -> i32 {
    if i == strs.len() as i32 {return 0;}

    if cache.contains_key(&(m,n,i)) {return *cache.get(&(m,n,i)).unwrap();}

    // dbg!(m,n,i);

    let str = &strs[i as usize];
    let mut zero_ct = 0i32;
    for c in str.chars() {
        if c == '0' {zero_ct += 1;}
    }
    let one_ct = str.len() as i32 - zero_ct;

    let mut res = dfs(strs, m, n, i+1, cache);
    if zero_ct <= m && one_ct <= n {
        res = max(res, 1+dfs(strs, m-zero_ct, n-one_ct, i+1, cache));
    }
    cache.insert((m,n,i), res);
    return res
}


