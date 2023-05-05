use std::collections::HashMap;

fn main() {
    let n = 5;

    let mut cache = HashMap::new();

    let res = dfs(n, &mut cache);
    dbg!(res);

    let res_table = tabulation(n);
    dbg!(res_table);


}

fn dfs(i: i32, cache: &mut HashMap<i32,i32>) -> i32 {

    if i <= 1 {return 1;}
    if cache.contains_key(&i) {return *cache.get(&i).unwrap();}

    let mut count_trees = 0;
    for j in 1..=i {
        let left = j - 1;
        let right = i - j;

        count_trees += dfs(left, cache) * dfs(right, cache);
    }

    cache.insert(i, count_trees);
    *cache.get(&i).unwrap()
}

fn tabulation(n: i32) -> i32 {
    
    let mut table = Vec::new();
    for i in 0..=n {
        if i <= 1 {
            table.push(1);
        } else {

            let mut ct = 0;
            for j in 1..=i {
                let left = j-1;
                let right = i-j;
                ct += table[left as usize] * table[right as usize];
            }
            table.push(ct);
        }
    }
    // dbg!(&table);
    table[n as usize]
}