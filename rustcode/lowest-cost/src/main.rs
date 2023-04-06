use std::cmp::min;

fn main() {
    let cost = vec![1,100,1,1,1,100,1,1,100,1];
    let lowest = lowest_cost(&cost);
    dbg!(lowest);
}

fn lowest_cost(cost: &Vec<i32>) -> i32 {

    let n = cost.len();
    let mut curr = cost[n-2];
    let mut prev = cost[n-1];

    for i in (0..cost.len()-3).rev() {
        let temp = curr;
        curr = cost[i] + min(curr, prev);
        prev = temp;
    }

    min(curr,prev)
}
