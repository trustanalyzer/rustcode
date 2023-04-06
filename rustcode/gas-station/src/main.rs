fn main() {
    let gas = vec![1,2,3,4,5];
    let cost = vec![3,4,5,1,2];
    let res = is_possible(&gas, &cost);
    dbg!(res);
}

fn is_possible(gas: &Vec<i32>, cost: &Vec<i32>) -> i32 {
    if gas.iter().sum::<i32>() < cost.iter().sum() {
        return -1;
    }

    let mut curr_gas = 0i32;
    let mut start = 0;
    for i in 0..gas.len() {
        curr_gas += gas[i] - cost[i];

        if curr_gas < 0 {
            curr_gas = 0;
            start = i+1;
        }
    }
    start as i32
}