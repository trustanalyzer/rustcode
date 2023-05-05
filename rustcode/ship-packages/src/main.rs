use std::cmp::min;

fn can_ship(m: u32, days: u32, weights: &Vec<u32>) -> bool {
    let mut curr_sum = 0;

    let mut days_required = 0;
    for wt in weights {
        if curr_sum + wt > m {
             days_required += 1;
             curr_sum = 0;
        }

        curr_sum += wt;
    }

    days_required < days

}

fn ship_within_days(weights: &Vec<u32>, days: u32) -> u32 {
    let mut l = *weights.iter().max().unwrap_or(&0);
    let mut r = weights.iter().sum();

    let mut res = r;
    while l <= r {
        let m = (l + r) / 2;
        if can_ship(m, days, weights) {
            res = min(res, m);
            r = m - 1;
        } else {
            l = m + 1;
        }
    }
    res
}

fn main() {
    let weights: Vec<u32> = (1..=10).collect();

    let res = ship_within_days(&weights, 5);
    dbg!(res);
}
