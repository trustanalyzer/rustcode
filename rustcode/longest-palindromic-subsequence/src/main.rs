use std::cmp::max;
use std::collections::HashMap;

fn main() {
    let mut cache: HashMap<(i32,i32), i32> = HashMap::new();
    let s = String::from("bbbab");
    for i in 0..s.len() as i32 {
        dfs(i, i, &s, &mut cache);
        dfs(i, i+1, &s, &mut cache);
    }
    dbg!(cache);

    tabulation(&s);

}

fn dfs(i: i32, j: i32, s: &String,
    cache: &mut HashMap<(i32, i32), i32>) -> i32 {

        if i < 0 || j == s.len() as i32 {return 0;}
        if cache.contains_key(&(i,j)) {return *cache.get(&(i,j)).unwrap();}

        if s.chars().nth(i as usize).unwrap() == s.chars().nth(j as usize).unwrap() {
            let temp = dfs(i-1, j+1, s, cache);
            let length = match i {
                j => 1,
                _ => 2,
            };
            cache.insert((i,j), length + temp);
        } else {
            let temp_1 = dfs(i-1, j, s, cache);
            let temp_2 = dfs(i, j+1, s, cache);
            cache.insert((i,j), max(temp_1, temp_2));
        }

        *cache.get(&(i,j)).unwrap()
    }

    fn tabulation(s: &String) -> usize {
        if s.len() == 0 {return 0;}

        let mut table = Vec::new();
        for i in 0..s.len() + 1 {
            table.push(Vec::new());
            for j in 0..s.len() + 1 {
                table[i].push(0usize);
            }
        }

        for i in 1..table.len() {
            for j in 1..table.len() - i {
                if j==i {table[i][j] = 1;}
                else if j==i+1 {table[i][j] = 2;}
                }
            }
            dbg!(table);
            5
        }   
