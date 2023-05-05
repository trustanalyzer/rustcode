use std::collections::HashMap;

fn main() {
    let n = 8;
    let minProfit = 11;
    let group = [2,3,5,5].to_vec();
    let profit = [6,7,8,3].to_vec();
    let mut cache = HashMap::new();

    let memo = dfs(0, n, 0, &mut cache, minProfit, &group, &profit);
    dbg!(memo);

    let num = profit.iter().sum::<i32>();
    dbg!(num);

    dbg!(number_ways(&group, &profit));
}

// return total number of ways based on constraints
fn number_ways(group: &Vec<i32>, profit: &Vec<i32>) -> i32 {
    
    let mut table = [[[0;25];9];2];
    // dbg!(table[0]);
    
    for i in 1..group.len(){
        for j in 0..8usize {
            for k in 0..24usize {

                // dbg!(i,j,k);
                table[i%2][j][k] = table[(i-1)%2][j][k];
                if j + group[(i-1)] as usize <= 8 {
                    // dbg!(k, profit[(i-1)] as usize);
                    table[i%2][j + group[(i-1)] as usize][k + profit[(i-1)] as usize] = table[(i-1)%2][j][k] + 1;
                }
            }
        }
    }

    let mut sum_overall = 0;
    for j in 0..=8usize {
        for k in 11..=24usize {
            dbg!(j,k);
            sum_overall += table[(group.len()-1)%2 as usize][j][k];
        }
    }
    sum_overall
}

fn dfs(i: i32, n: i32, p: i32, 
    cache: &mut HashMap<(i32,i32,i32), i32>,
    minProfit: i32,
    group: &Vec<i32>,
    profit: &Vec<i32>) -> i32 {

        if i == group.len() as i32 {
            if p < minProfit {return 0;}
            else {return 1;}
        }

        if let Some(val) = cache.get(&(i,n,p)) {
            return *val;
        }

        let without_element = dfs(i+1, n, p, cache, minProfit, group, profit);
        cache.insert((i,n,p), without_element);
        if group[i as usize] <= n {
            let with_element = dfs(i+1, n - group[i as usize], p + profit[i as usize], cache, minProfit, group, profit);
            cache
            .entry((i,n,p))
            .and_modify(|val| *val += with_element);
        }

        *cache.get(&(i,n,p)).unwrap()
}
